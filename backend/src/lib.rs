use std::{
    borrow::Cow,
    collections::HashMap,
    env,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        HeaderMap, HeaderValue, Method, StatusCode,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use pbkdf2::pbkdf2_hmac_array;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    RwLock,
};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{error, warn};

pub mod aws_lambda;
pub mod runtime_contract;
pub mod telemetry;

pub const MAX_FRAME_BYTES: usize = 4_096;
pub const MAX_TEXT_CHARS: usize = 1_000;
pub const MAX_SENDER_CHARS: usize = 48;
pub const MAX_USERNAME_CHARS: usize = 24;
pub const MIN_PASSWORD_CHARS: usize = 8;
pub const DEFAULT_SESSION_TTL_SECONDS: i64 = 3_600;
pub const DEFAULT_ALLOWED_ORIGINS: [&str; 12] = [
    "http://localhost:5173",
    "http://127.0.0.1:5173",
    "http://localhost:5174",
    "http://127.0.0.1:5174",
    "http://localhost:5175",
    "http://127.0.0.1:5175",
    "http://localhost:5176",
    "http://127.0.0.1:5176",
    "http://localhost:5177",
    "http://127.0.0.1:5177",
    "http://localhost:5178",
    "http://127.0.0.1:5178",
];
const PASSWORD_HASH_BYTES: usize = 32;
const PASSWORD_ITERATIONS: u32 = 100_000;
const DEFAULT_HISTORY_PAGE_SIZE: usize = 25;
const MAX_HISTORY_PAGE_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct Settings {
    pub session_ttl_seconds: i64,
    pub allowed_origins: Arc<Vec<String>>,
    pub allowed_origin_headers: Arc<Vec<HeaderValue>>,
}

impl Settings {
    pub fn from_env() -> Result<Self, String> {
        let session_ttl_seconds = runtime_contract::SessionPolicy::from_env()?.ttl_seconds();

        let origins_raw =
            env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| DEFAULT_ALLOWED_ORIGINS.join(","));
        let allowed_origins: Vec<String> = origins_raw
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(ToOwned::to_owned)
            .collect();

        if allowed_origins.is_empty() {
            return Err("ALLOWED_ORIGINS must contain at least one origin.".to_string());
        }

        let allowed_origin_headers = allowed_origins
            .iter()
            .map(|origin| {
                HeaderValue::from_str(origin)
                    .map_err(|_| format!("Invalid origin configured: {origin}"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            session_ttl_seconds,
            allowed_origins: Arc::new(allowed_origins),
            allowed_origin_headers: Arc::new(allowed_origin_headers),
        })
    }
}

#[derive(Clone)]
pub struct AppState {
    settings: Arc<Settings>,
    users: Arc<RwLock<UserStore>>,
    sessions: Arc<RwLock<SessionStore>>,
    messages: Arc<RwLock<MessageStore>>,
    connections: Arc<RwLock<HashMap<u64, UnboundedSender<String>>>>,
    next_connection_id: Arc<AtomicU64>,
    telemetry: Arc<telemetry::ServiceTelemetry>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings: Arc::new(settings.clone()),
            users: Arc::new(RwLock::new(UserStore::default())),
            sessions: Arc::new(RwLock::new(SessionStore::new(settings.session_ttl_seconds))),
            messages: Arc::new(RwLock::new(MessageStore::default())),
            connections: Arc::new(RwLock::new(HashMap::new())),
            next_connection_id: Arc::new(AtomicU64::new(1)),
            telemetry: telemetry::shared_telemetry(),
        }
    }

    async fn broadcast_json<T: Serialize>(&self, payload: &T) {
        let encoded = match serde_json::to_string(payload) {
            Ok(encoded) => encoded,
            Err(err) => {
                error!(error = %err, "failed to serialize broadcast payload");
                self.telemetry.record_runtime_error();
                return;
            }
        };

        let mut connections = self.connections.write().await;
        let targets = connections.len();
        let mut delivered = 0usize;
        let mut failed = 0usize;
        connections.retain(|_, sender| {
            let ok = sender.send(encoded.clone()).is_ok();
            if ok {
                delivered += 1;
            } else {
                failed += 1;
            }
            ok
        });
        self.telemetry.record_broadcast(targets, delivered, failed);
        tracing::info!(
            event = "broadcast",
            targets,
            delivered,
            failed,
            outcome = if failed == 0 { "success" } else { "partial_failure" },
            "broadcast completed"
        );
    }

    async fn add_connection(&self, sender: UnboundedSender<String>) -> u64 {
        let connection_id = self.next_connection_id.fetch_add(1, Ordering::Relaxed);
        self.connections.write().await.insert(connection_id, sender);
        self.telemetry.record_websocket_connect(true);
        connection_id
    }

    async fn remove_connection(&self, connection_id: u64) {
        self.connections.write().await.remove(&connection_id);
        self.telemetry.record_websocket_disconnect();
    }

    async fn store_message(&self, message: ChatEvent) {
        self.messages.write().await.push(message);
    }

    async fn list_messages(&self, before: Option<&str>, limit: usize) -> MessageHistoryPage {
        self.messages.read().await.list(before, limit)
    }

    fn is_allowed_origin(&self, origin: Option<&str>) -> bool {
        match origin {
            Some(origin) => self
                .settings
                .allowed_origins
                .iter()
                .any(|allowed| allowed == origin),
            None => true,
        }
    }
}

