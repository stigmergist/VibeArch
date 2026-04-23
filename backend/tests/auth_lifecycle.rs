use std::{net::SocketAddr, time::Duration};

use futures::StreamExt;
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
