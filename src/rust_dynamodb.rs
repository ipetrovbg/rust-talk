use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client as DynamoClient;

use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response};

use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "userName")]
    user_name: String,
    #[serde(rename = "userEmail")]
    user_email: String,
    #[serde(rename = "userRole")]
    user_role: String,
}

#[derive(Serialize)]
struct ResponsePayload {
    message: String,
}

async fn function_handler(_: Request) -> Result<Response<Body>, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = DynamoClient::new(&config);

    let admin_query_response = client
        .query()
        .table_name("Users")
        .key_condition_expression("userRole = :userRole")
        .expression_attribute_values(":userRole", AttributeValue::S(format!("admin")))
        .send()
        .await?;

    let items = admin_query_response.items.unwrap_or(vec![]);
    let admins_items: Result<Vec<User>, _> = from_items(items);
    let builder = Response::builder();

    let response = match admins_items {
        Ok(admins) => builder
            .status(StatusCode::OK)
            .body(serde_json::to_string(&admins)?.into())
            .map_err(Box::new)?,
        Err(_) => {
            let error_message = ResponsePayload {
                message: format!("Error parsing admins"),
            };

            builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(serde_json::to_string(&error_message)?.into())
                .map_err(Box::new)?
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
