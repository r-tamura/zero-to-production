use std::time::Duration;

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::new(10, 0))
        .build()
        .expect("Failded to build HTTP Client");

    // Act
    let response = client
        // Use the returned application address
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
