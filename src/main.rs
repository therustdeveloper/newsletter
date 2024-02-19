//! main.rs

use newsletter::startup::run;
use std::net::TcpListener;
use newsletter::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Early exit if we cannot find the configuration.yaml file
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Get the port from the configuration
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // create a tcp listener, it fails if it's unable to bind the port
    let listener = TcpListener::bind(address)?;

    // the run function is in the lib.rs file
    run(listener)?.await
}