#[derive(Default)]
struct UserStore {
    users: HashMap<String, UserRecord>,
}

impl UserStore {
    fn register(
        &mut self,
        username: &str,
        password: &str,
        display_name: &str,
    ) -> Result<UserRecord, String> {
        let username = normalize_username(username)?;
        let display_name = normalize_display_name(display_name)?;
        let password = validate_password(password)?;

        if self.users.contains_key(&username) {
            return Err("Username is already registered.".to_string());
        }

        let mut salt = [0_u8; 16];
        OsRng.fill_bytes(&mut salt);
        let password_hash = hash_password(&password, &salt);

        let user = UserRecord {
            username: username.clone(),
            display_name,
            password_salt: salt,
            password_hash,
        };
        self.users.insert(username, user.clone());
        Ok(user)
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<UserRecord, String> {
        let username = normalize_username(username)?;
        let password = validate_password(password)?;
        let user = self
            .users
            .get(&username)
            .cloned()
            .ok_or_else(|| "Invalid username or password.".to_string())?;

        let supplied_hash = hash_password(&password, &user.password_salt);
        if bool::from(user.password_hash.ct_eq(&supplied_hash)) {
            Ok(user)
        } else {
            Err("Invalid username or password.".to_string())
        }
    }
}

struct SessionStore {
    sessions: HashMap<String, SessionIdentity>,
    session_ttl_seconds: i64,
}

#[derive(Default)]
struct MessageStore {
    messages: Vec<ChatEvent>,
}

impl MessageStore {
    fn push(&mut self, message: ChatEvent) {
        self.messages.push(message);
    }

    fn list(&self, before: Option<&str>, limit: usize) -> MessageHistoryPage {
        let page_size = limit.clamp(1, MAX_HISTORY_PAGE_SIZE);
        let boundary = before.and_then(|cursor| {
            self.messages
                .iter()
                .position(|message| message.id.as_deref() == Some(cursor))
        });
        let upper_bound = boundary.unwrap_or(self.messages.len());
        let start = upper_bound.saturating_sub(page_size);
        let items = self.messages[start..upper_bound].to_vec();
        let has_more = start > 0;
        let next_before = if has_more {
            items.first().and_then(|message| message.id.clone())
        } else {
            None
        };

        MessageHistoryPage {
            messages: items,
            has_more,
            next_before,
        }
    }
}

impl SessionStore {
    fn new(session_ttl_seconds: i64) -> Self {
        Self {
            sessions: HashMap::new(),
            session_ttl_seconds,
        }
    }

