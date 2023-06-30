use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{service_fn, Body, Error, Request, RequestExt, Response};
use std::env;

/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);

    // Setup the tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Register the Lambda handler
    lambda_http::run(service_fn(|request: Request| {

        let res = put_item(&dynamodb_client, &table_name, request);

        res
    }))
    .await?;

    Ok(())
}

/// Put Item Lambda function
async fn put_item(
    client: &Client,
    table_name: &str,
    request: Request,
) -> Result<Response<Body>, Error> {
    // Extract path parameter from request
    let path_parameters = request.path_parameters();
    let isbn = match path_parameters.first("isbn") {
        Some(isbn) => isbn,
        None => {
            return Ok(Response::builder()
                .status(400)
                .body("isbn is required".into())?)
        }
    };

    // Extract body from request
    let body = match request.body() {
        Body::Empty => "".to_string(),
        Body::Text(body) => body.clone(),
        Body::Binary(body) => String::from_utf8_lossy(body).to_string(),
    };

    // Put the item in the DynamoDB table
    let res = client
        .put_item()
        .table_name(table_name)
        .item("isbn", AttributeValue::S(isbn.to_string()))
        .item("payload", AttributeValue::S(body))
        .send()
        .await;

    // Return a response to the end-user
    match res {
        Ok(_) => Ok(Response::builder().status(200).body("item saved".into())?),
        Err(_) => Ok(Response::builder()
            .status(500)
            .body("internal error".into())?),
    }
}