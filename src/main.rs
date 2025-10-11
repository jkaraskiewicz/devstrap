//! devstrap CLI entry point
//!
//! Command-line interface for the devstrap development environment bootstrapper.

mod cli;
mod commands;
mod init;

use clap::Parser;
use cli::Cli;
use commands::{list_packages, run_sync};
use std::process;

fn main() {
    let cli = Cli::parse();

    // Handle commands
    match &cli.command {
        Some(cli::Commands::List) => {
            list_packages();
            process::exit(0);
        }
        Some(cli::Commands::Sync { prune, refresh }) => {
            run_sync(&cli, *prune, *refresh);
        }
        None => {
            // Default behavior: sync
            run_sync(&cli, false, false);
        }
    }
}
