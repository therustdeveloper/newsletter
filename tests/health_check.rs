//! tests/health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // here we are requesting to operating system a tcp listener with port 0
    // the operating system receives the 0 value and creates a random port
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    // we this instruction we are capturing the port assigned by operating system
    let port =  listener.local_addr().unwrap().port();

    // create a actix web server with the tcp listener assigned by the operating system
    let server = newsletter::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    // returning the server URL with the assigned port
    format!("http://127.0.0.1:{}", port)
}