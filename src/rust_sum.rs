use lambda_http::{http::StatusCode, run, service_fn, Error, IntoResponse, Request, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct ResponseModel<'a> {
    sum: u64,
    message: &'a str,
}

async fn function_handler(_: Request) -> Result<impl IntoResponse, Error> {
    let mut sum: u64 = 0;
    for i in 0..100_000_000 {
        sum += i;
    }

    let response = ResponseModel {
        message: "The sum of 0..100,000,000",
        sum,
    };

    let resp = Response::builder()
        .status(StatusCode::OK)
        .body(json!(response).to_string())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
