//! Application initialization
//!
//! Handles application startup including logging setup and system detection.

use crate::cli::Cli;
use colored::Colorize;
use devstrap::common::{print_system_info, show_banner};
use devstrap::domain::{Config, SystemInfo};
use std::path::PathBuf;
use std::process;

/// Initialize application (logging and banner)
pub fn initialize_app(cli: &Cli) {
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    devstrap::init_logging();
    show_banner();
}

/// Load system information and configuration
///
/// # Arguments
/// * `cli` - CLI arguments
///
/// # Returns
/// Tuple of (`SystemInfo`, `Config`, `project_root`)
pub fn load_system_and_config(cli: &Cli) -> (SystemInfo, Config, PathBuf) {
    let system_info = match SystemInfo::detect() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("{} Failed to detect system information: {}", "✗".red(), e);
            process::exit(1);
        }
    };

    print_system_info(&system_info);

    let config = match Config::from_file(&cli.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!(
                "{} Failed to load configuration from {}: {}",
                "✗".red(),
                cli.config.display(),
                e
            );
            process::exit(1);
        }
    };

    let project_root = cli.config.parent().unwrap_or(&cli.config).to_path_buf();

    println!(
        "\n{} Configuration loaded from {}",
        "✓".green(),
        cli.config.display()
    );

    (system_info, config, project_root)
}
