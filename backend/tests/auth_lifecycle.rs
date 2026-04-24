use std::{net::SocketAddr, time::Duration};

use futures::{SinkExt, StreamExt};
use http::HeaderValue;
use reqwest::Client;
use serde_json::Value;
use simple_chat_backend::{build_app, Settings};
use tokio::net::TcpListener;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, protocol::Message},
};

const ALLOWED_ORIGIN: &str = "http://localhost:5173";

struct TestServer {
    address: SocketAddr,
    handle: tokio::task::JoinHandle<()>,
}

impl TestServer {
    fn http_url(&self, path: &str) -> String {
        format!("http://{}{}", self.address, path)
    }

    fn ws_url(&self, path: &str) -> String {
        format!("ws://{}{}", self.address, path)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

async fn spawn_server(session_ttl_seconds: i64) -> TestServer {
    let settings = Settings {
        session_ttl_seconds,
        allowed_origins: std::sync::Arc::new(vec![ALLOWED_ORIGIN.to_string()]),
        allowed_origin_headers: std::sync::Arc::new(vec![http::HeaderValue::from_static(
            ALLOWED_ORIGIN,
        )]),
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = build_app(settings);
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestServer { address, handle }
}

async fn register_user(server: &TestServer) -> Value {
    let client = Client::new();
    let response = client
        .post(server.http_url("/auth/register"))
        .header("Origin", ALLOWED_ORIGIN)
        .json(&serde_json::json!({
            "username": "alice",
            "password": "password123",
            "displayName": "Alice"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    response.json().await.unwrap()
}

async fn expect_policy_close(url: String, origin: &str) {
    let mut request = url.into_client_request().unwrap();
    request
        .headers_mut()
        .insert("Origin", HeaderValue::from_str(origin).unwrap());
    let (mut socket, _) = connect_async(request).await.unwrap();

    match socket.next().await {
        Some(Ok(Message::Close(_))) => {}
        other => panic!("expected policy close, got {other:?}"),
    }
}

#[tokio::test]
async fn register_returns_expiry_and_cors_header() {
    let server = spawn_server(5).await;
    let client = Client::new();
    let response = client
        .post(server.http_url("/auth/register"))
        .header("Origin", ALLOWED_ORIGIN)
        .json(&serde_json::json!({
            "username": "alice",
            "password": "password123",
            "displayName": "Alice"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    assert!(response
        .headers()
        .get("access-control-allow-origin")
        .is_some());

    let payload: Value = response.json().await.unwrap();
    assert!(payload.get("expiresAt").is_some());
}

#[tokio::test]
async fn health_reports_minimum_service_telemetry() {
    let server = spawn_server(5).await;
    let client = Client::new();

    let response = client.get(server.http_url("/health")).send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let payload: Value = response.json().await.unwrap();
    assert_eq!(payload["status"], "ok");
    assert!(payload["telemetry"]["uptimeSeconds"].is_number());
    assert!(payload["telemetry"]["counters"]["authRequestsTotal"].is_number());
    assert!(payload["telemetry"]["counters"]["broadcastAttemptsTotal"].is_number());
    assert!(payload["telemetry"]["indicators"]["authSuccessRate"].is_number());
    assert!(payload["telemetry"]["sloTargets"]["availabilityTarget"].is_number());
}

#[tokio::test]
async fn register_rejects_disallowed_http_origin() {
    let server = spawn_server(5).await;
    let client = Client::new();
    let response = client
        .post(server.http_url("/auth/register"))
        .header("Origin", "http://evil.example")
        .json(&serde_json::json!({
            "username": "alice",
            "password": "password123",
            "displayName": "Alice"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn logout_revokes_token_for_new_websocket_connections() {
    let server = spawn_server(5).await;
    let payload = register_user(&server).await;
    let token = payload.get("token").unwrap().as_str().unwrap();

    let client = Client::new();
    let logout = client
        .post(server.http_url("/auth/logout"))
        .header("Origin", ALLOWED_ORIGIN)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .unwrap();

    assert_eq!(logout.status(), reqwest::StatusCode::NO_CONTENT);
    expect_policy_close(
        server.ws_url(&format!("/ws/chat?token={token}")),
        ALLOWED_ORIGIN,
    )
    .await;
}

#[tokio::test]
async fn expired_session_is_rejected_for_new_websocket_connections() {
    let server = spawn_server(1).await;
    let payload = register_user(&server).await;
    let token = payload.get("token").unwrap().as_str().unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;

    expect_policy_close(
        server.ws_url(&format!("/ws/chat?token={token}")),
        ALLOWED_ORIGIN,
    )
    .await;
}

#[tokio::test]
async fn disallowed_websocket_origin_is_rejected() {
    let server = spawn_server(5).await;
    let payload = register_user(&server).await;
    let token = payload.get("token").unwrap().as_str().unwrap();

    expect_policy_close(
        server.ws_url(&format!("/ws/chat?token={token}")),
        "http://evil.example",
    )
    .await;
}

#[tokio::test]
async fn invalid_payload_returns_error_without_disconnect() {
    let server = spawn_server(5).await;
    let payload = register_user(&server).await;
    let token = payload.get("token").unwrap().as_str().unwrap();

    let mut request = server
        .ws_url(&format!("/ws/chat?token={token}"))
        .into_client_request()
        .unwrap();
    request
        .headers_mut()
        .insert("Origin", HeaderValue::from_static(ALLOWED_ORIGIN));
    let (mut socket, _) = connect_async(request).await.unwrap();

    let _ = socket.next().await;

    socket
        .send(Message::Text(r#"{"sender":"mallory","text":"hi"}"#.into()))
        .await
        .unwrap();

    let error = next_json_message(&mut socket).await;
    assert_eq!(error["type"], "error");
    assert_eq!(error["text"], "Field 'sender' is server-owned and must not be sent by clients.");

    socket
        .send(Message::Text(r#"{"text":"hello after error"}"#.into()))
        .await
        .unwrap();

    let chat = next_json_message(&mut socket).await;
    assert_eq!(chat["type"], "message");
    assert_eq!(chat["text"], "hello after error");
}

#[tokio::test]
async fn revoked_session_errors_active_socket_on_next_message() {
    let server = spawn_server(30).await;
    let payload = register_user(&server).await;
    let token = payload.get("token").unwrap().as_str().unwrap();

    let mut request = server
        .ws_url(&format!("/ws/chat?token={token}"))
        .into_client_request()
        .unwrap();
    request
        .headers_mut()
        .insert("Origin", HeaderValue::from_static(ALLOWED_ORIGIN));
    let (mut socket, _) = connect_async(request).await.unwrap();

    let _ = socket.next().await;

    let client = Client::new();
    let logout = client
        .post(server.http_url("/auth/logout"))
        .header("Origin", ALLOWED_ORIGIN)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .unwrap();
    assert_eq!(logout.status(), reqwest::StatusCode::NO_CONTENT);

    let send_result = socket
        .send(Message::Text(r#"{"text":"still here?"}"#.into()))
        .await;

    match send_result {
        Ok(()) => {
            match tokio::time::timeout(Duration::from_secs(5), socket.next()).await {
                Ok(Some(Ok(Message::Text(text)))) => {
                    let error: Value = serde_json::from_str(text.as_ref()).unwrap();
                    assert_eq!(error["type"], "error");
                    assert_eq!(error["text"], "Authentication required or session expired.");

                    match socket.next().await {
                        Some(Ok(Message::Close(_))) | Some(Err(_)) | None => {}
                        other => panic!("expected socket termination after revoked session, got {other:?}"),
                    }
                }
                Ok(Some(Err(_))) | Ok(None) => {}
                Ok(Some(other)) => panic!("expected error frame or termination after revocation, got {other:?}"),
                Err(_) => panic!("timed out waiting for revoked-session termination"),
            }
        }
        Err(_) => {}
    }
}

#[tokio::test]
async fn message_history_returns_recent_page_and_older_messages() {
    let server = spawn_server(30).await;
    let payload = register_user(&server).await;
    let token = payload.get("token").unwrap().as_str().unwrap();

    let mut request = server
        .ws_url(&format!("/ws/chat?token={token}"))
        .into_client_request()
        .unwrap();
    request
        .headers_mut()
        .insert("Origin", HeaderValue::from_static(ALLOWED_ORIGIN));
    let (mut socket, _) = connect_async(request).await.unwrap();

    let _ = socket.next().await;

    socket
        .send(Message::Text(r#"{"text":"first saved message"}"#.into()))
        .await
        .unwrap();
    let first = next_json_message(&mut socket).await;

    socket
        .send(Message::Text(r#"{"text":"second saved message"}"#.into()))
        .await
        .unwrap();
    let second = next_json_message(&mut socket).await;

    let client = Client::new();
    let recent = client
        .get(server.http_url("/auth/messages?limit=1"))
        .header("Origin", ALLOWED_ORIGIN)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .unwrap();

    assert_eq!(recent.status(), reqwest::StatusCode::OK);
    let recent_payload: Value = recent.json().await.unwrap();
    assert_eq!(recent_payload["messages"][0]["text"], "second saved message");
    assert_eq!(recent_payload["messages"][0]["id"], second["id"]);
    assert_eq!(recent_payload["hasMore"], true);

    let before = recent_payload["nextBefore"].as_str().unwrap();
    let older = client
        .get(server.http_url(&format!("/auth/messages?limit=1&before={before}")))
        .header("Origin", ALLOWED_ORIGIN)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .unwrap();

    assert_eq!(older.status(), reqwest::StatusCode::OK);
    let older_payload: Value = older.json().await.unwrap();
    assert_eq!(older_payload["messages"][0]["text"], "first saved message");
    assert_eq!(older_payload["messages"][0]["id"], first["id"]);
    assert_eq!(older_payload["hasMore"], false);
}

async fn next_json_message(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Value {
    let message = tokio::time::timeout(Duration::from_secs(5), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();

    match message {
        Message::Text(text) => serde_json::from_str(text.as_ref()).unwrap(),
        other => panic!("expected text frame, got {other:?}"),
    }
}
