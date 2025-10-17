//! Version resolution
//!
//! Resolves version shortcuts like "latest", "lts", "stable" to actual versions
//! using the appropriate version manager tools.

mod fnm;
mod mise;
mod pyenv;
mod rbenv;
mod rustup;
mod sdkman;

use crate::common::error::Result;
use anyhow::anyhow;

/// Version resolver for different runtime managers
pub struct VersionResolver;

impl VersionResolver {
    /// Resolve a version string to an actual version
    pub fn resolve(runtime: &str, version: &str, manager: Option<&str>) -> Result<String> {
        if Self::is_specific_version(version) {
            return Ok(version.to_string());
        }

        let mgr = manager.unwrap_or_else(|| Self::default_manager(runtime));

        match mgr {
            "mise" => mise::resolve(runtime, version),
            "rustup" => Ok(rustup::resolve(version)),
            "fnm" => fnm::resolve(version),
            "sdkman" => sdkman::resolve(runtime, version),
            "pyenv" => pyenv::resolve(version),
            "rbenv" => rbenv::resolve(version),
            _ => Err(anyhow!("Unknown manager: {mgr}")),
        }
    }

    /// Check if a version is specific (not a keyword)
    fn is_specific_version(version: &str) -> bool {
        !matches!(version, "latest" | "lts" | "stable" | "beta" | "nightly")
    }

    /// Get default manager for a runtime
    #[must_use]
    pub fn default_manager(runtime: &str) -> &str {
        match runtime {
            "node" | "nodejs" => "fnm",
            "java" | "kotlin" | "scala" | "groovy" => "sdkman",
            "rust" => "rustup",
            _ => "mise",
        }
    }
}
