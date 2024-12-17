use askama::{Template};
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use thiserror::Error;
use crate::data::errors::DataError;
use crate::models::templates::ServerErrorTemplate;

#[derive(Debug, Error)]
pub enum AppError{
    #[error("Database error")]
    Database(#[from] DataError),
    #[error("Template error")]
    Template(#[from] askama::Error),
    #[error("Failed loading session")]
    Session(#[from] tower_sessions::session::Error),

}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, response) = match self {
            AppError::Database(e) => server_error(e.to_string()),
            AppError::Template(e) => server_error(e.to_string()),
            AppError::Session(e) => server_error(e.to_string()),
        };
        (status, response).into_response()
    }
}

fn server_error(e: String)-> (StatusCode, Response<Body>) {
    tracing::error!("Server error: {}", e);

    let html_string = ServerErrorTemplate{ is_authenticated: false}.render().unwrap();

    // match html_string{
    //     Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, html.into_response()),
    //     Err(e) => {
    //         tracing::error!("Error rendering server error template: {}", e.to_string());
    //         (StatusCode::INTERNAL_SERVER_ERROR,
    //          "Error rendering server error template, please contact me".into_response())
    //     },
    // }

    (StatusCode::INTERNAL_SERVER_ERROR, Html(html_string).into_response())
}