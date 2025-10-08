//! Package manager detection

use super::enums::{Distro, Os, PackageManager};

/// Detect the default package manager for the OS
pub(super) fn detect_default_package_manager(os: Os, distro: Distro) -> Option<PackageManager> {
    match os {
        Os::MacOs => {
            if is_command_available("brew") {
                Some(PackageManager::Brew)
            } else {
                None
            }
        }
        Os::Linux => match distro {
            Distro::Ubuntu | Distro::Debian => Some(PackageManager::Apt),
            Distro::Fedora | Distro::Rhel | Distro::CentOs | Distro::Rocky | Distro::Alma => {
                if is_command_available("dnf") {
                    Some(PackageManager::Dnf)
                } else if is_command_available("yum") {
                    Some(PackageManager::Yum)
                } else {
                    None
                }
            }
            Distro::Arch | Distro::Manjaro => Some(PackageManager::Pacman),
            Distro::Unknown => None,
        },
        Os::Unknown => None,
    }
}

/// Detect all available package managers on the system
pub(super) fn detect_available_package_managers() -> Vec<PackageManager> {
    let mut managers = Vec::new();

    for pm in [
        PackageManager::Brew,
        PackageManager::Apt,
        PackageManager::Pacman,
        PackageManager::Dnf,
        PackageManager::Yum,
        PackageManager::Cargo,
        PackageManager::Npm,
        PackageManager::Pipx,
    ] {
        if is_command_available(pm.command()) {
            managers.push(pm);
        }
    }

    managers
}

/// Check if a command is available in PATH
#[must_use]
pub fn is_command_available(command: &str) -> bool {
    which::which(command).is_ok()
}
