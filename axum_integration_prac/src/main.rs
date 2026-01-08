use crate::error::Error;
use crate::bank::Bank;
use crate::templates::{ HtmlTemplate, HomeTemplate, DashboardTemplate, AccFormTemplate, BalanceFragment, DepositFragment, WithdrawFragment};
use axum::routing::{get, post};
use axum::response::{IntoResponse, Redirect, Response};
use axum::extract::{Path, State, Query};
use axum::{Router, Form};
use axum::http::HeaderMap;
use serde::{ Deserialize};
use std::sync::{ Arc, Mutex }; 
use tower_http::services::ServeDir;
mod error;
mod account;
mod bank;
mod templates;


type HandlerResult<T> = Result<T, Error>;

#[derive(Clone)]
struct AppState
{
    bank: Arc<Mutex<Bank>>,
}

#[derive(Deserialize)]
struct TransactionRequest
{
    amount: u64,
}

#[derive(Deserialize)]
struct MessageParams
{
    msg: Option<String>
}

#[derive(Deserialize)]
struct CreateRequest
{ 
    owner: String,
    pin: String,
    confirm_pin: String,
}

// #[derive(Serialize)]
// struct CreateResponse
// { 
//     acc_num: String,
//     msg: String,
// }


async fn show_acc_form_handler() -> impl IntoResponse
{

    println!("--> {:<12} - show_acc_form - ", "HANDLER");
    
    let template = AccFormTemplate
    {
        current_user: None,
        msg: None, 
    };
    HtmlTemplate(template)
}

async fn create_acc_handler(
    State(state): State<AppState>,
    Form(payload): Form<CreateRequest>,
    ) -> HandlerResult<Redirect>
{
    println!("--> {:<12} - create_acc_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();

    let acc_num = bank.create_account(payload.owner, payload.pin, payload.confirm_pin)?;
    let target = &format!("/dashboard/{}?msg=Account+Sucessfully+Created", &acc_num);

    Ok(Redirect::to(target))

}

async fn withdraw_form_handler(
    Path(acc_num): Path<String>,
    ) -> impl IntoResponse
{
    let template = WithdrawFragment { acc_num };

    HtmlTemplate(template)
}

async fn withdraw_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(acc_num): Path<String>,
    Form(payload): Form<TransactionRequest>,
    ) -> HandlerResult<impl IntoResponse>
{
    println!("--> {:<12} - withdraw_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();
    
    bank.withdraw(&acc_num, payload.amount)?;

    let is_htmx = headers.contains_key("hx-request");

    if is_htmx
    {

        let body_contents = format!(r#"
        <div class = "withdraw-success-container"
            hx-get = "/fragments/withdraw/{acc_num}"
            hx-trigger = "load delay:5s"
            hx-swap = "outerHTML"
            >
            <h1>
                Successfully Withdrawn ${amount}!
                This message will disappear in 5 seconds...
            </h1>
        </div>"#,
        
            amount = payload.amount,
        );
        Ok(
        Response::builder()
            .header("HX-TRIGGER","transaction_complete")
            .body(body_contents)?
            .into_response()
        )
    }
    else
    {
        let target = &format!("/balance/{}", &acc_num);
        Ok(Redirect::to(target).into_response())
    }
}

async fn deposit_form_handler(
    Path(acc_num): Path<String>,
    ) -> impl IntoResponse
{
    let template = DepositFragment { acc_num };

    HtmlTemplate(template)
}
async fn deposit_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(acc_num): Path<String>,
    Form(payload): Form<TransactionRequest>,
    ) -> HandlerResult<impl IntoResponse>
{
    
    println!("--> {:<12} - deposit_handler - ", "HANDLER");

    let mut bank = state.bank.lock().unwrap();
    bank.deposit(&acc_num, payload.amount)?;

    let is_htmx = headers.contains_key("hx-request");

    if is_htmx
    {
        let body_contents = format!(r#"
        <div class = "deposit-success-container"
            hx-get = "/fragments/deposit/{acc_num}"
            hx-trigger = "load delay:5s"
            hx-swap = "outerHTML"
            >
            <h1>
                Successfully Deposited ${amount}!
                This message will disappear in 5 seconds...
            </h1>
        </div>"#,

            amount = payload.amount,
        );
        Ok(
        Response::builder()
            .header("HX-Trigger","transaction_complete")
            .body(body_contents)?
            .into_response()
        )
    }
    else
    {
        let target = &format!("/balance/{acc_num}");
        Ok(Redirect::to(target).into_response())
    }

}

async fn balance(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(acc_num): Path<String>,
    Query(params): Query<MessageParams>,
    ) -> HandlerResult<impl IntoResponse>
{
    println!("--> {:<12} - balance - ", "HANDLER");

    let bank = state.bank.lock().unwrap();

    let balance = bank.check_balance(&acc_num)?;

    let is_htmx = headers.contains_key("hx-request");

    if is_htmx
    {
        let template = BalanceFragment
        {
            balance
        };
   
        Ok(HtmlTemplate(template).into_response())
    }
    else
    {
        let template = DashboardTemplate
        {
            current_user: Some(acc_num.clone()),
            acc_num,
            balance,
            msg: params.msg,
        };

        Ok(HtmlTemplate(template).into_response())
    }
}

async fn home() -> impl IntoResponse
{
    println!("--> {:<12} - home - ", "HANDLER");

    let template = HomeTemplate 
    {
        current_user: None,
        msg: None, 
    };

    HtmlTemplate(template)
}
async fn dashboard(
    Path(acc_num): Path<String>,
    State(state): State<AppState>,
    ) -> HandlerResult<impl IntoResponse>
{ 
    println!("--> {:<12} - dashboard - ", "HANDLER");
   
    let bank = state.bank.lock().unwrap();
    
    match bank.check_balance(&acc_num)
    {
        Ok(balance) =>
        {        
            let template = DashboardTemplate 
            {
                current_user: Some(acc_num.clone()),
                acc_num,
                balance,
                msg: None,
            };
            Ok(HtmlTemplate(template).into_response())
        }
        Err(_) =>
        {
            Ok(Redirect::to("/create").into_response())
        }
    }

}

fn create_app() -> Router
{
    let shared_state = AppState
    {
        bank: Arc::new(Mutex::new(Bank::new())),
    };

    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(home))
        .route("/dashboard/{acc_num}", get(dashboard))
        .route("/fragments/balance/{acc_num}", get(balance))
        .route("/fragments/deposit/{acc_num}", get(deposit_form_handler))
        .route("/fragments/withdraw/{acc_num}", get(withdraw_form_handler))
        .route("/deposit/{acc_num}", post(deposit_handler))
        .route("/withdraw/{acc_num}", post(withdraw_handler))
        .route("/create", get(show_acc_form_handler).post(create_acc_handler))
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
