//! Package-specific installation methods
//!
//! Contains the individual package installation logic for each supported method.

use crate::detect::PackageManager;
use crate::error::Result;
use crate::package::Package;
use crate::utils::run_command;
use anyhow::Context;
use colored::Colorize;

/// Install a package using system package manager
pub(super) fn install_with_system_package_manager(
    package_name: &str,
    pm: PackageManager,
) -> Result<()> {
    let (cmd, args) = match pm {
        PackageManager::Brew => ("brew", vec!["install", package_name]),
        PackageManager::Apt => ("sudo", vec!["apt-get", "install", "-y", package_name]),
        PackageManager::Pacman => (
            "sudo",
            vec!["pacman", "-S", "--noconfirm", "--needed", package_name],
        ),
        PackageManager::Dnf => ("sudo", vec!["dnf", "install", "-y", package_name]),
        PackageManager::Yum => ("sudo", vec!["yum", "install", "-y", package_name]),
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported package manager {} for package {}",
                pm.display_name(),
                package_name
            ));
        }
    };

    run_command(cmd, &args).with_context(|| {
        format!(
            "Failed to install {} via {}",
            package_name,
            pm.display_name()
        )
    })?;

    Ok(())
}

/// Install a package using Cargo
pub(super) fn install_with_cargo(package_name: &str) -> Result<()> {
    run_command("cargo", &["install", package_name])
        .with_context(|| format!("Failed to install {package_name} via Cargo"))?;
    Ok(())
}

/// Install a package using npm
pub(super) fn install_with_npm(package_name: &str) -> Result<()> {
    run_command("npm", &["install", "-g", package_name])
        .with_context(|| format!("Failed to install {package_name} via npm"))?;
    Ok(())
}

/// Install a package using pipx
pub(super) fn install_with_pipx(package_name: &str) -> Result<()> {
    run_command("pipx", &["install", package_name])
        .with_context(|| format!("Failed to install {package_name} via pipx"))?;
    Ok(())
}

/// Install a package from GitHub releases
pub(super) fn install_from_github(package_name: &str, _package: &Package) {
    // For now, just report that GitHub installation would happen
    // Full GitHub release installation would require:
    // 1. Fetch latest release from GitHub API
    // 2. Download appropriate binary for architecture
    // 3. Extract and install to ~/.local/bin or /usr/local/bin

    println!(
        "    {} GitHub installation for {} (would download from releases)",
        "ℹ".blue(),
        package_name
    );
}