    fn create(&mut self, user: &UserRecord) -> SessionIdentity {
        let mut raw_token = [0_u8; 32];
        OsRng.fill_bytes(&mut raw_token);
        let token = URL_SAFE_NO_PAD.encode(raw_token);
        let identity = SessionIdentity {
            token: token.clone(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            expires_at: runtime_contract::SessionPolicy::from_ttl_seconds(self.session_ttl_seconds)
                .expect("session ttl should be validated at startup")
                .expires_at(),
        };
        self.sessions.insert(token, identity.clone());
        identity
    }

    fn get(&mut self, token: &str) -> Option<SessionIdentity> {
        let identity = self.sessions.get(token).cloned()?;
        if identity.expires_at <= Utc::now() {
            self.sessions.remove(token);
            None
        } else {
            Some(identity)
        }
    }

    fn revoke(&mut self, token: &str) {
        self.sessions.remove(token);
    }
}

#[derive(Clone)]
struct UserRecord {
    username: String,
    display_name: String,
    password_salt: [u8; 16],
    password_hash: [u8; PASSWORD_HASH_BYTES],
}

#[derive(Clone)]
struct SessionIdentity {
    token: String,
    username: String,
    display_name: String,
    expires_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterRequest {
    username: String,
    password: String,
    display_name: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct ChatQuery {
    token: Option<String>,
}

#[derive(Deserialize)]
struct HistoryQuery {
    before: Option<String>,
    limit: Option<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionResponse {
    token: String,
    username: String,
    display_name: String,
    expires_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageHistoryResponse {
    messages: Vec<ChatEvent>,
    has_more: bool,
    next_before: Option<String>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    telemetry: telemetry::TelemetrySnapshot,
}

#[derive(Serialize)]
struct ErrorDetail<'a> {
    detail: &'a str,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChatEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "type")]
    event_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender: Option<String>,
    text: String,
    sent_at: String,
}

struct MessageHistoryPage {
    messages: Vec<ChatEvent>,
    has_more: bool,
    next_before: Option<String>,
}

struct AppError {
    status: StatusCode,
    detail: String,
}

impl AppError {
    fn bad_request(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            detail: detail.into(),
        }
    }

    fn unauthorized(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            detail: detail.into(),
        }
    }

    fn forbidden(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            detail: detail.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorDetail {
                detail: &self.detail,
            }),
        )
            .into_response()
    }
}

pub fn build_app(settings: Settings) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(
            settings.allowed_origin_headers.iter().cloned(),
        ))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    let state = AppState::new(settings);
    Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/messages", get(list_messages))
        .route("/ws/chat", get(chat_socket))
        .with_state(state)
        .layer(cors)
}

pub async fn run(settings: Settings) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_app(settings);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<HealthResponse> {
    let telemetry = telemetry::shared_telemetry().snapshot();
    Json(HealthResponse {
        status: "ok",
        telemetry,
    })
}

async fn register(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<SessionResponse>), AppError> {
    let started_at = std::time::Instant::now();
    ensure_allowed_http_origin(&state, &headers)?;

    let user = {
        let mut users = state.users.write().await;
        users
            .register(&payload.username, &payload.password, &payload.display_name)
            .map_err(AppError::bad_request)?
    };

    let session = {
        let mut sessions = state.sessions.write().await;
        sessions.create(&user)
    };

    state.telemetry.record_auth(true);
    tracing::info!(
        event = "auth_request",
        route = "register",
        outcome = "success",
        username = %user.username,
        duration_ms = started_at.elapsed().as_millis() as u64,
        "auth request completed"
    );

    Ok((StatusCode::CREATED, Json(SessionResponse::from(session))))
}

