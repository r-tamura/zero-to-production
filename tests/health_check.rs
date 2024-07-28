use std::{net::TcpListener, time::Duration};

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // Linuxポート0を指定するとOSの空きポートを自動で選択する
    let server = zero2prod::run(listener).expect("Failed to spawn our app.");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::new(10, 0))
        .build()
        .expect("Failded to build HTTP Client");

    // Act
    let response = client
        // Use the returned application address
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
