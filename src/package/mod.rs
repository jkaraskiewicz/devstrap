//! Package types and installation priority system
//!
//! This module defines package types, installation methods, and the priority
//! hierarchy for determining the best installation method.

pub mod method;
pub mod priority;
pub mod types;

// Re-export main types for backward compatibility
pub use method::InstallMethod;
pub use priority::determine_best_method;
pub use types::Package;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PackageConfig;
    use crate::detect::{Arch, Distro, Os, PackageManager, SystemInfo};

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
    fn test_install_method_priority() {
        let system_info = create_test_system_info();
        let system_default = system_info.default_package_manager;

        let brew = InstallMethod::SystemDefault(PackageManager::Brew);
        let cargo = InstallMethod::Cargo;
        let npm = InstallMethod::Npm;

        assert_eq!(brew.priority(system_default), 10);
        assert_eq!(npm.priority(system_default), 8);
        assert_eq!(cargo.priority(system_default), 6);
    }

    #[test]
    fn test_package_should_reinstall() {
        let system_info = create_test_system_info();

        let mut package = Package {
            id: "ripgrep".to_string(),
            config: PackageConfig {
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
            },
            current_method: Some(InstallMethod::Cargo),
            preferred_method: InstallMethod::SystemDefault(PackageManager::Brew),
        };

        // Should reinstall because brew (priority 10) > cargo (priority 6)
        assert!(package.should_reinstall(system_info.default_package_manager));

        // Change current method to brew - should not reinstall
        package.current_method = Some(InstallMethod::SystemDefault(PackageManager::Brew));
        assert!(!package.should_reinstall(system_info.default_package_manager));
    }
}
