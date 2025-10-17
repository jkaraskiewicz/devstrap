//! Main configuration structure

use super::runtime::RuntimeSpec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Custom deserializer for packages field
/// Supports both flat array and nested arrays
fn deserialize_packages<'de, D>(deserializer: D) -> Result<Vec<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct PackagesVisitor;

    impl<'de> Visitor<'de> for PackagesVisitor {
        type Value = Vec<Vec<String>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a flat array of strings or nested arrays of strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            let mut current_group = Vec::new();
            let mut is_nested = None;

            while let Some(value) = seq.next_element::<toml::Value>()? {
                match value {
                    toml::Value::String(s) => {
                        // Flat array: packages = ["git", "curl"]
                        if is_nested == Some(true) {
                            return Err(de::Error::custom(
                                "Cannot mix strings and arrays in packages field",
                            ));
                        }
                        is_nested = Some(false);
                        current_group.push(s);
                    }
                    toml::Value::Array(arr) => {
                        // Nested array: packages = [["git"], ["curl"]]
                        if is_nested == Some(false) {
                            return Err(de::Error::custom(
                                "Cannot mix strings and arrays in packages field",
                            ));
                        }
                        is_nested = Some(true);

                        let group: Vec<String> = arr
                            .into_iter()
                            .map(|v| match v {
                                toml::Value::String(s) => Ok(s),
                                _ => Err(de::Error::custom("Expected string in package array")),
                            })
                            .collect::<Result<_, _>>()?;
                        result.push(group);
                    }
                    _ => {
                        return Err(de::Error::custom("Expected string or array in packages field"))
                    }
                }
            }

            // If flat array, wrap in outer array
            if is_nested == Some(false) {
                result.push(current_group);
            } else if is_nested.is_none() {
                // Empty array
                result = vec![];
            }

            Ok(result)
        }
    }

    deserializer.deserialize_seq(PackagesVisitor)
}

/// Main configuration structure matching config.toml format
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Packages to install - supports both flat and nested arrays
    ///
    /// # Examples
    ///
    /// ```toml
    /// # Flat array
    /// packages = ["git", "curl", "ripgrep"]
    ///
    /// # Nested array for installation ordering
    /// packages = [["git", "curl"], ["ripgrep", "bat"]]
    /// ```
    #[serde(default, deserialize_with = "deserialize_packages")]
    pub packages: Vec<Vec<String>>,

    /// Optional: Pin specific package versions
    ///
    /// # Example
    ///
    /// ```toml
    /// [package_versions]
    /// git = "2.43.0"
    /// neovim = "0.9.5"
    /// ```
    #[serde(default)]
    pub package_versions: HashMap<String, String>,

    /// Special installation scripts for GitHub releases
    #[serde(default)]
    pub special_installs: HashMap<String, String>,

    /// Runtime and language version management
    #[serde(default)]
    pub runtimes: HashMap<String, RuntimeSpec>,

    /// System-provided compilers/languages to install
    #[serde(default)]
    pub system_languages: HashMap<String, bool>,
}

impl Config {
    /// Get all package groups (flat list of groups)
    #[must_use]
    pub fn get_package_groups(&self) -> &[Vec<String>] {
        &self.packages
    }

    /// Get all packages flattened into a single list
    #[must_use]
    pub fn get_all_packages(&self) -> Vec<String> {
        self.packages.iter().flatten().cloned().collect()
    }

    /// Get the version for a specific package (if pinned)
    #[must_use]
    pub fn get_package_version(&self, package_id: &str) -> Option<&str> {
        self.package_versions.get(package_id).map(String::as_str)
    }
}
