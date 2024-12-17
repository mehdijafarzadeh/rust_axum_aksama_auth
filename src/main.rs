use rust_axum_askama_tuto::init;
use rust_axum_askama_tuto::models::app::AppState;
use rust_axum_askama_tuto::routes::routers;

#[tokio::main]
async fn main() {

    let addr = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");
    init::logging();

    let pg_pool = init::database_connection().await;

    let session_layer = init::session(pg_pool.clone()).await;

    let app_state = AppState {
        connection_pool: pg_pool
    };

    tracing::info!("Server is starting...");
    tracing::info!("Listening at {}", addr);
    tracing::debug!("testing ");

    let app = routers(app_state).layer(session_layer);
    axum::serve(listener, app).await.expect("Failed to start server");

}















