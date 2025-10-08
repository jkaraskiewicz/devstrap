//! Builder methods for PackageConfig

use super::package::PackageConfig;

impl PackageConfig {
    /// Create a new package config with all fields None
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the package description
    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set the default package name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the npm package name
    #[must_use]
    pub fn npm(mut self, name: impl Into<String>) -> Self {
        self.npm = Some(name.into());
        self
    }

    /// Set the cargo crate name
    #[must_use]
    pub fn cargo(mut self, name: impl Into<String>) -> Self {
        self.cargo = Some(name.into());
        self
    }

    /// Set the pipx package name
    #[must_use]
    pub fn pipx(mut self, name: impl Into<String>) -> Self {
        self.pipx = Some(name.into());
        self
    }

    /// Set the GitHub repository (format: "owner/repo")
    #[must_use]
    pub fn github(mut self, repo: impl Into<String>) -> Self {
        self.github = Some(repo.into());
        self
    }

    /// Set the Homebrew formula name
    #[must_use]
    pub fn brew(mut self, name: impl Into<String>) -> Self {
        self.brew = Some(name.into());
        self
    }

    /// Set the APT package name
    #[must_use]
    pub fn apt(mut self, name: impl Into<String>) -> Self {
        self.apt = Some(name.into());
        self
    }

    /// Set the Pacman package name
    #[must_use]
    pub fn pacman(mut self, name: impl Into<String>) -> Self {
        self.pacman = Some(name.into());
        self
    }

    /// Set the DNF/YUM package name
    #[must_use]
    pub fn dnf(mut self, name: impl Into<String>) -> Self {
        self.dnf = Some(name.into());
        self
    }
}
