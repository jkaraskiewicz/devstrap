//! devstrap CLI entry point
//!
//! Command-line interface for the devstrap development environment bootstrapper.

mod cli;
mod init;
mod installation;
mod sync;

use clap::Parser;
use cli::Cli;
use devstrap::usecase::list_packages;
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
            sync::run_sync(&cli, *prune, *refresh);
        }
        None => {
            // No command specified - show help
            Cli::parse_from(&["devstrap", "--help"]);
        }
    }
}
