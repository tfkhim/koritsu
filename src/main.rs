use axum::{Router, routing::get};
use std::io::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route("/hello", get(async || "Hello World!"));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
