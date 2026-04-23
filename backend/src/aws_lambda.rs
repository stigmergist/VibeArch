use aws_config::BehaviorVersion;
use aws_lambda_events::event::apigw::ApiGatewayWebsocketProxyRequest;
use aws_credential_types::Credentials;
use aws_sdk_apigatewaymanagement::{primitives::Blob, Client as ApiGatewayManagementClient};
use aws_sdk_dynamodb::{types::AttributeValue, Client as DynamoDbClient};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Duration, Utc};
use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use pbkdf2::pbkdf2_hmac_array;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use crate::{
    DEFAULT_SESSION_TTL_SECONDS, MAX_FRAME_BYTES, MAX_SENDER_CHARS, MAX_TEXT_CHARS,
    MAX_USERNAME_CHARS, MIN_PASSWORD_CHARS,
};

const PASSWORD_HASH_BYTES: usize = 32;
const PASSWORD_ITERATIONS: u32 = 100_000;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRecord {
    pub username: String,
    pub display_name: String,
    pub password_salt_b64: String,
    pub password_hash_b64: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRecord {
    pub token: String,
    pub username: String,
    pub display_name: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionRecord {
    pub connection_id: String,
    pub username: String,
    pub display_name: String,
    pub connected_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResponse {
    pub token: String,
    pub username: String,
    pub display_name: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEnvelope {
    #[serde(rename = "type")]
    pub event_type: String,
    pub sender: Option<String>,
    pub text: String,
    pub sent_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsocketRouteResponse {
    pub status_code: i64,
    pub body: String,
    pub is_base64_encoded: bool,
}

#[derive(Clone, Debug)]
pub struct DynamoLayout {
    pub users_table: String,
    pub sessions_table: String,
    pub connections_table: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredSessionRecord {
    token: String,
    username: String,
    display_name: String,
    expires_at: String,
    expires_at_epoch: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredUserRecord {
    username: String,
    display_name: String,
    password_salt_b64: String,
    password_hash_b64: String,
    created_at: String,
}

#[derive(Clone, Debug)]
struct AwsClients {
    layout: DynamoLayout,
    dynamodb: DynamoDbClient,
    management: Option<ApiGatewayManagementClient>,
}

impl DynamoLayout {
    pub fn from_env() -> Self {
        let sam_local = running_in_sam_local();
        let local_dynamodb = std::env::var_os("DYNAMODB_ENDPOINT").is_some();
        let local_mode = sam_local || local_dynamodb;
        let layout = Self {
            users_table: resolve_table_name(
                "USERS_TABLE",
                "UsersTable",
                "simple-chat-users-local",
                "simple-chat-users",
                local_mode,
            ),
            sessions_table: resolve_table_name(
                "SESSIONS_TABLE",
                "SessionsTable",
                "simple-chat-sessions-local",
                "simple-chat-sessions",
                local_mode,
            ),
            connections_table: resolve_table_name(
                "CONNECTIONS_TABLE",
                "ConnectionsTable",
                "simple-chat-connections-local",
                "simple-chat-connections",
                local_mode,
            ),
        };

        layout
    }
}

impl AwsClients {
    async fn from_http_env() -> Result<Self, Error> {
        let shared = load_shared_config().await;
        Ok(Self {
            layout: DynamoLayout::from_env(),
            dynamodb: build_dynamodb_client(&shared),
            management: None,
        })
    }

    async fn from_ws_request(request: &ApiGatewayWebsocketProxyRequest) -> Result<Self, Error> {
        let shared = load_shared_config().await;
        let management = resolve_management_client(&shared, request);
        Ok(Self {
            layout: DynamoLayout::from_env(),
            dynamodb: build_dynamodb_client(&shared),
            management,
        })
    }

    async fn get_user(&self, username: &str) -> Result<Option<UserRecord>, Error> {
        let output = self
            .dynamodb
            .get_item()
            .table_name(&self.layout.users_table)
            .key("username", AttributeValue::S(username.to_string()))
            .send()
            .await
            .map_err(boxed)?;

        let item = match output.item {
            Some(item) => item,
            None => return Ok(None),
        };

        let stored: StoredUserRecord = serde_dynamo::from_item(item).map_err(boxed)?;
        Ok(Some(UserRecord {
            username: stored.username,
            display_name: stored.display_name,
            password_salt_b64: stored.password_salt_b64,
            password_hash_b64: stored.password_hash_b64,
        }))
    }

    async fn put_user(&self, user: &UserRecord) -> Result<(), Error> {
        let stored = StoredUserRecord {
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            password_salt_b64: user.password_salt_b64.clone(),
            password_hash_b64: user.password_hash_b64.clone(),
            created_at: Utc::now().to_rfc3339(),
        };

        self.dynamodb
            .put_item()
            .table_name(&self.layout.users_table)
            .set_item(Some(serde_dynamo::to_item(stored).map_err(boxed)?))
            .send()
            .await
            .map_err(boxed)?;
        Ok(())
    }

    async fn get_session(&self, token: &str) -> Result<Option<SessionRecord>, Error> {
        let output = self
            .dynamodb
            .get_item()
            .table_name(&self.layout.sessions_table)
            .key("token", AttributeValue::S(token.to_string()))
            .send()
            .await
            .map_err(boxed)?;

        let item = match output.item {
            Some(item) => item,
            None => return Ok(None),
        };

        let stored: StoredSessionRecord = serde_dynamo::from_item(item).map_err(boxed)?;
        if stored.expires_at_epoch <= Utc::now().timestamp() {
            self.delete_session(&stored.token).await?;
            return Ok(None);
        }

        Ok(Some(SessionRecord {
            token: stored.token,
            username: stored.username,
            display_name: stored.display_name,
            expires_at: stored.expires_at,
        }))
    }

    async fn put_session(&self, session: &SessionRecord) -> Result<(), Error> {
        let stored = StoredSessionRecord::try_from(session.clone()).map_err(boxed)?;
        self.dynamodb
            .put_item()
            .table_name(&self.layout.sessions_table)
            .set_item(Some(serde_dynamo::to_item(stored).map_err(boxed)?))
            .send()
            .await
            .map_err(boxed)?;
        Ok(())
    }

    async fn delete_session(&self, token: &str) -> Result<(), Error> {
        self.dynamodb
            .delete_item()
            .table_name(&self.layout.sessions_table)
            .key("token", AttributeValue::S(token.to_string()))
            .send()
            .await
            .map_err(boxed)?;
        Ok(())
    }

    async fn get_connection(&self, connection_id: &str) -> Result<Option<ConnectionRecord>, Error> {
        let output = self
            .dynamodb
            .get_item()
            .table_name(&self.layout.connections_table)
            .key("connectionId", AttributeValue::S(connection_id.to_string()))
            .send()
            .await
            .map_err(boxed)?;

        let item = match output.item {
            Some(item) => item,
            None => return Ok(None),
        };

        Ok(Some(serde_dynamo::from_item(item).map_err(boxed)?))
    }

    async fn put_connection(&self, connection: &ConnectionRecord) -> Result<(), Error> {
        self.dynamodb
            .put_item()
            .table_name(&self.layout.connections_table)
            .set_item(Some(serde_dynamo::to_item(connection).map_err(boxed)?))
            .send()
            .await
            .map_err(boxed)?;
        Ok(())
    }

    async fn delete_connection(&self, connection_id: &str) -> Result<(), Error> {
        self.dynamodb
            .delete_item()
            .table_name(&self.layout.connections_table)
            .key("connectionId", AttributeValue::S(connection_id.to_string()))
            .send()
            .await
            .map_err(boxed)?;
        Ok(())
    }

    async fn list_connections(&self) -> Result<Vec<ConnectionRecord>, Error> {
        let output = self
            .dynamodb
            .scan()
            .table_name(&self.layout.connections_table)
            .send()
            .await
            .map_err(boxed)?;

        let mut connections = Vec::new();
        for item in output.items.unwrap_or_default() {
            connections.push(serde_dynamo::from_item(item).map_err(boxed)?);
        }
        Ok(connections)
    }

    async fn broadcast_message(&self, message: &ChatEnvelope) -> Result<(), Error> {
        let Some(management) = &self.management else {
            tracing::info!("websocket management endpoint not configured; skipping fan-out");
            return Ok(());
        };

        let payload = serde_json::to_vec(message).map_err(boxed)?;
        for connection in self.list_connections().await? {
            let result = management
                .post_to_connection()
                .connection_id(connection.connection_id.clone())
                .data(Blob::new(payload.clone()))
                .send()
                .await;

            if let Err(error) = result {
                let error_text = error.to_string();
                if error_text.contains("GoneException") || error_text.contains("410") {
                    let _ = self.delete_connection(&connection.connection_id).await;
                }
                tracing::warn!(connection_id = %connection.connection_id, error = %error_text, "failed to post to websocket connection");
            }
        }

        Ok(())
    }
}

pub async fn handle_auth_http(request: Request) -> Result<Response<Body>, Error> {
    let path = request.raw_http_path();
    let method = request.method().as_str();

    match (method, path) {
        ("POST", "/auth/register") => register(request).await,
        ("POST", "/auth/login") => login(request).await,
        ("POST", "/auth/logout") => logout(request).await,
        _ => json_response(StatusCode::NOT_FOUND, json!({ "detail": "Route not found." })),
    }
}

pub async fn handle_ws_connect(
    request: ApiGatewayWebsocketProxyRequest,
) -> Result<WebsocketRouteResponse, Error> {
    let clients = AwsClients::from_ws_request(&request).await?;
    let connection_id = request
        .request_context
        .connection_id
        .unwrap_or_else(|| "unknown".to_string());
    let token = request
        .query_string_parameters
        .first("token")
        .map(str::trim)
        .unwrap_or_default();

    if token.is_empty() {
        return ws_response(401, "Missing token for websocket connection.");
    }

    let Some(session) = clients.get_session(token).await? else {
        return ws_response(401, "Authentication required or session expired.");
    };

    let connection = ConnectionRecord {
        connection_id: connection_id.clone(),
        username: session.username.clone(),
        display_name: session.display_name.clone(),
        connected_at: Utc::now().to_rfc3339(),
    };
    clients.put_connection(&connection).await?;

    clients
        .broadcast_message(&ChatEnvelope::system(format!(
            "{} joined the chat",
            session.display_name
        )))
        .await?;

    ws_response(200, "Connect accepted.")
}

pub async fn handle_ws_disconnect(
    request: ApiGatewayWebsocketProxyRequest,
) -> Result<WebsocketRouteResponse, Error> {
    let clients = AwsClients::from_ws_request(&request).await?;
    let connection_id = request
        .request_context
        .connection_id
        .unwrap_or_else(|| "unknown".to_string());

    let connection = clients.get_connection(&connection_id).await?;
    clients.delete_connection(&connection_id).await?;

    if let Some(connection) = connection {
        clients
            .broadcast_message(&ChatEnvelope::system(format!(
                "{} left the chat",
                connection.display_name
            )))
            .await?;
    }

    ws_response(200, "Disconnect accepted.")
}

pub async fn handle_ws_send_message(
    request: ApiGatewayWebsocketProxyRequest,
) -> Result<WebsocketRouteResponse, Error> {
    let clients = AwsClients::from_ws_request(&request).await?;
    let connection_id = request
        .request_context
        .connection_id
        .unwrap_or_else(|| "unknown".to_string());
    let Some(connection) = clients.get_connection(&connection_id).await? else {
        return ws_response(401, "Connection is not registered.");
    };

    let body = request.body.unwrap_or_default();
    match parse_and_validate(&body) {
        Ok(Some(text)) => {
            clients
                .broadcast_message(&ChatEnvelope::message(connection.display_name, text))
                .await?;
            ws_response(200, "Message accepted.")
        }
        Ok(None) => ws_response(200, "Blank message ignored."),
        Err(detail) => ws_response(400, detail),
    }
}

pub fn normalize_username(raw: &str) -> Result<String, String> {
    let username = raw.trim().to_lowercase();
    if username.is_empty() {
        return Err("Username is required.".to_string());
    }
    if username.chars().count() > MAX_USERNAME_CHARS {
        return Err(format!("Username must be at most {MAX_USERNAME_CHARS} characters."));
    }
    if !username
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
    {
        return Err("Username may only contain letters, numbers, '-' and '_' .".replace(" _ ", "_"));
    }

    Ok(username)
}

pub fn normalize_display_name(raw: &str) -> Result<String, String> {
    let display_name = raw.trim().to_string();
    if display_name.is_empty() {
        return Err("Display name is required.".to_string());
    }
    if display_name.chars().count() > MAX_SENDER_CHARS {
        return Err(format!("Display name must be at most {MAX_SENDER_CHARS} characters."));
    }

    Ok(display_name)
}

pub fn validate_password(raw: &str) -> Result<String, String> {
    if raw.chars().count() < MIN_PASSWORD_CHARS {
        return Err(format!("Password must be at least {MIN_PASSWORD_CHARS} characters."));
    }
    Ok(raw.to_string())
}

pub fn hash_password(password: &str, salt: &[u8; 16]) -> [u8; PASSWORD_HASH_BYTES] {
    pbkdf2_hmac_array::<Sha256, PASSWORD_HASH_BYTES>(password.as_bytes(), salt, PASSWORD_ITERATIONS)
}

pub fn new_user_record(payload: RegisterRequest) -> Result<UserRecord, String> {
    let username = normalize_username(&payload.username)?;
    let display_name = normalize_display_name(&payload.display_name)?;
    let password = validate_password(&payload.password)?;

    let mut salt = [0_u8; 16];
    OsRng.fill_bytes(&mut salt);
    let hash = hash_password(&password, &salt);

    Ok(UserRecord {
        username,
        display_name,
        password_salt_b64: URL_SAFE_NO_PAD.encode(salt),
        password_hash_b64: URL_SAFE_NO_PAD.encode(hash),
    })
}

pub fn authenticate_user(user: &UserRecord, payload: &LoginRequest) -> Result<(), String> {
    let username = normalize_username(&payload.username)?;
    let password = validate_password(&payload.password)?;
    if username != user.username {
        return Err("Invalid username or password.".to_string());
    }

    let salt_bytes = decode_fixed::<16>(&user.password_salt_b64)?;
    let expected_hash = decode_fixed::<PASSWORD_HASH_BYTES>(&user.password_hash_b64)?;
    let supplied_hash = hash_password(&password, &salt_bytes);

    if bool::from(expected_hash.ct_eq(&supplied_hash)) {
        Ok(())
    } else {
        Err("Invalid username or password.".to_string())
    }
}

pub fn new_session_record(user: &UserRecord) -> SessionRecord {
    let mut raw_token = [0_u8; 32];
    OsRng.fill_bytes(&mut raw_token);
    SessionRecord {
        token: URL_SAFE_NO_PAD.encode(raw_token),
        username: user.username.clone(),
        display_name: user.display_name.clone(),
        expires_at: (Utc::now() + Duration::seconds(DEFAULT_SESSION_TTL_SECONDS)).to_rfc3339(),
    }
}

pub fn parse_and_validate(raw: &str) -> Result<Option<String>, &'static str> {
    if raw.as_bytes().len() > MAX_FRAME_BYTES {
        return Err("Frame exceeds maximum allowed size (4096 bytes).");
    }

    let data: Value =
        serde_json::from_str(raw).map_err(|_| "Malformed JSON - message was not delivered.")?;
    let object = data.as_object().ok_or("Payload must be a JSON object.")?;

    if object.contains_key("sender") {
        return Err("Field 'sender' is server-owned and must not be sent by clients.");
    }

    let raw_text = object
        .get("text")
        .and_then(Value::as_str)
        .ok_or("Field 'text' must be a string.")?;
    let text = raw_text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    if text.chars().count() > MAX_TEXT_CHARS {
        return Err("Message text exceeds 1000 characters.");
    }

    Ok(Some(text.to_string()))
}

async fn register(request: Request) -> Result<Response<Body>, Error> {
    let clients = AwsClients::from_http_env().await?;
    let payload: RegisterRequest = match parse_json_body(request.body()) {
        Ok(payload) => payload,
        Err(detail) => return json_response(StatusCode::BAD_REQUEST, json!({ "detail": detail })),
    };
    let user = match new_user_record(payload) {
        Ok(user) => user,
        Err(detail) => return json_response(StatusCode::BAD_REQUEST, json!({ "detail": detail })),
    };

    if clients.get_user(&user.username).await?.is_some() {
        return json_response(StatusCode::BAD_REQUEST, json!({ "detail": "Username is already registered." }));
    }

    clients.put_user(&user).await?;
    let session = new_session_record(&user);
    clients.put_session(&session).await?;

    tracing::info!(
        users_table = %clients.layout.users_table,
        sessions_table = %clients.layout.sessions_table,
        username = %user.username,
        "register handler stored user and session records"
    );

    json_response(StatusCode::CREATED, serde_json::to_value(SessionResponse::from(session))?)
}

async fn login(request: Request) -> Result<Response<Body>, Error> {
    let clients = AwsClients::from_http_env().await?;
    let payload: LoginRequest = match parse_json_body(request.body()) {
        Ok(payload) => payload,
        Err(detail) => return json_response(StatusCode::BAD_REQUEST, json!({ "detail": detail })),
    };
    let username = match normalize_username(&payload.username) {
        Ok(username) => username,
        Err(detail) => return json_response(StatusCode::BAD_REQUEST, json!({ "detail": detail })),
    };

    let Some(user) = clients.get_user(&username).await? else {
        return json_response(StatusCode::UNAUTHORIZED, json!({ "detail": "Invalid username or password." }));
    };
    if authenticate_user(&user, &payload).is_err() {
        return json_response(StatusCode::UNAUTHORIZED, json!({ "detail": "Invalid username or password." }));
    }

    let session = new_session_record(&user);
    clients.put_session(&session).await?;

    tracing::info!(
        users_table = %clients.layout.users_table,
        sessions_table = %clients.layout.sessions_table,
        username = %username,
        "login handler loaded user and stored session"
    );

    json_response(StatusCode::OK, serde_json::to_value(SessionResponse::from(session))?)
}

async fn logout(request: Request) -> Result<Response<Body>, Error> {
    let clients = AwsClients::from_http_env().await?;
    let headers = request.headers();
    let authorization = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    let token = match extract_bearer_token(authorization) {
        Ok(token) => token,
        Err(detail) => return json_response(StatusCode::UNAUTHORIZED, json!({ "detail": detail })),
    };
    clients.delete_session(&token).await?;

    tracing::info!(
        sessions_table = %clients.layout.sessions_table,
        token_present = !token.is_empty(),
        "logout handler revoked session"
    );

    empty_response(StatusCode::NO_CONTENT)
}

fn extract_bearer_token(value: &str) -> Result<String, String> {
    let (scheme, token) = value
        .split_once(' ')
        .ok_or_else(|| "Authorization must use a bearer token.".to_string())?;
    if !scheme.eq_ignore_ascii_case("bearer") || token.trim().is_empty() {
        return Err("Authorization must use a bearer token.".to_string());
    }
    Ok(token.trim().to_string())
}

fn parse_json_body<T: for<'de> Deserialize<'de>>(body: &Body) -> Result<T, String> {
    match body {
        Body::Text(text) => serde_json::from_str(text).map_err(|_| "Request body must be valid JSON.".to_string()),
        Body::Binary(bytes) => serde_json::from_slice(bytes).map_err(|_| "Request body must be valid JSON.".to_string()),
        Body::Empty => Err("Request body is required.".to_string()),
    }
}

fn decode_fixed<const N: usize>(encoded: &str) -> Result<[u8; N], String> {
    let bytes = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| "Stored credential data is invalid.".to_string())?;
    let vec: Vec<u8> = bytes;
    let array: [u8; N] = vec
        .try_into()
        .map_err(|_| "Stored credential data is invalid.".to_string())?;
    Ok(array)
}

fn boxed(message: impl std::fmt::Display + std::fmt::Debug) -> Error {
    tracing::error!(error = %message, debug = ?message, "aws handler operation failed");
    Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        message.to_string(),
    ))
}

fn json_response(status: StatusCode, payload: serde_json::Value) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Body::Text(payload.to_string()))?)
}

