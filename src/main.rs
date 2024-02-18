//! main.rs

use newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // create a tcp listener, it fails if it's unable to bind the port
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind port");

    // the run function is in the lib.rs file
    run(listener)?.await
}
