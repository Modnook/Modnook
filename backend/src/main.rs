use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::config::commands::Commands;

pub mod api;
pub mod cache;
pub mod config;
pub mod database;
pub mod error;
pub mod repositories;
pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .init();

    let cli = config::commands::Cli::parse();

    match cli.command {
        Commands::Run(server_options) => crate::api::run_server(server_options).await,
    }
}
