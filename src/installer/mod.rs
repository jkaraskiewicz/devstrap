//! Package installation logic with sequential execution
//!
//! This module handles the actual installation of packages using various
//! package managers and methods. All packages are installed sequentially
//! to avoid conflicts with package manager lock files.

pub mod core;
pub mod helpers;
pub mod methods;
mod package_install;

// Re-export main types
pub use core::Installer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::detect::{Arch, Distro, Os, PackageManager, SystemInfo};
    use std::collections::HashMap;

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

    fn create_test_config() -> Config {
        let mut packages = HashMap::new();
        packages.insert("test_group".to_string(), vec!["ripgrep".to_string()]);

        Config {
            packages,
            special_installs: HashMap::new(),
            runtimes: HashMap::new(),
            frameworks: HashMap::new(),
            system_languages: HashMap::new(),
        }
    }

    #[test]
    fn test_installer_creation() {
        let config = create_test_config();
        let system_info = create_test_system_info();
        let _installer = Installer::new(config, system_info, true);
        // Test passes if installer creation succeeds
    }
}
