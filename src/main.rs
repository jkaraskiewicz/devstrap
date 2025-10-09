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

/// Update packages to latest versions
fn update_packages(installer: &Installer, config: &Config, target: Option<&str>) {
    println!(
        "
{}",
        "═".repeat(60).cyan()
    );
    println!("{}", "PACKAGE UPDATE".bold().cyan());
    println!("{}", "═".repeat(60).cyan());

    if let Some(package_name) = target {
        // Update specific package
        if let Err(e) = installer.update_package(package_name) {
            eprintln!("{} Failed to update package {}: {}", "✗".red(), package_name, e);
        }
    } else {
        // Update all packages
        for group in config.get_install_groups() {
            if let Err(e) = installer.update_group(&group) {
                eprintln!("{} Failed to update group {}: {}", "✗".red(), group, e);
            }
        }
    }

    println!(
        "
{}",
        "═".repeat(60).green()
    );
    println!("{}", "✓ Package update complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}

/// Update runtimes to latest versions
fn update_runtimes(config: &Config, cli: &Cli, target: Option<&str>) {
    // Load or create lockfile
    let lockfile_path = cli
        .config
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("devstrap.lock");

    // Delete lockfile to force re-resolution of versions
    if !cli.dry_run {
        if let Some(runtime_name) = target {
            // If updating specific runtime, we'd need to modify the lockfile
            // For now, we'll delete it entirely
            println!("  {} Clearing version lock for {}...", "↻".cyan(), runtime_name);
        } else {
            println!("  {} Clearing all version locks...", "↻".cyan());
        }
        let _ = std::fs::remove_file(&lockfile_path);
    }

    let lockfile = Lockfile::default();
    let mut runtime_manager = RuntimeManager::new(config.clone(), lockfile, cli.dry_run);

    if let Err(e) = runtime_manager.install_all() {
        eprintln!("{} Runtime update failed: {}", "✗".red(), e);
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
        Some(cli::Commands::Update { target }) => {
            run_update(&cli, target.as_deref());
        }
        Some(cli::Commands::Install) | None => {
            // Default behavior: install
            run_install(&cli);
        }
    }
}

/// Run the update command
fn run_update(cli: &Cli, target: Option<&str>) {
    initialize_app(cli);

    let (system_info, config, _project_root) = load_system_and_config(cli);

    if cli.dry_run {
        println!(
            "\n{} {}",
            "⚠".yellow().bold(),
            "DRY RUN MODE - No changes will be made".yellow().bold()
        );
    }

    let update_msg = if let Some(pkg) = target {
        format!("Update {} to latest version?", pkg)
    } else {
        "Update all packages and runtimes to latest versions?".to_string()
    };

    if !cli.dry_run && !cli.yes && !confirm(&update_msg) {
        println!("{}", "Update cancelled".yellow());
        process::exit(0);
    }

    println!(
        "
{}",
        "═".repeat(60).cyan()
    );
    println!("{}", "UPDATE MODE".bold().cyan());
    println!("{}", "═".repeat(60).cyan());

    let installer = Installer::new(config.clone(), system_info.clone(), cli.dry_run);

    // Update packages
    update_packages(&installer, &config, target);

    // Update runtimes
    update_runtimes(&config, cli, target);

    println!(
        "
{}",
        "═".repeat(60).green()
    );
    println!("{}", "✓ Update complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}

/// Run the install command
fn run_install(cli: &Cli) {
    initialize_app(cli);

    let (system_info, config, _project_root) = load_system_and_config(cli);

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
    run_runtime_installation(&config, cli);

    println!(
        "
{}",
        "═".repeat(60).green()
    );
    println!("{}", "✓ devstrap installation complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}
