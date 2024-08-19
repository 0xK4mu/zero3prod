use tokio::net::TcpListener;


async fn spawn_app() {
    let app = zero3prod::app();
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await });
}


#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await;
    //tokio::time::sleep(Duration::from_secs(1)).await;
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

