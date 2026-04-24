use lambda_http::{run, service_fn, Error};
use simple_chat_backend::{aws_lambda::handle_auth_http, telemetry};

#[tokio::main]
async fn main() -> Result<(), Error> {
    telemetry::init_tracing();
    run(service_fn(handle_auth_http)).await
}
