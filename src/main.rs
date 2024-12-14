use rust_axum_askama_tuto::routes::routers;

#[tokio::main]
async fn main() {

    let port = 8000;
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    let app = routers();

    println!("Server running at http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();

}