fn empty_response(status: StatusCode) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(status).body(Body::Empty)?)
}

fn ws_response(status_code: i64, message: &str) -> Result<WebsocketRouteResponse, Error> {
    Ok(WebsocketRouteResponse {
        status_code,
        body: message.to_string(),
        is_base64_encoded: false,
    })
}

impl From<SessionRecord> for SessionResponse {
    fn from(value: SessionRecord) -> Self {
        Self {
            token: value.token,
            username: value.username,
            display_name: value.display_name,
            expires_at: value.expires_at,
        }
    }
}

impl TryFrom<SessionRecord> for StoredSessionRecord {
    type Error = String;

    fn try_from(value: SessionRecord) -> Result<Self, Self::Error> {
        let expires_at_epoch = DateTime::parse_from_rfc3339(&value.expires_at)
            .map_err(|_| "Session expiry timestamp is invalid.".to_string())?
            .with_timezone(&Utc)
            .timestamp();

        Ok(Self {
            token: value.token,
            username: value.username,
            display_name: value.display_name,
            expires_at: value.expires_at,
            expires_at_epoch,
        })
    }
}

impl ChatEnvelope {
    fn system(text: String) -> Self {
        Self {
            event_type: "system".to_string(),
            sender: None,
            text,
            sent_at: Utc::now().to_rfc3339(),
        }
    }

