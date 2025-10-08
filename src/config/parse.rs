//! Configuration parsing and validation
//!
//! Implements loading, parsing, and validating configuration files.

use super::config::Config;
use crate::builtin;
use crate::error::Result;
use anyhow::{anyhow, Context};
use std::fs;
use std::path::Path;

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse TOML in {}", path.display()))?;

        config.validate()?;
        Ok(config)
    }

    /// Validate that all packages in groups are available in builtin packages
    fn validate_group_packages(&self) -> Result<()> {
        for (group_name, packages) in &self.packages {
            for package_id in packages {
                if builtin::get_package(package_id).is_none() {
                    return Err(anyhow!(
                        "Package '{package_id}' in group '{group_name}' is not a supported package. \
                         Run 'devstrap --list-packages' to see available packages."
                    ));
                }
            }
        }
        Ok(())
    }

    /// Validate the entire configuration
    ///
    /// Ensures all package references are valid builtin packages.
    pub fn validate(&self) -> Result<()> {
        self.validate_group_packages()?;
        Ok(())
    }

}
