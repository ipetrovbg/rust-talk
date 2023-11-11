use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response};
use serde::Serialize;

#[derive(Serialize)]
struct ResponsePayload {
    sum: usize,
    message: String,
}

async fn function_handler(_: Request) -> Result<Response<Body>, Error> {
    let numbers: Vec<usize> = (0..100_000_000).collect();
    let sum = numbers.iter().sum();

    let response = ResponsePayload {
        message: format!("Sum of 0..100,000,000 is {}", sum),
        sum,
    };

    let resp = Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&response)?.into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
