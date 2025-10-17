//! Package installation detection utilities
//!
//! Functions for detecting how packages are currently installed.

use super::command::run_command_output;
use crate::domain::{PackageManager, SystemInfo};
use crate::domain::system::is_command_available;
use crate::domain::InstallMethod;

/// Detect installation method from executable path
fn detect_from_path(package_name: &str) -> Option<InstallMethod> {
    let path = which::which(package_name).ok()?;
    let path_str = path.to_string_lossy();

    if path_str.contains("/.cargo/bin/") {
        return Some(InstallMethod::Cargo);
    }

    if path_str.contains("/node_modules/") || path_str.contains("/npm/") {
        return Some(InstallMethod::Npm);
    }

    if path_str.contains("/opt/homebrew/") || path_str.contains("/usr/local/Cellar/") {
        return Some(InstallMethod::SystemDefault(PackageManager::Brew));
    }

    if path_str.starts_with("/usr/bin/")
        || path_str.starts_with("/bin/")
        || path_str.starts_with("/usr/local/bin/")
    {
        return Some(InstallMethod::System);
    }

    None
}

/// Check package managers for installed package
fn check_package_managers(package_name: &str, system_info: &SystemInfo) -> Option<InstallMethod> {
    use super::command::run_command;

    if system_info.has_package_manager(PackageManager::Brew)
        && run_command("brew", &["list", package_name]).is_ok()
    {
        return Some(InstallMethod::SystemDefault(PackageManager::Brew));
    }

    if system_info.has_package_manager(PackageManager::Npm) {
        if let Ok(output) = run_command_output("npm", &["list", "-g", "--depth=0"]) {
            if output.contains(package_name) {
                return Some(InstallMethod::Npm);
            }
        }
    }

    if system_info.has_package_manager(PackageManager::Pipx) {
        if let Ok(output) = run_command_output("pipx", &["list"]) {
            if output.contains(package_name) {
                return Some(InstallMethod::Pipx);
            }
        }
    }

    None
}

/// Detect how a package was installed by checking its location and package managers
///
/// # Arguments
/// * `package_name` - Name of the package to detect
/// * `system_info` - System information for checking available package managers
///
/// # Returns
/// The detected installation method, or None if package is not installed
#[must_use]
pub fn detect_installation_method(
    package_name: &str,
    system_info: &SystemInfo,
) -> Option<InstallMethod> {
    if !is_command_available(package_name) {
        return None;
    }

    if let Some(method) = detect_from_path(package_name) {
        return Some(method);
    }

    if let Some(method) = check_package_managers(package_name, system_info) {
        return Some(method);
    }

    Some(InstallMethod::System)
}
