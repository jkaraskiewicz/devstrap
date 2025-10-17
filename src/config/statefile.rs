//! State file tracking for devstrap installations
//!
//! Tracks which packages and runtimes devstrap has installed to enable
//! safe sync operations with --prune flag.

use crate::domain::InstallMethod;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Record of a package installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRecord {
    /// Installation method used
    pub method: String,
    /// Version installed (if known)
    pub version: Option<String>,
    /// Timestamp of installation
    pub installed_at: String,
}

/// Record of a runtime installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeRecord {
    /// Version installed
    pub version: String,
    /// Version manager used
    pub manager: String,
    /// Timestamp of installation
    pub installed_at: String,
}

/// State file tracking what devstrap has installed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StateFile {
    /// Packages installed by devstrap
    #[serde(default)]
    pub packages: HashMap<String, PackageRecord>,
    
    /// Runtimes installed by devstrap
    #[serde(default)]
    pub runtimes: HashMap<String, RuntimeRecord>,
}

impl StateFile {
    /// Load state file from path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read state file: {}", path.display()))?;

        let state: StateFile = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse state file: {}", path.display()))?;

        Ok(state)
    }

    /// Save state file to path
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize state file")?;

        fs::write(path, contents)
            .with_context(|| format!("Failed to write state file: {}", path.display()))?;

        Ok(())
    }

    /// Add a package to the state
    pub fn add_package(&mut self, id: String, method: &InstallMethod, version: Option<String>) {
        let record = PackageRecord {
            method: method.display_name(),
            version,
            installed_at: chrono::Utc::now().to_rfc3339(),
        };
        self.packages.insert(id, record);
    }

    /// Remove a package from the state
    pub fn remove_package(&mut self, id: &str) {
        self.packages.remove(id);
    }

    /// Add a runtime to the state
    pub fn add_runtime(&mut self, name: String, version: String, manager: String) {
        let record = RuntimeRecord {
            version,
            manager,
            installed_at: chrono::Utc::now().to_rfc3339(),
        };
        self.runtimes.insert(name, record);
    }

    /// Remove a runtime from the state
    pub fn remove_runtime(&mut self, name: &str) {
        self.runtimes.remove(name);
    }

    /// Check if a package was installed by devstrap
    #[must_use]
    pub fn has_package(&self, id: &str) -> bool {
        self.packages.contains_key(id)
    }

    /// Check if a runtime was installed by devstrap
    #[must_use]
    pub fn has_runtime(&self, name: &str) -> bool {
        self.runtimes.contains_key(name)
    }

    /// Get all package IDs installed by devstrap
    #[must_use]
    pub fn get_package_ids(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }

    /// Get all runtime names installed by devstrap
    #[must_use]
    pub fn get_runtime_names(&self) -> Vec<String> {
        self.runtimes.keys().cloned().collect()
    }
}
