use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{LoginTemplate, SignUpTemplate};

pub async fn log_in_handler() -> Response {
    let html_string = LoginTemplate{}.render().unwrap();
    Html(html_string).into_response()
}
pub async fn sign_up_handler() -> Response {
    let html_string = SignUpTemplate{}.render().unwrap();
    Html(html_string).into_response()
}