//! CLI definition and argument parsing
//!
//! Defines the command-line interface structure and argument parsing logic.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Universal development environment bootstrapper
#[derive(Parser, Debug)]
#[command(
    name = "devstrap",
    version,
    about = "Universal development environment bootstrapper",
    long_about = None
)]
pub struct Cli {
    /// Path to config.toml file
    #[arg(short, long, default_value = "config.toml", global = true)]
    pub config: PathBuf,

    /// Dry run - show what would be done without making changes
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Skip confirmation prompts (for CI/automated environments)
    #[arg(short, long, global = true)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install packages and runtimes (default behavior)
    Install,

    /// Update installed packages and runtimes to latest versions
    Update {
        /// Optional: Specific package or runtime to update
        target: Option<String>,
    },

    /// List all available packages
    List,
}
