use rust_axum_askama_tuto::init;
use rust_axum_askama_tuto::routes::routers;

#[tokio::main]
async fn main() {

    let addr = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");
    init::logging();

    init::database_connection().await;

    tracing::info!("Server is starting...");
    tracing::info!("Listening at {}", addr);
    tracing::debug!("testing ");

    let app = routers();
    axum::serve(listener, app).await.expect("Failed to start server");

}















