use std::{net::SocketAddr, sync::Arc};

use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use http::HeaderValue;
use reqwest::Client;
use serde_json::Value;
use simple_chat_backend::{
    aws_lambda::{new_session_record_with_policy, new_user_record, RegisterRequest},
    build_app,
    runtime_contract::{self, SessionPolicy},
    Settings,
};
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
        allowed_origins: Arc::new(vec![ALLOWED_ORIGIN.to_string()]),
        allowed_origin_headers: Arc::new(vec![HeaderValue::from_static(ALLOWED_ORIGIN)]),
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        axum::serve(listener, build_app(settings)).await.unwrap();
    });

    TestServer { address, handle }
}

async fn register_user(server: &TestServer, username: &str, display_name: &str) -> Value {
    let client = Client::new();
    let response = client
        .post(server.http_url("/auth/register"))
        .header("Origin", ALLOWED_ORIGIN)
        .json(&serde_json::json!({
            "username": username,
            "password": "password123",
            "displayName": display_name,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    response.json().await.unwrap()
}

async fn next_json_message(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Value {
    match socket.next().await {
        Some(Ok(Message::Text(text))) => serde_json::from_str(text.as_ref()).unwrap(),
        other => panic!("expected text frame, got {other:?}"),
    }
}

#[tokio::test]
async fn local_handler_and_aws_session_path_share_ttl_policy() {
    let ttl_seconds = 90;
    let session_policy = SessionPolicy::from_ttl_seconds(ttl_seconds).unwrap();
    let server = spawn_server(ttl_seconds).await;
    let local_session = register_user(&server, "alice", "Alice").await;
    let local_expires_at = DateTime::parse_from_rfc3339(
        local_session.get("expiresAt").unwrap().as_str().unwrap(),
    )
    .unwrap()
    .with_timezone(&Utc);

    let aws_user = new_user_record(RegisterRequest {
        username: "bob".to_string(),
        password: "password123".to_string(),
        display_name: "Bob".to_string(),
    })
    .unwrap();
    let aws_session = new_session_record_with_policy(&aws_user, session_policy);
    let aws_expires_at = DateTime::parse_from_rfc3339(&aws_session.expires_at)
        .unwrap()
        .with_timezone(&Utc);

    let now = Utc::now();
    let local_remaining = local_expires_at.signed_duration_since(now).num_seconds();
    let aws_remaining = aws_expires_at.signed_duration_since(now).num_seconds();

    assert!(local_remaining >= ttl_seconds - 2);
    assert!(local_remaining <= ttl_seconds + 2);
    assert!(aws_remaining >= ttl_seconds - 2);
    assert!(aws_remaining <= ttl_seconds + 2);
    assert!((local_remaining - aws_remaining).abs() <= 2);
}

#[tokio::test]
async fn local_handler_uses_shared_payload_validation_contract() {
    let server = spawn_server(30).await;
    let session = register_user(&server, "alice", "Alice").await;
    let token = session.get("token").unwrap().as_str().unwrap();
    let invalid_payload = r#"{"sender":"mallory","text":"hi"}"#;

    let expected_error = runtime_contract::parse_and_validate_chat_text(invalid_payload)
        .unwrap_err()
        .to_string();

    let mut request = server
        .ws_url(&format!("/ws/chat?token={token}"))
        .into_client_request()
        .unwrap();
    request
        .headers_mut()
        .insert("Origin", HeaderValue::from_static(ALLOWED_ORIGIN));
    let (mut socket, _) = connect_async(request).await.unwrap();

    let _ = next_json_message(&mut socket).await;

    socket
        .send(Message::Text(invalid_payload.to_string().into()))
        .await
        .unwrap();

    let error = next_json_message(&mut socket).await;
    assert_eq!(error["type"], "error");
    assert_eq!(error["text"], expected_error);

    socket
        .send(Message::Text(r#"{"text":"hello after error"}"#.to_string().into()))
        .await
        .unwrap();

    let chat = next_json_message(&mut socket).await;
    assert_eq!(chat["type"], "message");
    assert_eq!(chat["text"], "hello after error");
}