use lambda_http::{run, service_fn, tracing, Error};
use simple_chat_backend::aws_lambda::handle_auth_http;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(handle_auth_http)).await
}
