use crate::error::Error;
use crate::bank::Bank;

use axum::routing::{get, post};
use axum::response::{IntoResponse, Html};
use axum::extract::{Path, State, Json};
use axum::Router;
use serde::{ Deserialize};
use std::sync::{ Arc, Mutex }; 

mod error;
mod account;
mod bank;

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

async fn create_acc_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
    ) -> impl IntoResponse
{
    println!("--> {:<12} - create_acc_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();

    match bank.create_account(payload.owner, payload.pin)
    {
        Ok(acc) => acc.into_response(),
        Err(e) => format!("Error {:?}", e).into_response(),
    }

}
async fn withdraw_handler(
    State(state): State<AppState>,
    Json(payload): Json<TransactionRequest>,
    ) -> impl IntoResponse
{
    println!("--> {:<12} - withdraw_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();
    
    match bank.withdraw(&payload.acc_num, payload.amount)
    {
        Ok(_) => "Withdraw Successful".into_response(),
        Err(e) => format!("Error {:?}", e).into_response(),
    }
}

async fn deposit_handler(
    State(state): State<AppState>,
    Json(payload): Json<TransactionRequest>,
    ) -> impl IntoResponse
{
    
    println!("--> {:<12} - deposit_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();

    match bank.deposit(&payload.acc_num, payload.amount)
    {
        Ok(_) => "Deposit Successful".into_response(),
        Err(e) => format!("Error {:?}", e).into_response(),
    }
}

async fn balance(
    State(state): State<AppState>,
    Path(acc_num): Path<String>,
    ) -> impl IntoResponse
{
    println!("--> {:<12} - balance - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();

    match bank.check_balance(&acc_num)
    {
        Ok(bal) => format!("Balance: ${bal}").into_response(),
        Err(e) => format!("Error {:?}", e).into_response(),
    }
}

async fn hello_world() -> impl IntoResponse
{ 
    println!("--> {:<12} - hello_world - ", "HANDLER");

    Html(format!("Hello, World!"))
}

fn create_app() -> Router
{
    let shared_state = AppState
    {
        bank: Arc::new(Mutex::new(Bank::new())),
    };

    Router::new()
        .route("/", get(hello_world))
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