    fn message(sender: String, text: String) -> Self {
        Self {
            event_type: "message".to_string(),
            sender: Some(sender),
            text,
            sent_at: Utc::now().to_rfc3339(),
        }
    }
}

fn build_dynamodb_client(shared_config: &aws_config::SdkConfig) -> DynamoDbClient {
    if let Ok(endpoint) = std::env::var("DYNAMODB_ENDPOINT") {
        let config = aws_sdk_dynamodb::config::Builder::from(shared_config)
            .endpoint_url(endpoint)
            .build();
        DynamoDbClient::from_conf(config)
    } else if running_in_sam_local() {
        let config = aws_sdk_dynamodb::config::Builder::from(shared_config)
            .endpoint_url("http://host.docker.internal:8001")
            .build();
        DynamoDbClient::from_conf(config)
    } else {
        DynamoDbClient::new(shared_config)
    }
}

async fn load_shared_config() -> aws_config::SdkConfig {
    let mut loader = aws_config::defaults(BehaviorVersion::latest());
    let local_mode = std::env::var("DYNAMODB_ENDPOINT").is_ok() || running_in_sam_local();

    if let Ok(region) = std::env::var("AWS_REGION") {
        loader = loader.region(aws_config::Region::new(region));
    } else if local_mode {
        loader = loader.region(aws_config::Region::new("us-east-1"));
    }

    if local_mode {
        let access_key =
            std::env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "local".to_string());
        let secret_key =
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| "local".to_string());
        loader = loader.credentials_provider(Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "local",
        ));
    } else if let (Ok(access_key), Ok(secret_key)) = (
        std::env::var("AWS_ACCESS_KEY_ID"),
        std::env::var("AWS_SECRET_ACCESS_KEY"),
    ) {
        loader = loader.credentials_provider(Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "env",
        ));
    }

    loader.load().await
}

