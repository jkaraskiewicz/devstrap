//! Platform and package manager detection
//!
//! This module provides functionality to detect the operating system,
//! distribution, architecture, and available package managers.

pub mod enums;
pub mod system;

mod os_detection;
mod pm_detection;

pub use enums::*;
pub use system::*;

// Re-export functions needed by tests
#[allow(unused_imports)]
pub(crate) use os_detection::{detect_arch, detect_os};

// Tests from original module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_detection() {
        let os = detect_os();
        assert!(matches!(os, Os::MacOs | Os::Linux));
    }

    #[test]
    fn test_arch_detection() {
        let arch = detect_arch();
        assert!(matches!(
            arch,
            Arch::X86_64 | Arch::Arm64 | Arch::Armv7 | Arch::Unknown
        ));
    }

    #[test]
    fn test_system_info_detection() {
        let info = SystemInfo::detect().unwrap();
        assert!(matches!(info.os, Os::MacOs | Os::Linux));
    }

    #[test]
    fn test_package_manager_command() {
        assert_eq!(PackageManager::Brew.command(), "brew");
        assert_eq!(PackageManager::Cargo.command(), "cargo");
    }
}
