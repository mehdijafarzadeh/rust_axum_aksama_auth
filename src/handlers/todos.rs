use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use crate::models::templates::{CreateTemplate, TodosTemplate};

pub(crate) async fn create_handler() -> Response {
    let html_string = CreateTemplate{}.render().unwrap();
    Html(html_string).into_response()
}
pub(crate) async fn todos_handler() -> Response {
    let html_string = TodosTemplate{}.render().unwrap();
    Html(html_string).into_response()
}