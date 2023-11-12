use lambda_http::{http::StatusCode, run, service_fn, Error, IntoResponse, Request, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct ResponsePayload<'a> {
    message: &'a str,
}

fn response(status: StatusCode, body: String) -> Result<Response<String>, Error> {
    let response = Response::builder()
        .status(status)
        .body(body)
        .map_err(Box::new)?;

    Ok(response)
}

async fn function_handler(_: Request) -> Result<impl IntoResponse, Error> {
    let message = ResponsePayload {
        message: "Hello Rust Talk from Rust!",
    };

    let result = response(StatusCode::OK, json!(message).to_string())?;

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
