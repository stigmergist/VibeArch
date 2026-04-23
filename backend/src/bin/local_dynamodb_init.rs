use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_dynamodb::{
    config::Builder,
    types::{
        AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType,
        TimeToLiveSpecification,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let endpoint = std::env::var("DYNAMODB_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:8001".to_string());
    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
    let access_key = std::env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "local".to_string());
    let secret_key =
        std::env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| "local".to_string());

    let shared = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_config::Region::new(region))
        .credentials_provider(Credentials::new(access_key, secret_key, None, None, "local"))
        .load()
        .await;

    let config = Builder::from(&shared).endpoint_url(endpoint.clone()).build();
    let client = Client::from_conf(config);

    ensure_table(&client, "simple-chat-users-local", "username").await?;
    ensure_table(&client, "simple-chat-sessions-local", "token").await?;
    ensure_table(&client, "simple-chat-connections-local", "connectionId").await?;
    enable_ttl(&client, "simple-chat-sessions-local", "expiresAtEpoch").await?;

    println!("local DynamoDB tables are ready at {endpoint}");
    Ok(())
}

async fn ensure_table(
    client: &Client,
    table_name: &str,
    attribute_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let exists = client.describe_table().table_name(table_name).send().await.is_ok();
    if exists {
        println!("table already exists: {table_name}");
        return Ok(());
    }

    let create_result = client
        .create_table()
        .table_name(table_name)
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name(attribute_name)
                .attribute_type(ScalarAttributeType::S)
                .build()?,
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name(attribute_name)
                .key_type(KeyType::Hash)
                .build()?,
        )
        .send()
        .await;

    if let Err(error) = create_result {
        if !error.to_string().contains("ResourceInUseException") {
            return Err(error.into());
        }
    }

    println!("created table: {table_name}");
    Ok(())
}

async fn enable_ttl(
    client: &Client,
    table_name: &str,
    attribute_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = client
        .update_time_to_live()
        .table_name(table_name)
        .time_to_live_specification(
            TimeToLiveSpecification::builder()
                .enabled(true)
                .attribute_name(attribute_name)
                .build()?,
        )
        .send()
        .await;
    Ok(())
}