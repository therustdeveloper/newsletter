//! tests/health_check.rs

use std::net::TcpListener;

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

    // returning the app address with port number
    format!("http://127.0.0.1:{}", port)
}

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // get the app address with port number
    let app_address = spawn_app();

    // use reqwest crate to interact with the application
    let client = reqwest::Client::new();

    // this body corresponds to the information send by a HTML form
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // here we are sending the request to subscriptions in the expected format
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // validate if the response code is OK
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // get the app address with port number
    let app_address = spawn_app();

    // use reqwest crate to interact with the application
    let client = reqwest::Client::new();

    // prepare the test cases
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    // send a request for each test_cases values
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // the expected result for each test cases is a 400 response code
        assert_eq!(
            400,
            response.status().as_u16(),
            // additional customized error message on test failure
            "The API did not fail with 400 bad request when the payload was {}.",
            error_message
        );
    }

}