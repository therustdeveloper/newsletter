//! src/lib.rs
pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
pub mod logger;

use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(flatten)]
    pub(crate) instrumentation: telemetry::Instrumentation,
}
