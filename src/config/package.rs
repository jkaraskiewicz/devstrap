//! Package configuration and builder

use serde::{Deserialize, Serialize};

/// Package configuration defining installation method
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageConfig {
    /// Package description (for --list-packages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Simple package name (used for most package managers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// NPM package name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npm: Option<String>,

    /// Cargo crate name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo: Option<String>,

    /// Pipx package name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipx: Option<String>,

    /// GitHub repository (format: "owner/repo")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,

    /// Homebrew formula name (if different from package ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brew: Option<String>,

    /// APT package name (if different from package ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apt: Option<String>,

    /// Pacman package name (if different from package ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacman: Option<String>,

    /// DNF/YUM package name (if different from package ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnf: Option<String>,
}

impl PackageConfig {
    /// Get the package name for a specific installation method
    #[must_use]
    pub fn name_for_method(&self, method: &str) -> Option<&str> {
        match method {
            "npm" => self.npm.as_deref(),
            "cargo" => self.cargo.as_deref(),
            "pipx" => self.pipx.as_deref(),
            "github" => self.github.as_deref(),
            "brew" => self.brew.as_deref().or(self.name.as_deref()),
            "apt" => self.apt.as_deref().or(self.name.as_deref()),
            "pacman" => self.pacman.as_deref().or(self.name.as_deref()),
            "dnf" | "yum" => self.dnf.as_deref().or(self.name.as_deref()),
            _ => self.name.as_deref(),
        }
    }

    /// Get all available installation methods for this package
    #[must_use]
    pub fn available_methods(&self) -> Vec<String> {
        let mut methods = Vec::new();

        if self.name.is_some() {
            methods.extend(vec![
                "brew".to_string(),
                "apt".to_string(),
                "pacman".to_string(),
                "dnf".to_string(),
                "yum".to_string(),
            ]);
        }
        if self.npm.is_some() {
            methods.push("npm".to_string());
        }
        if self.cargo.is_some() {
            methods.push("cargo".to_string());
        }
        if self.pipx.is_some() {
            methods.push("pipx".to_string());
        }
        if self.github.is_some() {
            methods.push("github".to_string());
        }
        if self.brew.is_some() {
            methods.push("brew".to_string());
        }

        methods
    }
}

