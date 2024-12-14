use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

use crate::handlers::auth::log_in_handler;
use crate::handlers::auth::sign_up_handler;
use crate::handlers::public::home_handler;
use crate::handlers::todos::create_handler;
use crate::handlers::todos::todos_handler;

pub fn routers() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/create", get(create_handler))
        .route("/login", get(log_in_handler))
        .route("/sign-up", get(sign_up_handler))
        .route("/todos", get(todos_handler))
        .nest_service("/static", server_dir);
    app
}