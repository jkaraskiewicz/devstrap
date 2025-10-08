//! Main configuration structure

use super::{FrameworkSpec, RuntimeSpec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration structure matching config.toml format
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Packages to install, organized in groups
    /// Group names are custom (e.g., "base", "`dev_tools`", "`my_tools`")
    /// Groups are installed sequentially in the order they appear
    #[serde(default)]
    pub packages: HashMap<String, Vec<String>>,

    /// Special installation scripts for GitHub releases
    #[serde(default)]
    pub special_installs: HashMap<String, String>,

    /// Runtime and language version management
    #[serde(default)]
    pub runtimes: HashMap<String, RuntimeSpec>,

    /// Framework configurations
    #[serde(default)]
    pub frameworks: HashMap<String, FrameworkSpec>,

    /// System-provided compilers/languages to install
    #[serde(default)]
    pub system_languages: HashMap<String, bool>,
}

impl Config {
    /// Get packages in a specific installation group
    #[must_use]
    pub fn get_group_packages(&self, group: &str) -> Vec<String> {
        self.packages.get(group).cloned().unwrap_or_default()
    }

    /// Get all installation groups in order
    #[must_use]
    pub fn get_install_groups(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }
}
