//! System information detection
//!
//! Provides the `SystemInfo` struct and detection functions for operating
//! system, distribution, architecture, and package manager availability.

use super::enums::{Arch, Distro, Os, PackageManager};
use crate::common::error::Result;

use crate::service::{os_detection, pm_detection};

// Re-export is_command_available for public use
pub use pm_detection::is_command_available;

/// System information including OS, architecture, and available package managers
#[derive(Debug, Clone)]
pub struct SystemInfo {
    /// Operating system type
    pub os: Os,
    /// Linux distribution (or Unknown for non-Linux)
    pub distro: Distro,
    /// CPU architecture
    pub arch: Arch,
    /// Default package manager for this system
    pub default_package_manager: Option<PackageManager>,
    /// All available package managers on this system
    pub available_package_managers: Vec<PackageManager>,
    /// Whether running in Windows Subsystem for Linux
    pub is_wsl: bool,
    /// Whether running on Apple Silicon (M1/M2/M3)
    pub is_apple_silicon: bool,
}

impl SystemInfo {
    /// Detect all system information
    pub fn detect() -> Result<Self> {
        let os = os_detection::detect_os();
        let distro = os_detection::detect_distro(os);
        let arch = os_detection::detect_arch();
        let is_wsl = os_detection::detect_wsl();
        let is_apple_silicon = os == Os::MacOs && arch == Arch::Arm64;
        let default_package_manager = pm_detection::detect_default_package_manager(os, distro);
        let available_package_managers = pm_detection::detect_available_package_managers();

        Ok(Self {
            os,
            distro,
            arch,
            default_package_manager,
            available_package_managers,
            is_wsl,
            is_apple_silicon,
        })
    }

    /// Check if a package manager is available
    #[must_use]
    pub fn has_package_manager(&self, pm: PackageManager) -> bool {
        self.available_package_managers.contains(&pm)
    }
}
