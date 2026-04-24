use std::{collections::HashMap, sync::Arc};

use aws_lambda_events::{
    event::apigw::{
        ApiGatewayWebsocketProxyRequest, ApiGatewayWebsocketProxyRequestContext,
    },
};
use axum::{
    body::Bytes,
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{SinkExt, StreamExt};
use query_map::QueryMap;
use serde::Deserialize;
use tokio::sync::{mpsc::{unbounded_channel, UnboundedSender}, RwLock};
use tracing::{error, info, warn};

use simple_chat_backend::aws_lambda::{
    handle_ws_connect, handle_ws_disconnect, handle_ws_send_message,
};

#[derive(Clone, Default)]
struct GatewayState {
    peers: Arc<RwLock<HashMap<String, UnboundedSender<String>>>>,
}

#[derive(Deserialize)]
struct ConnectQuery {
    token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    simple_chat_backend::telemetry::init_tracing();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    info!("local websocket gateway listening on 127.0.0.1:3001");

    let app = Router::new()
        .route("/ws/chat", get(ws_chat))
        .route("/@connections/:connection_id", post(post_to_connection))
        .with_state(GatewayState::default());

    axum::serve(listener, app).await?;
    Ok(())
}

async fn ws_chat(
    ws: WebSocketUpgrade,
    State(state): State<GatewayState>,
    Query(query): Query<ConnectQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, query.token.unwrap_or_default()))
}

async fn handle_socket(socket: WebSocket, state: GatewayState, token: String) {
    let connection_id = format!(
        "local-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    );

    let connect_request = websocket_request(&connection_id, Some(token), None, "$connect", "CONNECT");
    match handle_ws_connect(connect_request).await {
        Ok(response) if response.status_code < 400 => {}
        Ok(response) => {
            warn!(connection_id = %connection_id, body = %response.body, "websocket connect rejected");
            return;
        }
        Err(error) => {
            error!(connection_id = %connection_id, error = %error, "websocket connect handler failed");
            return;
        }
    }

    let (sender, mut receiver) = unbounded_channel::<String>();
    state.peers.write().await.insert(connection_id.clone(), sender);

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let writer = tokio::spawn(async move {
        while let Some(payload) = receiver.recv().await {
            if ws_sender.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let request = websocket_request(&connection_id, None, Some(text.to_string()), "$default", "MESSAGE");
                if let Err(error) = handle_ws_send_message(request).await {
                    error!(connection_id = %connection_id, error = %error, "message handler failed");
                    break;
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => {}
            Err(error) => {
                error!(connection_id = %connection_id, error = %error, "websocket receive failed");
                break;
            }
        }
    }

    state.peers.write().await.remove(&connection_id);
    writer.abort();

    if let Err(error) = handle_ws_disconnect(websocket_request(
        &connection_id,
        None,
        None,
        "$disconnect",
        "DISCONNECT",
    ))
    .await
    {
        error!(connection_id = %connection_id, error = %error, "disconnect handler failed");
    }
}

async fn post_to_connection(
    State(state): State<GatewayState>,
    Path(connection_id): Path<String>,
    body: Bytes,
) -> impl IntoResponse {
    let payload = match String::from_utf8(body.to_vec()) {
        Ok(payload) => payload,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let peers = state.peers.read().await;
    match peers.get(&connection_id) {
        Some(sender) if sender.send(payload).is_ok() => StatusCode::OK,
        Some(_) => StatusCode::GONE,
        None => StatusCode::GONE,
    }
}

fn websocket_request(
    connection_id: &str,
    token: Option<String>,
    body: Option<String>,
    route_key: &str,
    event_type: &str,
) -> ApiGatewayWebsocketProxyRequest {
    let mut request = ApiGatewayWebsocketProxyRequest::default();
    request.body = body;
    request.query_string_parameters = match token {
        Some(token) => QueryMap::from(HashMap::from([(String::from("token"), token)])),
        None => QueryMap::default(),
    };
    let mut context = ApiGatewayWebsocketProxyRequestContext::default();
    context.route_key = Some(route_key.to_string());
    context.event_type = Some(event_type.to_string());
    context.connection_id = Some(connection_id.to_string());
    context.domain_name = Some("127.0.0.1:3001".to_string());
    context.stage = Some("local".to_string());
    request.request_context = context;
    request
}