//! Runtime specification and version management

use serde::{Deserialize, Serialize};

/// Runtime specification for language/tool version management
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RuntimeSpec {
    /// Simple version string: "latest", "lts", "stable", "3.11", etc.
    Simple(String),
    /// Detailed configuration with manager and version info
    Detailed {
        /// Single version to install
        #[serde(default)]
        version: Option<String>,
        /// Multiple versions to install
        #[serde(default)]
        versions: Option<Vec<String>>,
        /// Default version to use
        #[serde(default)]
        default: Option<String>,
        /// Version manager to use
        #[serde(default)]
        manager: Option<String>,
        /// Required runtime dependency
        #[serde(default)]
        requires: Option<String>,
    },
}

impl RuntimeSpec {
    /// Get the version(s) to install
    #[must_use]
    pub fn get_versions(&self) -> Vec<String> {
        match self {
            Self::Simple(v) => vec![v.clone()],
            Self::Detailed {
                version, versions, ..
            } => {
                if let Some(vs) = versions {
                    vs.clone()
                } else if let Some(v) = version {
                    vec![v.clone()]
                } else {
                    vec!["latest".to_string()]
                }
            }
        }
    }

    /// Get the default version to use
    #[must_use]
    pub fn get_default_version(&self) -> String {
        match self {
            Self::Simple(v) => v.clone(),
            Self::Detailed {
                default,
                version,
                versions,
                ..
            } => default
                .clone()
                .or_else(|| version.clone())
                .or_else(|| versions.as_ref().and_then(|vs| vs.first().cloned()))
                .unwrap_or_else(|| "latest".to_string()),
        }
    }

    /// Get the preferred manager
    #[must_use]
    pub fn get_manager(&self) -> Option<String> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed { manager, .. } => manager.clone(),
        }
    }

    /// Get runtime dependency
    #[must_use]
    pub fn get_requires(&self) -> Option<String> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed { requires, .. } => requires.clone(),
        }
    }
}
