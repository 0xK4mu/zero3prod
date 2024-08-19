use std::{future::IntoFuture, time::Duration};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("try request");
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    tokio::spawn(zero3prod::run().into_future());
}
