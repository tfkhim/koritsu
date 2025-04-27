use axum::{Router, routing::post};
use std::io::Error;
use tokio::net::TcpListener;

mod github_events_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route(
        "/github/events",
        post(github_events_handler::handle_github_workflow_run),
    );

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
