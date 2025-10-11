//! Package and runtime installation command logic

use crate::cli::Cli;
use colored::Colorize;
use devstrap::runtime::RuntimeManager;
use devstrap::{Config, Installer, Lockfile};
use std::path::Path;

/// Run the package installation process
pub fn run_installation(installer: &Installer, _config: &Config) {
    println!("\n{}", "═".repeat(60).cyan());
    println!("{}", "PACKAGE INSTALLATION".bold().cyan());
    println!("{}", "═".repeat(60).cyan());

    if let Err(e) = installer.install_all() {
        eprintln!("{} Package installation failed: {}", "✗".red(), e);
    }

    println!("\n{}", "═".repeat(60).green());
    println!("{}", "✓ Package installation complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}

/// Run runtime installation
pub fn run_runtime_installation(config: &Config, cli: &Cli, refresh: bool) {
    let lockfile_path = cli
        .config
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("devstrap.lock");

    let lockfile = if refresh {
        handle_refresh(&lockfile_path, cli.dry_run);
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

fn handle_refresh(lockfile_path: &Path, dry_run: bool) {
    if !dry_run {
        println!("  {} Refreshing version locks...", "↻".cyan());
        let _ = std::fs::remove_file(lockfile_path);
    }
}
