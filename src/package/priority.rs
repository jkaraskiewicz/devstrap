//! Package installation priority logic
//!
//! Determines the best installation method based on priority and availability.

use super::method::InstallMethod;
use crate::config::PackageConfig;
use crate::detect::{PackageManager, SystemInfo};

/// Determine the best installation method for a package
///
/// # Arguments
/// * `package_config` - Package configuration
/// * `system_info` - System information
///
/// # Returns
/// The best installation method based on priority and availability
#[must_use]
pub fn determine_best_method(
    package_config: &PackageConfig,
    system_info: &SystemInfo,
) -> Option<InstallMethod> {
    let available_methods = package_config.available_methods();
    let system_default = system_info.default_package_manager;

    let mut candidates: Vec<InstallMethod> = Vec::new();

    for method_str in &available_methods {
        if let Some(method) = InstallMethod::from_string(method_str, system_info) {
            // Check if this method is actually available on the system
            let is_available = match &method {
                InstallMethod::SystemDefault(pm) => system_info.has_package_manager(*pm),
                InstallMethod::Npm => system_info.has_package_manager(PackageManager::Npm),
                InstallMethod::Cargo => system_info.has_package_manager(PackageManager::Cargo),
                InstallMethod::Pipx => system_info.has_package_manager(PackageManager::Pipx),
                InstallMethod::GitHub | InstallMethod::System => true, // Always available
            };

            if is_available {
                candidates.push(method);
            }
        }
    }

    // Sort by priority (highest first)
    candidates.sort_by(|a, b| {
        let a_priority = a.priority(system_default);
        let b_priority = b.priority(system_default);
        b_priority.cmp(&a_priority)
    });

    candidates.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::{Arch, Distro, Os};

    fn create_test_system_info() -> SystemInfo {
        SystemInfo {
            os: Os::MacOs,
            distro: Distro::Unknown,
            arch: Arch::Arm64,
            default_package_manager: Some(PackageManager::Brew),
            available_package_managers: vec![
                PackageManager::Brew,
                PackageManager::Cargo,
                PackageManager::Npm,
            ],
            is_wsl: false,
            is_apple_silicon: true,
        }
    }

    #[test]
    fn test_determine_best_method() {
        let system_info = create_test_system_info();

        let package_config = crate::config::PackageConfig {
            description: None,
            name: Some("ripgrep".to_string()),
            cargo: Some("ripgrep".to_string()),
            npm: None,
            pipx: None,
            github: None,
            brew: None,
            apt: None,
            pacman: None,
            dnf: None,
        };

        let method = determine_best_method(&package_config, &system_info).unwrap();
        // Should prefer brew (system default, priority 10) over cargo (priority 6)
        assert!(matches!(
            method,
            InstallMethod::SystemDefault(PackageManager::Brew)
        ));
    }
}
