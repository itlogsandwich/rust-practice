use askama::Template;
use axum::response::{IntoResponse, Response, Html};

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate;


#[derive(Template)]
#[template(path = "signup.html")]
pub struct AccFormTemplate;

#[derive(Template)]
#[template(path = "balance.html")]
pub struct BalanceTemplate
{
    pub balance: u64
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
            Err(err) => println!("Error: {err}").into_response(),
        }
    }
}
