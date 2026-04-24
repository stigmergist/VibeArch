use simple_chat_backend::{run, Settings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let settings = Settings::from_env().map_err(|error| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("settings error: {error}"))
    })?;

    run(settings).await
}
