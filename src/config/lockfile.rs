//! Runtime version lockfile
//!
//! Manages the lockfile that pins "latest", "lts", and other dynamic versions
//! to specific resolved versions for reproducible installations.

use crate::error::Result;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Lockfile containing resolved runtime versions
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Lockfile {
    /// Resolved runtime versions
    #[serde(default)]
    pub runtimes: HashMap<String, ResolvedRuntime>,

    /// Resolved framework versions
    #[serde(default)]
    pub frameworks: HashMap<String, ResolvedFramework>,
}

/// Resolved runtime with pinned version
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolvedRuntime {
    /// The version that was requested ("latest", "lts", specific version)
    pub requested: String,
    /// The actual resolved version
    pub resolved: String,
    /// Manager used for installation
    pub manager: String,
    /// Timestamp of resolution
    #[serde(default)]
    pub resolved_at: Option<String>,
}

/// Resolved framework with pinned version
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolvedFramework {
    /// The version that was requested
    pub requested: String,
    /// The actual resolved version
    pub resolved: String,
    /// Timestamp of resolution
    #[serde(default)]
    pub resolved_at: Option<String>,
}

impl Lockfile {
    /// Load lockfile from path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read lockfile: {}", path.display()))?;

        let lockfile: Self = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse lockfile: {}", path.display()))?;

        Ok(lockfile)
    }

    /// Save lockfile to path
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let contents = toml::to_string_pretty(self).context("Failed to serialize lockfile")?;

        fs::write(path, contents)
            .with_context(|| format!("Failed to write lockfile: {}", path.display()))?;

        Ok(())
    }

    /// Get resolved version for a runtime
    #[must_use]
    pub fn get_runtime_version(&self, name: &str) -> Option<&str> {
        self.runtimes.get(name).map(|r| r.resolved.as_str())
    }

    /// Get resolved version for a framework
    #[must_use]
    pub fn get_framework_version(&self, name: &str) -> Option<&str> {
        self.frameworks.get(name).map(|f| f.resolved.as_str())
    }

    /// Update or add a runtime resolution
    pub fn set_runtime(
        &mut self,
        name: String,
        requested: String,
        resolved: String,
        manager: String,
    ) {
        let resolved_at = chrono::Utc::now().to_rfc3339();
        self.runtimes.insert(
            name,
            ResolvedRuntime {
                requested,
                resolved,
                manager,
                resolved_at: Some(resolved_at),
            },
        );
    }

    /// Update or add a framework resolution
    pub fn set_framework(&mut self, name: String, requested: String, resolved: String) {
        let resolved_at = chrono::Utc::now().to_rfc3339();
        self.frameworks.insert(
            name,
            ResolvedFramework {
                requested,
                resolved,
                resolved_at: Some(resolved_at),
            },
        );
    }

    /// Check if a runtime needs re-resolution (requested version changed)
    #[must_use]
    pub fn needs_resolution(&self, name: &str, requested: &str) -> bool {
        match self.runtimes.get(name) {
            Some(resolved) => resolved.requested != requested,
            None => true,
        }
    }
}
