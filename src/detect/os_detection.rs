//! Operating system and architecture detection

use super::enums::{Arch, Distro, Os};
use std::fs;

/// Detect the operating system
pub(crate) fn detect_os() -> Os {
    match std::env::consts::OS {
        "macos" => Os::MacOs,
        "linux" => Os::Linux,
        _other => Os::Unknown,
    }
}

/// Detect system architecture
pub(crate) fn detect_arch() -> Arch {
    match std::env::consts::ARCH {
        "x86_64" | "amd64" => Arch::X86_64,
        "aarch64" | "arm64" => Arch::Arm64,
        "armv7" | "armv7l" => Arch::Armv7,
        _ => Arch::Unknown,
    }
}

/// Detect the Linux distribution
pub(super) fn detect_distro(os: Os) -> Distro {
    if os != Os::Linux {
        return Distro::Unknown;
    }

    // Check /etc/os-release first (standard on modern systems)
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if let Some(id) = line.strip_prefix("ID=") {
                let id = id.trim_matches('"').to_lowercase();
                return match id.as_str() {
                    "ubuntu" => Distro::Ubuntu,
                    "debian" => Distro::Debian,
                    "fedora" => Distro::Fedora,
                    "rhel" => Distro::Rhel,
                    "centos" => Distro::CentOs,
                    "rocky" => Distro::Rocky,
                    "almalinux" => Distro::Alma,
                    "arch" => Distro::Arch,
                    "manjaro" => Distro::Manjaro,
                    _ => Distro::Unknown,
                };
            }
        }
    }

    // Fallback to checking specific files
    if fs::metadata("/etc/debian_version").is_ok() {
        return Distro::Debian;
    }
    if fs::metadata("/etc/redhat-release").is_ok() {
        return Distro::Rhel;
    }
    if fs::metadata("/etc/arch-release").is_ok() {
        return Distro::Arch;
    }

    Distro::Unknown
}

/// Detect if running in WSL (Windows Subsystem for Linux)
pub(super) fn detect_wsl() -> bool {
    if let Ok(contents) = fs::read_to_string("/proc/version") {
        contents.to_lowercase().contains("microsoft")
    } else {
        false
    }
}
