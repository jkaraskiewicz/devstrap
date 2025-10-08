//! System and package manager enum definitions
//!
//! Defines the core enum types for operating systems, distributions,
//! architectures, and package managers for the detection system.

/// Supported operating systems
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Os {
    /// macOS operating system
    MacOs,
    /// Linux operating system
    Linux,
    /// Unknown or unsupported OS
    Unknown,
}

/// Linux distributions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distro {
    /// Ubuntu Linux
    Ubuntu,
    /// Debian Linux
    Debian,
    /// Fedora Linux
    Fedora,
    /// Red Hat Enterprise Linux
    Rhel,
    /// `CentOS` Linux
    CentOs,
    /// Rocky Linux
    Rocky,
    /// `AlmaLinux`
    Alma,
    /// Arch Linux
    Arch,
    /// Manjaro Linux
    Manjaro,
    /// Unknown or unsupported distribution
    Unknown,
}

/// System architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    /// `x86_64` / AMD64 architecture
    X86_64,
    /// ARM64 / `AArch64` architecture
    Arm64,
    /// `ARMv7` architecture
    Armv7,
    /// Unknown or unsupported architecture
    Unknown,
}

/// Available package managers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageManager {
    /// Homebrew package manager (macOS/Linux)
    Brew,
    /// APT package manager (Debian/Ubuntu)
    Apt,
    /// Pacman package manager (Arch Linux)
    Pacman,
    /// DNF package manager (Fedora/RHEL)
    Dnf,
    /// YUM package manager (older RHEL/CentOS)
    Yum,
    /// Cargo package manager (Rust)
    Cargo,
    /// npm package manager (Node.js)
    Npm,
    /// pipx package manager (Python)
    Pipx,
}

impl PackageManager {
    /// Get the command name for this package manager
    #[must_use]
    pub fn command(&self) -> &'static str {
        match self {
            Self::Brew => "brew",
            Self::Apt => "apt-get",
            Self::Pacman => "pacman",
            Self::Dnf => "dnf",
            Self::Yum => "yum",
            Self::Cargo => "cargo",
            Self::Npm => "npm",
            Self::Pipx => "pipx",
        }
    }

    /// Get human-readable name
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Brew => "Homebrew",
            Self::Apt => "APT",
            Self::Pacman => "Pacman",
            Self::Dnf => "DNF",
            Self::Yum => "YUM",
            Self::Cargo => "Cargo",
            Self::Npm => "npm",
            Self::Pipx => "pipx",
        }
    }
}
