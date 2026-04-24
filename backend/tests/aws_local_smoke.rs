use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::{SinkExt, StreamExt};
use reqwest::Client;
use serde_json::Value;
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const AUTH_BASE_URL: &str = "http://127.0.0.1:3000/auth";
const CHAT_WS_URL: &str = "ws://127.0.0.1:3001/ws/chat";

#[tokio::test]
#[ignore = "requires the SAM-local auth API, local websocket gateway, and DynamoDB local to be running"]
async fn sam_local_auth_and_websocket_round_trip() {
    run_smoke_round_trip(AUTH_BASE_URL, CHAT_WS_URL).await;
}

#[tokio::test]
#[ignore = "requires deployed AWS auth and websocket endpoints via SMOKE_AUTH_BASE_URL and SMOKE_CHAT_WS_URL"]
async fn deployed_aws_auth_and_websocket_round_trip() {
    let auth_base_url = std::env::var("SMOKE_AUTH_BASE_URL")
        .expect("SMOKE_AUTH_BASE_URL must be set for the deployed AWS smoke test");
    let chat_ws_url =
        std::env::var("SMOKE_CHAT_WS_URL").expect("SMOKE_CHAT_WS_URL must be set for the deployed AWS smoke test");

    run_smoke_round_trip(&auth_base_url, &chat_ws_url).await;
}

async fn run_smoke_round_trip(auth_base_url: &str, chat_ws_url: &str) {
    let username = format!(
        "smoke-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let display_name = "Smoke Tester";
    let password = "password123";
    let client = Client::new();

    let response = client
        .post(format!("{auth_base_url}/register"))
        .json(&serde_json::json!({
            "username": username,
            "password": password,
            "displayName": display_name,
        }))
        .send()
        .await
        .expect("register request should succeed");

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    let session: Value = response.json().await.expect("register response should be JSON");
    let token = session["token"]
        .as_str()
        .expect("register response should include a token");

    let (mut socket, _) = connect_async(format!("{chat_ws_url}?token={token}"))
        .await
        .expect("websocket should connect with the session token");

    socket
        .send(Message::Text(
            serde_json::json!({ "text": "hello from the smoke test" })
                .to_string()
                .into(),
        ))
        .await
        .expect("chat message should send");

    let chat = next_text_message(&mut socket).await;
    assert_eq!(chat["type"], "message");
    assert_eq!(chat["sender"], display_name);
    assert_eq!(chat["text"], "hello from the smoke test");

    socket.close(None).await.expect("socket should close cleanly");
}

async fn next_text_message(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Value {
    let message = timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("websocket should produce a message within timeout")
        .expect("websocket stream should remain open")
        .expect("websocket message should be readable");

    match message {
        Message::Text(text) => serde_json::from_str(text.as_ref())
            .expect("websocket text frame should contain JSON"),
        other => panic!("expected text frame, got {other:?}"),
    }
}