fn resolve_management_client(
    shared_config: &aws_config::SdkConfig,
    request: &ApiGatewayWebsocketProxyRequest,
) -> Option<ApiGatewayManagementClient> {
    let endpoint = std::env::var("WEBSOCKET_MANAGEMENT_ENDPOINT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            if running_in_sam_local() {
                Some("http://host.docker.internal:3001".to_string())
            } else {
                None
            }
        })
        .or_else(|| {
            let domain_name = request.request_context.domain_name.as_ref()?;
            let stage = request.request_context.stage.as_ref()?;
            Some(format!("https://{domain_name}/{stage}"))
        })?;

    let config = aws_sdk_apigatewaymanagement::config::Builder::from(shared_config)
        .endpoint_url(endpoint)
        .build();
    Some(ApiGatewayManagementClient::from_conf(config))
}

fn running_in_sam_local() -> bool {
    std::env::var_os("AWS_SAM_LOCAL").is_some()
}

fn resolve_table_name(
    env_key: &str,
    local_logical_id: &str,
    local_name: &str,
    default_name: &str,
    sam_local: bool,
) -> String {
    match std::env::var(env_key) {
        Ok(value) if sam_local && value == local_logical_id => local_name.to_string(),
        Ok(value) => value,
        Err(_) if sam_local => local_name.to_string(),
        Err(_) => default_name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        authenticate_user, new_session_record, new_user_record, parse_and_validate, LoginRequest,
        RegisterRequest,
    };

    #[test]
    fn register_record_normalizes_fields() {
        let user = new_user_record(RegisterRequest {
            username: " Alice ".to_string(),
            password: "password123".to_string(),
            display_name: " Alice Cooper ".to_string(),
        })
        .unwrap();

        assert_eq!(user.username, "alice");
        assert_eq!(user.display_name, "Alice Cooper");
    }

    #[test]
    fn authenticate_user_checks_password_hash() {
        let user = new_user_record(RegisterRequest {
            username: "alice".to_string(),
            password: "password123".to_string(),
            display_name: "Alice".to_string(),
        })
        .unwrap();

        authenticate_user(
            &user,
            &LoginRequest {
                username: "alice".to_string(),
                password: "password123".to_string(),
            },
        )
        .unwrap();
    }

    #[test]
    fn parse_and_validate_rejects_sender_field() {
        let result = parse_and_validate(r#"{"sender":"alice","text":"hello"}"#);
        assert_eq!(
            result.unwrap_err(),
            "Field 'sender' is server-owned and must not be sent by clients."
        );
    }

    #[test]
    fn new_session_record_sets_expiry() {
        let user = new_user_record(RegisterRequest {
            username: "alice".to_string(),
            password: "password123".to_string(),
            display_name: "Alice".to_string(),
        })
        .unwrap();

        let session = new_session_record(&user);
        assert_eq!(session.username, "alice");
        assert!(!session.token.is_empty());
        assert!(!session.expires_at.is_empty());
    }
}
