mod cli;
mod commands;
mod config;
mod error;

use clap::Parser;
use cli::Cli;
use commands::run_command;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    run_command(cli.command).await
}

