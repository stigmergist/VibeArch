use aws_lambda_events::event::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use simple_chat_backend::aws_lambda::handle_ws_disconnect;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(handler)).await
}

async fn handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<impl serde::Serialize, Error> {
    handle_ws_disconnect(event.payload).await
}
