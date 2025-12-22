#![allow(unused)]
pub use self::error::{Error, Result};

use crate::model::ModelController;
use axum::extract::{Path, Query};
use axum::handler::HandlerWithoutStateExt as _;
use axum::http::{Method, StatusCode, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{MethodRouter, get, get_service};
use axum::{Json, Router, middleware};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod model;
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
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});            println!(" ->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    println!();
    res 
}

#[tokio::main]
async fn main() -> Result<()>
{
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
                mc.clone(),
                web::mw_auth::mw_ctx_resolve,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());


    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .map_err(|err| format!("Cannot start TcpListener. \nCause: {err}"))?;
 
    println!("->> LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes_all)
        .await
        .map_err(|err| format!("Cannot start axum::serve. \nCause:{err}"))?;

    Ok(())
}

