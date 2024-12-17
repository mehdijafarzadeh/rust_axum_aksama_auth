use askama::Template;
use axum::Extension;
use axum::response::{Html, IntoResponse, Response};
use crate::handlers::errors::AppError;
use crate::models::app::CurrentUser;
use crate::models::templates::HomeTemplate;

pub(crate) async fn home(Extension(current_user): Extension<CurrentUser> ) -> Result<Response, AppError> {
    let html_string = HomeTemplate{
        is_authenticated: current_user.is_authenticated
    }.render()?;
    Ok(Html(html_string).into_response())
}
