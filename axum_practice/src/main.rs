#![allow(unused)]

pub use self::error::{Error, Result};
use std::net::SocketAddr;
use axum::{Router, Json, middleware};
use axum::extract::{Query, Path};
use axum::routing::get;
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;


mod error;
mod web;

fn routes_hello() -> Router
{
    Router::new().route(
            "/hello",
            get(handler_hello)).route(
            "/hello2/{name}",
            get(handler_hello_name))
}

fn routes_static() -> Router
{
    Router::new().fallback_service(ServeDir::new("./"))
}
#[derive(Debug, Deserialize)]
struct HelloParams 
{
    name:Option<String>,
}

// mappped to /hello?name=Jen
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse
{
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong> {name}!!! </strong>"))
}

//mapped to path //hello/Name

async fn handler_hello_name(Path(name): Path<String>) -> impl IntoResponse
{
    println!("->> {:<12} - handler_hello_name - {name:?}", "HANDLER");

    Html(format!("Hello <strong> {name}!!! </strong>"))
}

async fn main_response_mapper(res: Response) -> Response
{
    println!("->> {:<12} - main_response_mapper", "HANDLER");

    println!();
    res 
}

#[tokio::main]
async fn main()
{
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());


    let addr = SocketAddr::from(([127, 0 , 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum_server::bind(addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

