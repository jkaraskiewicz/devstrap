//! Framework specification and configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Framework specification
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FrameworkSpec {
    /// Simple version string
    Simple(String),
    /// Detailed configuration
    Detailed {
        /// Framework version
        version: Option<String>,
        /// Required runtime dependency
        #[serde(default)]
        requires: Option<String>,
        /// Extra configuration options
        #[serde(flatten)]
        extra: HashMap<String, toml::Value>,
    },
}

impl FrameworkSpec {
    /// Get the version to install
    #[must_use]
    pub fn get_version(&self) -> String {
        match self {
            Self::Simple(v) => v.clone(),
            Self::Detailed { version, .. } => {
                version.clone().unwrap_or_else(|| "latest".to_string())
            }
        }
    }

    /// Get framework dependency
    #[must_use]
    pub fn get_requires(&self) -> Option<String> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed { requires, .. } => requires.clone(),
        }
    }

    /// Get extra configuration options
    #[must_use]
    pub fn get_extra(&self) -> HashMap<String, toml::Value> {
        match self {
            Self::Simple(_) => HashMap::new(),
            Self::Detailed { extra, .. } => extra.clone(),
        }
    }
}
