//! main.rs

use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Early exit if we cannot find the configuration.yaml file
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // Get the port from the configuration
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // create a tcp listener, it fails if it's unable to bind the port
    let listener = TcpListener::bind(address)?;

    // the run function is in the lib.rs file
    run(listener, connection_pool)?.await
}
