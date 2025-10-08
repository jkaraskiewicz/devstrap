//! Package types and data structures
//!
//! Core package structures and basic methods.

use super::method::InstallMethod;
use crate::config::PackageConfig;
use crate::detect::PackageManager;

/// Package with its configuration and metadata
#[derive(Debug, Clone)]
pub struct Package {
    /// Package identifier
    pub id: String,
    /// Package configuration from TOML
    pub config: PackageConfig,
    /// Currently installed method (if detected)
    pub current_method: Option<InstallMethod>,
    /// Preferred installation method
    pub preferred_method: InstallMethod,
}

impl Package {
    /// Create a new package with preferred method
    #[must_use]
    pub fn new(id: String, config: PackageConfig, preferred_method: InstallMethod) -> Self {
        Self {
            id,
            config,
            current_method: None,
            preferred_method,
        }
    }

    /// Get the package name for the preferred installation method
    #[must_use]
    pub fn package_name(&self) -> Option<String> {
        let method_str = match &self.preferred_method {
            InstallMethod::SystemDefault(pm) => pm.command(),
            InstallMethod::Npm => "npm",
            InstallMethod::Cargo => "cargo",
            InstallMethod::Pipx => "pipx",
            InstallMethod::GitHub => "github",
            InstallMethod::System => return Some(self.id.clone()),
        };

        self.config
            .name_for_method(method_str)
            .map(std::string::ToString::to_string)
    }

    /// Check if this package should be reinstalled
    ///
    /// Returns true if:
    /// - Package is not currently installed
    /// - Preferred method has higher priority than current method
    #[must_use]
    pub fn should_reinstall(&self, system_default: Option<PackageManager>) -> bool {
        match &self.current_method {
            None => false, // Not installed, so should install (not reinstall)
            Some(current) => {
                let current_priority = current.priority(system_default);
                let preferred_priority = self.preferred_method.priority(system_default);
                preferred_priority > current_priority
            }
        }
    }

    /// Check if package is currently installed
    #[must_use]
    pub fn is_installed(&self) -> bool {
        self.current_method.is_some()
    }
}
