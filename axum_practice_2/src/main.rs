
use crate::error::ApiError;
use axum::{routing::get, Router, response::IntoResponse, response::Html};

mod error;

async fn health_check() -> impl IntoResponse
{
    println!("--> {:<12} - health_check - ", "HANDLER");

    Html(format!("<strong> Merry Christmas! </strong>"))
}
fn create_app() -> Router
{
    Router::new()
        .route("/health", get(health_check))
}

#[tokio::main]
async fn main() -> Result<(), ApiError>
{
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await?;

    Ok(())

}
