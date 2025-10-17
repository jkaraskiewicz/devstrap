//! Installation method types and utilities
//!
//! Defines installation methods and conversion functions.

use crate::domain::{PackageManager, SystemInfo};
use std::fmt;

/// Installation method for a package
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstallMethod {
    /// OS default package manager (brew, apt, etc.)
    SystemDefault(PackageManager),
    /// NPM global package
    Npm,
    /// Cargo crate
    Cargo,
    /// Pipx application
    Pipx,
    /// System package (already installed)
    System,
    /// GitHub release
    GitHub,
}

impl InstallMethod {
    /// Get priority for this installation method
    ///
    /// Priority hierarchy: OS default (10) > npm (8) > cargo (6) > pipx (4) > system (2) > github (1)
    #[must_use]
    pub fn priority(&self, system_default: Option<PackageManager>) -> u8 {
        match self {
            Self::SystemDefault(pm) => {
                if Some(*pm) == system_default {
                    10
                } else {
                    2
                }
            }
            Self::Npm => 8,
            Self::Cargo => 6,
            Self::Pipx => 4,
            Self::System => 2,
            Self::GitHub => 1,
        }
    }

    /// Get the command/tool name for this method
    #[must_use]
    pub fn command(&self) -> &str {
        match self {
            Self::SystemDefault(pm) => pm.command(),
            Self::Npm => "npm",
            Self::Cargo => "cargo",
            Self::Pipx => "pipx",
            Self::System => "system",
            Self::GitHub => "github",
        }
    }

    /// Get human-readable display name
    #[must_use]
    pub fn display_name(&self) -> String {
        match self {
            Self::SystemDefault(pm) => pm.display_name().to_string(),
            Self::Npm => "npm".to_string(),
            Self::Cargo => "Cargo".to_string(),
            Self::Pipx => "pipx".to_string(),
            Self::System => "System".to_string(),
            Self::GitHub => "GitHub".to_string(),
        }
    }

    /// Create from string method name
    #[must_use]
    pub fn from_string(s: &str, _system_info: &SystemInfo) -> Option<Self> {
        match s {
            "npm" => Some(Self::Npm),
            "cargo" => Some(Self::Cargo),
            "pipx" => Some(Self::Pipx),
            "system" => Some(Self::System),
            "github" => Some(Self::GitHub),
            "brew" => Some(Self::SystemDefault(PackageManager::Brew)),
            "apt" => Some(Self::SystemDefault(PackageManager::Apt)),
            "pacman" => Some(Self::SystemDefault(PackageManager::Pacman)),
            "dnf" => Some(Self::SystemDefault(PackageManager::Dnf)),
            "yum" => Some(Self::SystemDefault(PackageManager::Yum)),
            _ => None,
        }
    }
}

impl fmt::Display for InstallMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
