use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client as DynamoClient;

use lambda_http::{
    http::StatusCode, run, service_fn, Body, Error, IntoResponse, Request, Response,
};

use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;
use serde_json::json;

#[derive(Serialize, Deserialize)]
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
struct ErrorPayload<'a> {
    message: &'a str,
    error: String,
}

fn response(status: StatusCode, body: String) -> Result<Response<String>, Error> {
    let response = Response::builder()
        .status(status)
        .body(body)
        .map_err(Box::new)?;

    Ok(response)
}

async fn function_handler(_: Request) -> Result<impl IntoResponse, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = DynamoClient::new(&config);

    let admin_query_response = client
        .query()
        .table_name("Users")
        .key_condition_expression("userRole = :userRole")
        .expression_attribute_values(":userRole", AttributeValue::S("admin".to_string()))
        .send()
        .await?;

    let items = admin_query_response.items.unwrap_or(vec![]);
    let admins_result: Result<Vec<User>, serde_dynamo::Error> = from_items(items);

    let result = match admins_result {
        Ok(admins) => {
            let admins_string = json!(admins).to_string();
            let result = response(StatusCode::OK, admins_string)?;

            result
        }
        Err(e) => {
            let error_payload = ErrorPayload {
                message: "Error while parsing admins",
                error: e.to_string(),
            };
            let error_string = json!(error_payload).to_string();
            let result = response(StatusCode::INTERNAL_SERVER_ERROR, error_string)?;
            result
        }
    };

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
