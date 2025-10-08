//! Package uninstallation utilities
//!
//! Functions for removing packages via different package managers.

use super::command::run_command;
use crate::detect::PackageManager;
use crate::error::Result;
use crate::package::InstallMethod;
use anyhow::{anyhow, Context};

/// Uninstall a package using the specified method
///
/// # Arguments
/// * `package_name` - Name of the package
/// * `method` - Installation method to use for uninstallation
///
/// # Returns
/// Result indicating success or failure
pub fn uninstall_package(package_name: &str, method: &InstallMethod) -> Result<()> {
    match method {
        InstallMethod::Cargo => {
            run_command("cargo", &["uninstall", package_name])
                .with_context(|| format!("Failed to uninstall {package_name} via Cargo"))?;
        }
        InstallMethod::SystemDefault(pm) => match pm {
            PackageManager::Brew => {
                run_command("brew", &["uninstall", package_name]).ok();
            }
            PackageManager::Apt => {
                run_command("sudo", &["apt-get", "remove", "-y", package_name]).ok();
            }
            PackageManager::Pacman => {
                run_command("sudo", &["pacman", "-R", "--noconfirm", package_name]).ok();
            }
            PackageManager::Dnf => {
                run_command("sudo", &["dnf", "remove", "-y", package_name]).ok();
            }
            PackageManager::Yum => {
                run_command("sudo", &["yum", "remove", "-y", package_name]).ok();
            }
            _ => {}
        },
        InstallMethod::Npm => {
            run_command("npm", &["uninstall", "-g", package_name]).ok();
        }
        InstallMethod::Pipx => {
            run_command("pipx", &["uninstall", package_name]).ok();
        }
        InstallMethod::System | InstallMethod::GitHub => {
            return Err(anyhow!(
                "Cannot uninstall system packages: {} ({})",
                package_name,
                method.display_name()
            ));
        }
    }

    Ok(())
}