async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<SessionResponse>, AppError> {
    let started_at = std::time::Instant::now();
    ensure_allowed_http_origin(&state, &headers)?;

    let user = {
        let users = state.users.read().await;
        users
            .authenticate(&payload.username, &payload.password)
            .map_err(AppError::unauthorized)?
    };

    let session = {
        let mut sessions = state.sessions.write().await;
        sessions.create(&user)
    };

    state.telemetry.record_auth(true);
    tracing::info!(
        event = "auth_request",
        route = "login",
        outcome = "success",
        username = %user.username,
        duration_ms = started_at.elapsed().as_millis() as u64,
        "auth request completed"
    );

    Ok(Json(SessionResponse::from(session)))
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Result<StatusCode, AppError> {
    let started_at = std::time::Instant::now();
    ensure_allowed_http_origin(&state, &headers)?;
    let token = extract_bearer_token(&headers)?;
    let mut sessions = state.sessions.write().await;
    sessions.revoke(&token);
    state.telemetry.record_auth(true);
    tracing::info!(
        event = "auth_request",
        route = "logout",
        outcome = "success",
        token_present = !token.is_empty(),
        duration_ms = started_at.elapsed().as_millis() as u64,
        "auth request completed"
    );
    Ok(StatusCode::NO_CONTENT)
}

async fn list_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<MessageHistoryResponse>, AppError> {
    ensure_allowed_http_origin(&state, &headers)?;
    let token = extract_bearer_token(&headers)?;

    let identity = {
        let mut sessions = state.sessions.write().await;
        sessions.get(&token)
    };
    if identity.is_none() {
        state.telemetry.record_auth(false);
        return Err(AppError::unauthorized(
            "Authentication required or session expired.",
        ));
    }

    let page = state
        .list_messages(query.before.as_deref(), query.limit.unwrap_or(DEFAULT_HISTORY_PAGE_SIZE))
        .await;

    Ok(Json(MessageHistoryResponse {
        messages: page.messages,
        has_more: page.has_more,
        next_before: page.next_before,
    }))
}

async fn chat_socket(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ChatQuery>,
) -> impl IntoResponse {
    let origin = header_value(&headers, ORIGIN);
    let token = query.token.unwrap_or_default();
    ws.on_upgrade(move |socket| websocket_session(socket, state, origin, token))
}

async fn websocket_session(
    socket: WebSocket,
    state: AppState,
    origin: Option<String>,
    token: String,
) {
    if !state.is_allowed_origin(origin.as_deref()) {
        state.telemetry.record_websocket_connect(false);
        tracing::warn!(
            event = "websocket_connect",
            outcome = "rejected_origin",
            origin = origin.as_deref().unwrap_or("<missing>"),
            "websocket connect rejected"
        );
        let mut socket = socket;
        close_socket(&mut socket, "Origin not allowed.").await;
        return;
    }

    let identity = {
        let mut sessions = state.sessions.write().await;
        sessions.get(token.trim())
    };

    let Some(identity) = identity else {
        state.telemetry.record_websocket_connect(false);
        tracing::warn!(
            event = "websocket_connect",
            outcome = "rejected_session",
            origin = origin.as_deref().unwrap_or("<missing>"),
            "websocket connect rejected"
        );
        let mut socket = socket;
        close_socket(&mut socket, "Authentication required or session expired.").await;
        return;
    };

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (connection_tx, mut connection_rx) = unbounded_channel::<String>();
    let connection_id = state.add_connection(connection_tx.clone()).await;
    tracing::info!(
        event = "websocket_connect",
        outcome = "accepted",
        connection_id,
        username = %identity.username,
        "websocket connect accepted"
    );

    let writer = tokio::spawn(async move {
        while let Some(payload) = connection_rx.recv().await {
            if ws_sender.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    state
        .broadcast_json(&ChatEvent::system(format!(
            "{} joined the chat",
            identity.display_name
        )))
        .await;

    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(Message::Text(raw)) => {
                let still_valid = {
                    let mut sessions = state.sessions.write().await;
                    sessions.get(&identity.token)
                };
                if still_valid.is_none() {
                    state.telemetry.record_runtime_error();
                    tracing::warn!(
                        event = "websocket_message",
                        outcome = "expired_session",
                        connection_id,
                        username = %identity.username,
                        "websocket message rejected because session expired"
                    );
                    let _ = connection_tx.send(
                        serde_json::to_string(&ChatEvent::error(
                            "Authentication required or session expired.",
                        ))
                        .unwrap_or_default(),
                    );
                    break;
                }

                match runtime_contract::parse_and_validate_chat_text(&raw) {
                    Ok(Some(text)) => {
                        let text_len = text.chars().count();
                        state.telemetry.record_message_accepted();
                        tracing::info!(
                            event = "websocket_message",
                            outcome = "accepted",
                            connection_id,
                            username = %identity.username,
                            text_len,
                            "websocket message accepted"
                        );
                        let message = ChatEvent::message(identity.display_name.clone(), text);
                        state.store_message(message.clone()).await;
                        state
                            .broadcast_json(&message)
                            .await;
                    }
                    Ok(None) => {}
                    Err(detail) => {
                        state.telemetry.record_message_rejected();
                        warn!(detail = %detail, "rejected websocket payload");
                        tracing::warn!(
                            event = "websocket_message",
                            outcome = "rejected_validation",
                            connection_id,
                            username = %identity.username,
                            detail = %detail,
                            "websocket message rejected"
                        );
                        let _ = connection_tx.send(
                            serde_json::to_string(&ChatEvent::error(detail)).unwrap_or_default(),
                        );
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => {}
            Err(err) => {
                state.telemetry.record_runtime_error();
                error!(error = %err, "unexpected websocket error");
                tracing::error!(
                    event = "websocket_message",
                    outcome = "runtime_error",
                    connection_id,
                    username = %identity.username,
                    error = %err,
                    "websocket session failed"
                );
                break;
            }
        }
    }

    state.remove_connection(connection_id).await;
    tracing::info!(
        event = "websocket_disconnect",
        connection_id,
        username = %identity.username,
        "websocket disconnected"
    );
    state
        .broadcast_json(&ChatEvent::system(format!(
            "{} left the chat",
            identity.display_name
        )))
        .await;
    writer.abort();
}

fn ensure_allowed_http_origin(state: &AppState, headers: &HeaderMap) -> Result<(), AppError> {
    let origin = header_value(headers, ORIGIN);
    if state.is_allowed_origin(origin.as_deref()) {
        Ok(())
    } else {
        state.telemetry.record_auth(false);
        tracing::warn!(
            event = "auth_request",
            route = "origin_check",
            outcome = "forbidden_origin",
            origin = origin.as_deref().unwrap_or("<missing>"),
            "http auth request rejected"
        );
        Err(AppError::forbidden("Origin not allowed."))
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Result<String, AppError> {
    let value = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("Authorization header is required."))?
        .to_str()
        .map_err(|_| AppError::unauthorized("Authorization must use a bearer token."))?;

    let (scheme, token) = value
        .split_once(' ')
        .ok_or_else(|| AppError::unauthorized("Authorization must use a bearer token."))?;

    if !scheme.eq_ignore_ascii_case("bearer") || token.trim().is_empty() {
        return Err(AppError::unauthorized(
            "Authorization must use a bearer token.",
        ));
    }

    Ok(token.trim().to_string())
}

fn normalize_username(raw: &str) -> Result<String, String> {
    let username = raw.trim().to_lowercase();
    if username.is_empty() {
        return Err("Username is required.".to_string());
    }
    if username.chars().count() > MAX_USERNAME_CHARS {
        return Err(format!(
            "Username must be at most {MAX_USERNAME_CHARS} characters."
        ));
    }
    if !username
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
    {
        return Err("Username may only contain letters, numbers, '-' and '_'.".to_string());
    }

    Ok(username)
}

fn normalize_display_name(raw: &str) -> Result<String, String> {
    let display_name = raw.trim().to_string();
    if display_name.is_empty() {
        return Err("Display name is required.".to_string());
    }
    if display_name.chars().count() > MAX_SENDER_CHARS {
        return Err(format!(
            "Display name must be at most {MAX_SENDER_CHARS} characters."
        ));
    }

    Ok(display_name)
}

fn validate_password(raw: &str) -> Result<String, String> {
    if raw.chars().count() < MIN_PASSWORD_CHARS {
        return Err(format!(
            "Password must be at least {MIN_PASSWORD_CHARS} characters."
        ));
    }
    Ok(raw.to_string())
}

fn hash_password(password: &str, salt: &[u8; 16]) -> [u8; PASSWORD_HASH_BYTES] {
    pbkdf2_hmac_array::<Sha256, PASSWORD_HASH_BYTES>(password.as_bytes(), salt, PASSWORD_ITERATIONS)
}

fn header_value(headers: &HeaderMap, name: axum::http::header::HeaderName) -> Option<String> {
    headers.get(name)?.to_str().ok().map(ToOwned::to_owned)
}

async fn close_socket(socket: &mut WebSocket, reason: &str) {
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: axum::extract::ws::close_code::POLICY,
            reason: Cow::Owned(reason.to_string()),
        })))
        .await;
}

impl SessionResponse {
    fn from(identity: SessionIdentity) -> Self {
        Self {
            token: identity.token,
            username: identity.username,
            display_name: identity.display_name,
            expires_at: identity.expires_at.to_rfc3339(),
        }
    }
}

impl ChatEvent {
    fn system(text: String) -> Self {
        Self {
            id: None,
            event_type: "system",
            sender: None,
            text,
            sent_at: Utc::now().to_rfc3339(),
        }
    }

    fn message(sender: String, text: String) -> Self {
        Self {
            id: Some(generate_message_id()),
            event_type: "message",
            sender: Some(sender),
            text,
            sent_at: Utc::now().to_rfc3339(),
        }
    }

    fn error(text: &'static str) -> Self {
        Self {
            id: None,
            event_type: "error",
            sender: None,
            text: text.to_string(),
            sent_at: Utc::now().to_rfc3339(),
        }
    }
}

fn generate_message_id() -> String {
    let timestamp = Utc::now().timestamp_micros();
    let mut random = [0_u8; 6];
    OsRng.fill_bytes(&mut random);
    format!("{timestamp:020}-{}", URL_SAFE_NO_PAD.encode(random))
}
