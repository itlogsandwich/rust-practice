use crate::error::Error;
use askama::Template;
use axum::response::{IntoResponse, Response, Html};


#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate
{
    pub current_user: Option<String>,
    pub msg: Option<String>,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate
{
    pub current_user: Option<String>,
    pub acc_num: String,
    pub balance: u64,
    pub msg: Option<String>,
}


#[derive(Template)]
#[template(path = "signup.html")]
pub struct AccFormTemplate
{
    pub current_user: Option<String>,
    pub msg: Option<String>,
}



#[derive(Template)]
#[template(path = "fragments/balance_display.html")]
pub struct BalanceFragment
{
    pub balance: u64,
}

#[derive(Template)]
#[template(path = "fragments/deposit_display.html")]
pub struct DepositFragment
{
    pub acc_num: String,
}

#[derive(Template)]
#[template(path = "fragments/withdraw_display.html")]
pub struct WithdrawFragment
{
    pub acc_num: String,
}
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
    where T: Template,
{
    fn into_response(self) -> Response
    {
        match self.0.render()
        {
            Ok(html) => Html(html).into_response(),
            Err(err) => 
            {
                println!("Error: {err}");
                (Error::InternalServer(err.to_string())).into_response()
            },
        }
    }
}
