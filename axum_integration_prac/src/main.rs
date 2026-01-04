use crate::error::Error;
use crate::bank::Bank;
use crate::templates::{ HtmlTemplate, DashboardTemplate, BalanceTemplate };
use axum::routing::{get, post};
use axum::response::{IntoResponse, Html};
use axum::extract::{Path, State, Json};
use axum::Router;
use serde::{ Deserialize, Serialize};
use std::sync::{ Arc, Mutex }; 
use tower_http::services::ServeDir;
mod error;
mod account;
mod bank;
mod templates;

#[derive(Clone)]
struct AppState
{
    bank: Arc<Mutex<Bank>>,
}

#[derive(Deserialize)]
struct TransactionRequest
{
    acc_num: String,
    amount: u64,
}

#[derive(Deserialize)]
struct CreateRequest
{ 
    owner: String,
    pin: String,
}

#[derive(Serialize)]
struct CreateResponse
{ 
    acc_num: String,
    msg: String,
}



async fn create_acc_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
    ) -> impl IntoResponse
{
    println!("--> {:<12} - create_acc_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();

    match bank.create_account(payload.owner, payload.pin)
    {
        Ok(acc) => 
        {
            Json(CreateResponse
            {
                acc_num: acc,
                msg: String::from("Account Successfully Created!"),
            }).into_response()
        },
        Err(e) => format!("Error {:?}", e).into_response(),
    }

}
async fn withdraw_handler(
    State(state): State<AppState>,
    Json(payload): Json<TransactionRequest>,
    ) -> Result<impl IntoResponse, Error>
{
    println!("--> {:<12} - withdraw_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();
    
    bank.withdraw(&payload.acc_num, payload.amount)?;

    Ok("Withdrawn Successfully!")
}

async fn deposit_handler(
    State(state): State<AppState>,
    Json(payload): Json<TransactionRequest>,
    ) -> Result<impl IntoResponse, Error>
{
    
    println!("--> {:<12} - deposit_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();

    bank.deposit(&payload.acc_num, payload.amount)?;

    Ok("Deposit Successfully")
}

async fn balance(
    State(state): State<AppState>,
    Path(acc_num): Path<String>,
    ) -> Result<impl IntoResponse, Error>
{
    println!("--> {:<12} - balance - ", "HANDLER");

    let bank = state.bank.lock().unwrap();

    let balance = bank.check_balance(&acc_num)?;

    let template = BalanceTemplate
    {
        balance
    };

    Ok(HtmlTemplate(template))
}

async fn dashboard() -> impl IntoResponse
{ 
    println!("--> {:<12} - dashboard - ", "HANDLER");
    
    let template = DashboardTemplate{};
    HtmlTemplate(template)
}

fn create_app() -> Router
{
    let shared_state = AppState
    {
        bank: Arc::new(Mutex::new(Bank::new())),
    };

    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(dashboard))
        .route("/balance/{acc_num}", get(balance))
        .route("/deposit", post(deposit_handler))
        .route("/withdraw", post(withdraw_handler))
        .route("/create", post(create_acc_handler))
        .with_state(shared_state)
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
