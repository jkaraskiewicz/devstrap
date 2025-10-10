//! devstrap CLI entry point
//!
//! Command-line interface for the devstrap development environment bootstrapper.

mod cli;
mod init;

use clap::Parser;
use cli::Cli;
use colored::Colorize;
use devstrap::runtime::RuntimeManager;
use devstrap::{builtin, confirm, Config, Installer, Lockfile};
use init::{initialize_app, load_system_and_config};
use std::process;

/// Display all available packages with descriptions
fn list_packages() {
    println!("{}", "Available packages:".bold().cyan());
    println!("{}", "═".repeat(60).cyan());

    let package_ids = builtin::get_all_package_ids();

    for package_id in package_ids {
        if let Some(pkg) = builtin::get_package(package_id) {
            let desc = pkg
                .description
                .as_deref()
                .unwrap_or("No description available");
            println!("  • {} - {}", package_id.green().bold(), desc.dimmed());
        }
    }

    println!("\n{}", "Usage in config.toml:".bold());
    println!("  [packages]");
    println!("  base = [\"git\", \"ripgrep\", \"bat\"]");
    println!("  dev_tools = [\"fzf\", \"fd\"]");
}

/// Run the package installation process
fn run_installation(installer: &Installer, _config: &Config) {
    println!(
        "
{}",
        "═".repeat(60).cyan()
    );
    println!("{}", "PACKAGE INSTALLATION".bold().cyan());
    println!("{}", "═".repeat(60).cyan());

    if let Err(e) = installer.install_all() {
        eprintln!("{} Package installation failed: {}", "✗".red(), e);
    }

    println!(
        "
{}",
        "═".repeat(60).green()
    );
    println!("{}", "✓ Package installation complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}

/// Run runtime installation
fn run_runtime_installation(config: &Config, cli: &Cli, refresh: bool) {
    // Load or create lockfile
    let lockfile_path = cli
        .config
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("devstrap.lock");

    // If refresh flag is set, clear lockfile to force re-resolution
    let lockfile = if refresh {
        if !cli.dry_run {
            println!("  {} Refreshing version locks...", "↻".cyan());
            let _ = std::fs::remove_file(&lockfile_path);
        }
        Lockfile::default()
    } else {
        Lockfile::from_file(&lockfile_path).unwrap_or_default()
    };

    let mut runtime_manager = RuntimeManager::new(config.clone(), lockfile, cli.dry_run);

    if let Err(e) = runtime_manager.install_all() {
        eprintln!("{} Runtime installation failed: {}", "✗".red(), e);
    } else if let Err(e) = runtime_manager.save_lockfile(&lockfile_path) {
        eprintln!("{} Failed to save lockfile: {}", "✗".red(), e);
    }
}



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

/// Run the sync command
fn run_sync(cli: &Cli, _prune: bool, refresh: bool) {
    initialize_app(cli);

    let (system_info, config, _project_root) = load_system_and_config(cli);

    if cli.dry_run {
        println!(
            "\n{} {}",
            "⚠".yellow().bold(),
            "DRY RUN MODE - No changes will be made".yellow().bold()
        );
    }

    if !cli.dry_run && !cli.yes && !confirm("Proceed with sync?") {
        println!("{}", "Sync cancelled".yellow());
        process::exit(0);
    }

    let installer = Installer::new(config.clone(), system_info.clone(), cli.dry_run);

    // Install packages first
    run_installation(&installer, &config);

    // Then install runtimes
    run_runtime_installation(&config, cli, refresh);

    println!(
        "
{}",
        "═".repeat(60).green()
    );
    println!("{}", "✓ devstrap sync complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}
