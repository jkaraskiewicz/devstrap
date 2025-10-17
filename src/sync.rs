//! Sync command - declarative package and runtime management

use crate::cli::Cli;
use crate::init::{initialize_app, load_system_and_config};
use crate::installation::{run_installation, run_runtime_installation};
use devstrap::common::confirm;
use devstrap::config::StateFile;
use devstrap::domain::{Config, SystemInfo};
use devstrap::usecase::Installer;
use colored::Colorize;
use std::path::Path;
use std::process;

/// Run the sync command
pub fn run_sync(cli: &Cli, prune: bool, refresh: bool) {
    initialize_app(cli);

    let (system_info, config, _project_root) = load_system_and_config(cli);
    let state_path = get_state_path(cli);
    let mut state = StateFile::from_file(&state_path).unwrap_or_default();

    show_dry_run_warning(cli.dry_run);

    let (to_install, to_remove) = calculate_diff(&config, &state);

    if !show_sync_plan(&to_install, &to_remove, prune, cli.dry_run) {
        return;
    }

    if !confirm_sync(cli, prune, &to_install, &to_remove) {
        println!("{}", "Sync cancelled".yellow());
        process::exit(0);
    }

    handle_package_removal(prune, &to_remove, &mut state, cli, &system_info);

    let installer = Installer::new(config.clone(), system_info.clone(), cli.dry_run);
    run_installation(&installer, &config);

    update_state_for_installed(&to_install, &mut state, &system_info, cli.dry_run);
    run_runtime_installation(&config, cli, refresh);

    save_state(&state, &state_path, cli.dry_run);
    show_completion();
}

fn get_state_path(cli: &Cli) -> std::path::PathBuf {
    cli.config
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("devstrap.state")
}

fn show_dry_run_warning(dry_run: bool) {
    if dry_run {
        println!(
            "\n{} {}",
            "⚠".yellow().bold(),
            "DRY RUN MODE - No changes will be made".yellow().bold()
        );
    }
}

fn calculate_diff(config: &Config, state: &StateFile) -> (Vec<String>, Vec<String>) {
    let desired_packages = config.get_all_packages();
    let current_packages = state.get_package_ids();

    let to_install: Vec<String> = desired_packages
        .iter()
        .filter(|pkg| !state.has_package(pkg))
        .cloned()
        .collect();

    let to_remove: Vec<String> = current_packages
        .iter()
        .filter(|pkg| !desired_packages.contains(pkg))
        .cloned()
        .collect();

    (to_install, to_remove)
}

/// Returns true if sync should continue, false if everything is in sync
fn show_sync_plan(
    to_install: &[String],
    to_remove: &[String],
    prune: bool,
    dry_run: bool,
) -> bool {
    if !to_install.is_empty() || (prune && !to_remove.is_empty()) {
        println!("\n{}", "Sync Plan:".bold().cyan());

        if !to_install.is_empty() {
            println!("  {} To install:", "✓".green());
            for pkg in to_install {
                println!("    • {}", pkg.green());
            }
        }

        if prune && !to_remove.is_empty() {
            println!("  {} To remove:", "✗".red());
            for pkg in to_remove {
                println!("    • {}", pkg.red());
            }
        } else if !to_remove.is_empty() {
            println!(
                "  {} Packages not in config (use --prune to remove):",
                "⚠".yellow()
            );
            for pkg in to_remove {
                println!("    • {}", pkg.yellow());
            }
        }
        println!();
        true
    } else {
        println!("\n{} Everything in sync!", "✓".green().bold());
        !dry_run
    }
}

fn confirm_sync(
    cli: &Cli,
    prune: bool,
    to_install: &[String],
    to_remove: &[String],
) -> bool {
    if cli.dry_run || cli.yes {
        return true;
    }

    if !to_install.is_empty() || (prune && !to_remove.is_empty()) {
        confirm("Proceed with sync?")
    } else {
        true
    }
}

fn handle_package_removal(
    prune: bool,
    to_remove: &[String],
    state: &mut StateFile,
    cli: &Cli,
    system_info: &SystemInfo,
) {
    if !prune || to_remove.is_empty() {
        return;
    }

    println!("\n{}", "═".repeat(60).red());
    println!("{}", "REMOVING PACKAGES".bold().red());
    println!("{}", "═".repeat(60).red());

    for pkg_id in to_remove {
        if let Some(record) = state.packages.get(pkg_id) {
            let method_str = record.method.clone();
            println!(
                "  {} Removing {} (installed via {})...",
                "✗".red(),
                pkg_id,
                method_str
            );

            if !cli.dry_run {
                attempt_uninstall(pkg_id, &method_str, system_info, state);
            } else {
                println!("    {} Would remove {}", "[DRY-RUN]".yellow(), pkg_id);
            }
        }
    }
}

fn attempt_uninstall(pkg_id: &str, method_str: &str, system_info: &SystemInfo, state: &mut StateFile) {
    let method = devstrap::domain::InstallMethod::from_string(&method_str.to_lowercase(), system_info);
    if let Some(method) = method {
        if let Err(e) = devstrap::common::uninstall_package(pkg_id, &method) {
            eprintln!("    {} Failed to uninstall {}: {}", "✗".red(), pkg_id, e);
        } else {
            state.remove_package(pkg_id);
        }
    }
}

fn update_state_for_installed(
    to_install: &[String],
    state: &mut StateFile,
    system_info: &SystemInfo,
    dry_run: bool,
) {
    if dry_run {
        return;
    }

    for pkg_id in to_install {
        let method = system_info
            .default_package_manager
            .map(devstrap::domain::InstallMethod::SystemDefault)
            .unwrap_or(devstrap::domain::InstallMethod::System);
        state.add_package(pkg_id.clone(), &method, None);
    }
}

fn save_state(state: &StateFile, state_path: &Path, dry_run: bool) {
    if dry_run {
        return;
    }

    if let Err(e) = state.save(state_path) {
        eprintln!("{} Failed to save state file: {}", "✗".red(), e);
    }
}

fn show_completion() {
    println!("\n{}", "═".repeat(60).green());
    println!("{}", "✓ devstrap sync complete!".green().bold());
    println!("{}", "═".repeat(60).green());
}
