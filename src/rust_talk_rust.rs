use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response};
use serde::Serialize;

#[derive(Serialize)]
struct ResponsePayload {
    message: String,
}

async fn function_handler(_: Request) -> Result<Response<Body>, Error> {
    let message = ResponsePayload {
        message: format!("Hello Rust Talk from Rust!"),
    };

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&message)?.into())
        .map_err(Box::new)?;

    return Ok(resp);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
