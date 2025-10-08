//! CLI definition and argument parsing
//!
//! Defines the command-line interface structure and argument parsing logic.

use clap::Parser;
use std::path::PathBuf;

/// Universal development environment bootstrapper
#[derive(Parser, Debug)]
#[command(
    name = "devstrap",
    version,
    about = "Universal development environment bootstrapper",
    long_about = None
)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Path to config.toml file
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,

    /// Dry run - show what would be done without making changes
    #[arg(long)]
    pub dry_run: bool,

    /// List all available packages and exit
    #[arg(long)]
    pub list_packages: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Skip confirmation prompts (for CI/automated environments)
    #[arg(short, long)]
    pub yes: bool,
}
