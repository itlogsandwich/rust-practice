
use crate::error::ApiError;
use axum::{routing::get, Router, response::IntoResponse, response::Html, extract::Path};

mod error;

async fn health_check() -> impl IntoResponse
{
    println!("--> {:<12} - health_check - ", "HANDLER");

    Html(format!("<strong> Merry Christmas! </strong>"))
}

async fn index() -> impl IntoResponse
{
    println!("--> {:<12} - index - ", "HANDLER");

    Html(format!("<strong> LOOK WHO WE HAVE HERE </strong>"))
}

async fn index_user(Path(name): Path<String> ) -> impl IntoResponse
{ 
    println!("--> {:<12} - index_user - ", "HANDLER");
    
    Html(format!("<strong> Oh, it's you, {name} </strong>"))
}

fn create_app() -> Router
{
    Router::new()
        .route("/health", get(health_check))
        .route("/", get(index))
        .route("/{name}", get(index_user))
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
