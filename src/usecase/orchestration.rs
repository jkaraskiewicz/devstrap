//! Installation helper functions
//!
//! Utility functions for package status checking and installation dispatch.

use crate::service::package_manager::methods::{
    install_from_github, install_with_cargo, install_with_npm, install_with_pipx,
    install_with_system_package_manager,
};
use crate::builtin;
use crate::domain::SystemInfo;
use crate::common::error::Result;
use crate::domain::{determine_best_method, InstallMethod, Package};
use crate::common::detect_installation_method;
use colored::Colorize;

/// Prepare packages for installation
#[must_use]
pub fn prepare_packages(package_ids: &[String], system_info: &SystemInfo) -> Vec<Package> {
    let mut packages = Vec::new();

    for package_id in package_ids {
        if let Some(package_config) = builtin::get_package(package_id) {
            if let Some(method) = determine_best_method(package_config, system_info) {
                let mut package = Package::new(package_id.clone(), package_config.clone(), method);

                package.current_method = detect_installation_method(package_id, system_info);

                packages.push(package);
            }
        }
    }

    packages
}

/// Report installation errors
pub fn report_errors(errors: Vec<(String, anyhow::Error)>) {
    if !errors.is_empty() {
        println!("\n{}", "Errors during installation:".red().bold());
        for (pkg, err) in errors {
            eprintln!("  {} {}: {}", "✗".red(), pkg, err);
        }
    }
}

/// Dispatch installation to appropriate method
pub fn dispatch_installation(
    package_name: &str,
    method: &InstallMethod,
    package: &Package,
) -> Result<()> {
    match method {
        InstallMethod::SystemDefault(pm) => {
            install_with_system_package_manager(package_name, *pm)?;
        }
        InstallMethod::Cargo => install_with_cargo(package_name)?,
        InstallMethod::Npm => install_with_npm(package_name)?,
        InstallMethod::Pipx => install_with_pipx(package_name)?,
        InstallMethod::GitHub => {
            install_from_github(package_name, package);
        }
        InstallMethod::System => {
            // Already installed via system
        }
    }
    Ok(())
}

/// Print package installation status
pub fn print_package_status(package: &Package, needs_install: bool) {
    if let Some(current_method) = &package.current_method {
        if needs_install {
            if matches!(current_method, InstallMethod::System) {
                println!(
                    "  {} {} (installing via {} alongside system version)",
                    "↻".yellow(),
                    package.id.bold(),
                    package.preferred_method.display_name().green()
                );
            } else {
                println!(
                    "  {} {} (currently via {}, preferring {})",
                    "↻".yellow(),
                    package.id.bold(),
                    current_method.display_name().yellow(),
                    package.preferred_method.display_name().green()
                );
            }
        } else {
            println!(
                "  {} {} (via {})",
                "✓".green(),
                package.id.dimmed(),
                current_method.display_name().dimmed()
            );
        }
    } else {
        println!(
            "  {} {} (via {})",
            "↓".cyan(),
            package.id.bold(),
            package.preferred_method.display_name().cyan()
        );
    }
}
