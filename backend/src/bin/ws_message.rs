use aws_lambda_events::event::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use simple_chat_backend::{aws_lambda::handle_ws_send_message, telemetry};

#[tokio::main]
async fn main() -> Result<(), Error> {
    telemetry::init_tracing();
    run(service_fn(handler)).await
}

async fn handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<impl serde::Serialize, Error> {
    handle_ws_send_message(event.payload).await
}
