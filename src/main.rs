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
fn run_installation(installer: &Installer, config: &Config) {
    println!(
        "
{}",
        "═".repeat(60).cyan()
    );
    println!("{}", "PACKAGE INSTALLATION".bold().cyan());
    println!("{}", "═".repeat(60).cyan());

    for group in config.get_install_groups() {
        if let Err(e) = installer.install_group(&group) {
            eprintln!("{} Failed to install group {}: {}", "✗".red(), group, e);
        }
    }

    println!(
        "
{}",
        "═".repeat(60).green()
    );
    println!("{}", "✓ Package installation complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}

/// Run runtime and framework installation
fn run_runtime_installation(config: &Config, cli: &Cli) {
    // Load or create lockfile
    let lockfile_path = cli
        .config
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("devstrap.lock");

    let lockfile = Lockfile::from_file(&lockfile_path).unwrap_or_default();

    let mut runtime_manager = RuntimeManager::new(config.clone(), lockfile, cli.dry_run);

    if let Err(e) = runtime_manager.install_all() {
        eprintln!("{} Runtime installation failed: {}", "✗".red(), e);
    } else if let Err(e) = runtime_manager.save_lockfile(&lockfile_path) {
        eprintln!("{} Failed to save lockfile: {}", "✗".red(), e);
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.list_packages {
        list_packages();
        process::exit(0);
    }

    initialize_app(&cli);

    let (system_info, config, _project_root) = load_system_and_config(&cli);

    if cli.dry_run {
        println!(
            "\n{} {}",
            "⚠".yellow().bold(),
            "DRY RUN MODE - No changes will be made".yellow().bold()
        );
    }

    if !cli.dry_run && !cli.yes && !confirm("Proceed with installation?") {
        println!("{}", "Installation cancelled".yellow());
        process::exit(0);
    }

    let installer = Installer::new(config.clone(), system_info.clone(), cli.dry_run);

    // Install packages first
    run_installation(&installer, &config);

    // Then install runtimes and frameworks
    run_runtime_installation(&config, &cli);

    println!(
        "
{}",
        "═".repeat(60).green()
    );
    println!("{}", "✓ devstrap installation complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}
