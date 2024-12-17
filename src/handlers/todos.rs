use askama::Template;
use axum::Extension;
use axum::response::{Html, IntoResponse, Response};
use crate::handlers::errors::AppError;
use crate::models::app::CurrentUser;
use crate::models::templates::{CreateTemplate, TodosTemplate};

pub(crate) async fn create_handler(Extension(current_user): Extension<CurrentUser>) -> Result<Response, AppError> {
    let html_string = CreateTemplate{
        is_authenticated: current_user.is_authenticated
    }.render()?;
    Ok(Html(html_string).into_response())
}
pub(crate) async fn todos_handler(Extension(current_user): Extension<CurrentUser>) -> Result<Response, AppError> {
    let html_string = TodosTemplate{
        is_authenticated: current_user.is_authenticated
    }.render()?;
    Ok(Html(html_string).into_response())
}