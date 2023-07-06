use aws_sdk_dynamodb::Client;
use lambda_http::{service_fn, Body, Error, Request, RequestExt, Response};
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
        delete_item(&data_access, request)
    }))
    .await?;

    Ok(())
}

/// Delete an Item from DynamoDB
async fn delete_item<T: DataAccess>(
    data_access: &T,
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

    let res = data_access.delete(isbn.to_string()).await;

    // Return a response to the end-user
    match res {
        Ok(_) => Ok(Response::builder()
            .status(204)
            .body("item deleted".into())?),
        Err(_) => Ok(Response::builder()
            .status(500)
            .body("internal error".into())?),
    }
}