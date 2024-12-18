use std::time::Duration;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::{middleware, Router};
use axum::routing::get;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::Span;
use crate::handlers::auth::{log_in_handler, post_log_in_handler, post_sign_up_hander};
use crate::handlers::auth::sign_up_handler;
use crate::handlers::public::home;
use crate::handlers::todos::create_handler;
use crate::handlers::todos::todos_handler;
use crate::middleware::{authenticate, required_authentication};
use crate::models::app::AppState;

pub fn routers(app_state: AppState) -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/log-in", get(log_in_handler).post(post_log_in_handler))
        .route("/sign-up", get(sign_up_handler).post(post_sign_up_hander))
        .nest_service("/static", server_dir)
        .merge(protected_routes())
        .layer(middleware::from_fn(authenticate))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
            .on_request(on_request)
            .on_response(on_response)
            .on_failure(on_failure)
        );
    app
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/create", get(create_handler))
        .route("/todos", get(todos_handler))
        .route_layer(middleware::from_fn(required_authentication))
}

fn on_request(request: &Request<Body>, _: &Span) {
 tracing::info!(
     "-> Request started: method: {} path {}",
     request.method(), request.uri().path()
 )
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span){
    tracing::info!(
        "<-Response generated: status {} in {:?}",
        response.status(), latency)
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!(
        "-x- Request failed: error: {:?}, latency: {:?}",
        error, latency)
}