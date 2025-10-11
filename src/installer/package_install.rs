//! Package installation helpers

use super::package_orchestration::{dispatch_installation, print_package_status};
use crate::error::Result;
use crate::package::{InstallMethod, Package};
use crate::utils::uninstall_package;
use colored::Colorize;

/// Check if package needs installation/reinstall
pub(super) fn should_install_package(
    package: &Package,
    default_pm: Option<crate::detect::PackageManager>,
) -> bool {
    if let Some(_current_method) = &package.current_method {
        if package.should_reinstall(default_pm) {
            return true;
        }
        return false;
    }
    true
}

/// Handle package uninstallation if needed
pub(super) fn handle_uninstall(
    package: &Package,
    package_name: &str,
    default_pm: Option<crate::detect::PackageManager>,
    dry_run: bool,
) -> Result<()> {
    if let Some(current_method) = &package.current_method {
        if package.should_reinstall(default_pm)
            && !matches!(current_method, InstallMethod::System)
            && !dry_run
        {
            uninstall_package(package_name, current_method)?;
        }
    }
    Ok(())
}

/// Install a single package
pub(super) fn install_package(
    package: &Package,
    default_pm: Option<crate::detect::PackageManager>,
    dry_run: bool,
) -> Result<()> {
    let package_name = package.package_name().unwrap_or_else(|| package.id.clone());

    let needs_install = should_install_package(package, default_pm);
    print_package_status(package, needs_install);

    if !needs_install {
        return Ok(());
    }

    handle_uninstall(package, &package_name, default_pm, dry_run)?;

    if dry_run {
        println!(
            "    {} Would install {}",
            "[DRY-RUN]".yellow(),
            package_name
        );
        return Ok(());
    }

    execute_installation(&package_name, package)
}

/// Update a single package to latest version
pub(super) fn update_package(
    package: &Package,
    _default_pm: Option<crate::detect::PackageManager>,
    dry_run: bool,
) -> Result<()> {
    let package_name = package.package_name().unwrap_or_else(|| package.id.clone());

    println!("  {} {}", "↻".cyan(), package_name.bold());

    if dry_run {
        println!(
            "    {} Would update {} to latest version",
            "[DRY-RUN]".yellow(),
            package_name
        );
        return Ok(());
    }

    execute_update(&package_name, package)
}

/// Execute the actual package update
fn execute_update(package_name: &str, package: &Package) -> Result<()> {
    println!(
        "    Updating {} via {}...",
        package_name,
        package.preferred_method.display_name()
    );

    match dispatch_installation(package_name, &package.preferred_method, package) {
        Ok(()) => {
            println!(
                "    {} Successfully updated {}",
                "✓".green(),
                package_name
            );
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "    {} Failed to update {}: {}",
                "✗".red(),
                package_name,
                e
            );
            Err(e)
        }
    }
}

/// Execute the actual installation
fn execute_installation(package_name: &str, package: &Package) -> Result<()> {
    println!(
        "    Installing {} via {}...",
        package_name,
        package.preferred_method.display_name()
    );

    match dispatch_installation(package_name, &package.preferred_method, package) {
        Ok(()) => {
            println!(
                "    {} Successfully installed {}",
                "✓".green(),
                package_name
            );
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "    {} Failed to install {}: {}",
                "✗".red(),
                package_name,
                e
            );
            Err(e)
        }
    }
}
