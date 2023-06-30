use async_trait::async_trait;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use mockall::automock;
use serde_json::Value;

#[automock]
#[async_trait]
pub trait DataAccess {
    async fn create(&self, isbn: String, payload: String) -> Result<(), ()>;
    async fn get(&self, isbn: String) -> Result<String, ()>;
    async fn list(&self) -> Result<String, ()>;
    async fn delete(&self, isbn: String) -> Result<(), ()>;
}

pub struct DynamoDbDataAccess {
    client: Client,
    table_name: String,
}

impl DynamoDbDataAccess {
    pub fn new(client: Client, table_name: String) -> DynamoDbDataAccess {
        DynamoDbDataAccess {
            client: client,
            table_name: table_name,
        }
    }
}

#[async_trait]
impl DataAccess for DynamoDbDataAccess {
    async fn create(&self, isbn: String, payload: String) -> Result<(), ()> {
        let res = &self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("isbn", AttributeValue::S(isbn.to_string()))
            .item("payload", AttributeValue::S(payload))
            .send()
            .await;

        // Log response from DDB
        tracing::info!("Response [{:?}] ", res);

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn get(&self, isbn: String) -> Result<String, ()> {
        let res = &self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("isbn", AttributeValue::S(isbn.to_string()))
            .send()
            .await;

        // Log response from DDB
        tracing::info!("Response [{:?}] ", res);

        // Return a response to the end-user
        match res {
            Ok(query_result) => {
                let payload = query_result.item().expect("Payload attribute should exist");

                Ok(payload["payload"].as_s().unwrap().to_string().into())
            }
            Err(_) => Err(()),
        }
    }

    async fn list(&self) -> Result<String, ()> {
        let res = &self
            .client
            .scan()
            .table_name(&self.table_name)
            .send()
            .await;

        // Log response from DDB
        tracing::info!("Response [{:?}] ", res);

        // Return a response to the end-user
        match res {
            Ok(query_result) => {
                let items = query_result.items().expect("Items should exist");

                let json_array: Vec<Value> = items
                    .into_iter()
                    .filter_map(|item| {
                        if let Some(AttributeValue::S(payload_value)) = item.get("payload") {
                            serde_json::from_str(&payload_value).ok()
                        } else {
                            None
                        }
                    })
                    .collect();

                let json_string = serde_json::to_string(&json_array).unwrap();

                Ok(json_string)
            }
            Err(_) => Err(()),
        }
    }

    async fn delete(&self, isbn: String) -> Result<(), ()> {
        let res = &self
            .client
            .delete_item()
            .table_name(&self.table_name)
            .key("isbn", AttributeValue::S(isbn.to_string()))
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}