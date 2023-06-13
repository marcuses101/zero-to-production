use reqwest;

#[tokio::test]
async fn health_check_works(){
    spawn_app();
    // Arrange
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

// Launch our application in the background somehow
fn spawn_app()   {
 let server = zero_to_production::run().expect("Failed to bind address");
 let _ = tokio::spawn(server);
}
