use lambda_http::{run, service_fn, Body, Error, Request, Response};

async fn function_handler(_: Request) -> Result<Response<Body>, Error> {
    let message = format!("Hello Rust Talk from Rust!");

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
