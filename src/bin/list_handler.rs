use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Body, Error, Request, Response};
use book_api::{DataAccess, DynamoDbDataAccess};
use std::env;

/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);
    let data_access = DynamoDbDataAccess::new(dynamodb_client, table_name);

    // Setup the tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Register the Lambda handler
    lambda_http::run(service_fn(|request: Request| {
        list_item(&data_access, request)
    }))
    .await?;

    Ok(())
}

/// Scan items from DynamoDB
async fn list_item<T: DataAccess>(
    data_access: &T,
    _request: Request,
) -> Result<Response<Body>, Error> {

    // List item in the DynamoDB table
    let res = data_access.list().await;

    tracing::info!("Listing results res [{:?}] ", res);

    // Return a response to the end-user
    match res {
        Ok(query_result) => Ok(Response::builder().status(200).body(query_result.into())?),
        Err(_) => Ok(Response::builder()
            .status(500)
            .body("internal error".into())?),
    }
}
