

use tokio::net::TcpListener;

use axum::{http::StatusCode, routing::get, Router};


async fn health_check() -> StatusCode{
    StatusCode::OK
}

pub async fn run() -> Result<(), std::io::Error> {
    println!("ceate app");
    let app = Router::new()
        .route("/health_check", get(health_check));

    println!("ceate listener");
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    println!("server ok");
    axum::serve(listener, app).await
}