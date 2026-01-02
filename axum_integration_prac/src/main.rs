use crate::error::Error;
use axum::routing::get;
use axum::response::{IntoResponse, Html};
use axum::extract::Path;
use axum::Router;

mod error;

async fn balance() -> impl IntoResponse
{
    println!("--> {:<12} - balance - ", "HANDLER");

    Html(format!("$100"))
}

async fn hello_world() -> impl IntoResponse
{ 
    println!("--> {:<12} - hello_world - ", "HANDLER");

    Html(format!("Hello, World!"))
}

fn create_app() -> Router
{
    Router::new()
        .route("/", get(hello_world))
        .route("/balance", get(balance))
}

#[tokio::main]
async fn main() -> Result<(), Error>
{

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await?;

    Ok(())
}
