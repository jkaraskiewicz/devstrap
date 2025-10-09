//! Package installation core coordinator
//!
//! Provides the Installer struct and group-level installation coordination.

use super::helpers::{prepare_packages, report_errors};
use super::package_install;
use crate::config::Config;
use crate::detect::SystemInfo;
use crate::error::Result;
use crate::package::Package;
use colored::Colorize;
use std::sync::Arc;

/// Main installer coordinator
pub struct Installer {
    config: Arc<Config>,
    system_info: Arc<SystemInfo>,
    dry_run: bool,
}

impl Installer {
    /// Create a new installer
    #[must_use]
    pub fn new(config: Config, system_info: SystemInfo, dry_run: bool) -> Self {
        Self {
            config: Arc::new(config),
            system_info: Arc::new(system_info),
            dry_run,
        }
    }

    /// Install all packages defined in `install_groups` sequentially
    ///
    /// Groups are processed sequentially, and packages within each group
    /// are also installed sequentially to avoid lock file conflicts.
    pub fn install_all(&self) -> Result<()> {
        let groups = self.config.get_install_groups();

        for group_name in groups {
            self.install_group(&group_name)?;
        }

        Ok(())
    }

    /// Install packages from a specific group
    ///
    /// # Arguments
    /// * `group_name` - Name of the installation group
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn install_group(&self, group_name: &str) -> Result<()> {
        let package_ids = self.config.get_group_packages(group_name);

        if package_ids.is_empty() {
            return Ok(());
        }

        println!(
            "
{} {}",
            "Installing group:".bold().cyan(),
            group_name.bold()
        );

        let packages = prepare_packages(&package_ids, &self.system_info);
        let results = self.install_packages(&packages);
        let errors = Self::collect_errors(results, &packages);

        report_errors(errors);

        Ok(())
    }

    /// Update packages from a specific group to latest versions
    pub fn update_group(&self, group_name: &str) -> Result<()> {
        let package_ids = self.config.get_group_packages(group_name);

        if package_ids.is_empty() {
            return Ok(());
        }

        println!(
            "
{} {}",
            "Updating group:".bold().cyan(),
            group_name.bold()
        );

        let packages = prepare_packages(&package_ids, &self.system_info);
        let results = self.update_packages(&packages);
        let errors = Self::collect_errors(results, &packages);

        report_errors(errors);

        Ok(())
    }

    /// Update a specific package to latest version
    pub fn update_package(&self, package_id: &str) -> Result<()> {
        println!("  {} Updating {}...", "↻".cyan(), package_id.bold());

        let packages = prepare_packages(&[package_id.to_string()], &self.system_info);

        if packages.is_empty() {
            return Err(anyhow::anyhow!("Package {} not found", package_id));
        }

        let results = self.update_packages(&packages);
        let errors = Self::collect_errors(results, &packages);

        report_errors(errors);

        Ok(())
    }

    /// Install multiple packages sequentially
    fn install_packages(&self, packages: &[Package]) -> Vec<Result<()>> {
        packages
            .iter()
            .map(|package| {
                package_install::install_package(
                    package,
                    self.system_info.default_package_manager,
                    self.dry_run,
                )
            })
            .collect()
    }

    /// Update multiple packages sequentially
    fn update_packages(&self, packages: &[Package]) -> Vec<Result<()>> {
        packages
            .iter()
            .map(|package| {
                package_install::update_package(
                    package,
                    self.system_info.default_package_manager,
                    self.dry_run,
                )
            })
            .collect()
    }

    /// Collect installation errors from results
    fn collect_errors(
        results: Vec<Result<()>>,
        packages: &[Package],
    ) -> Vec<(String, anyhow::Error)> {
        results
            .into_iter()
            .enumerate()
            .filter_map(|(idx, result)| result.err().map(|e| (packages[idx].id.clone(), e)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::{Arch, Distro, Os, PackageManager};

    #[test]
    fn test_installer_creation() {
        let system_info = SystemInfo {
            os: Os::MacOs,
            distro: Distro::Unknown,
            arch: Arch::Arm64,
            default_package_manager: Some(PackageManager::Brew),
            available_package_managers: vec![PackageManager::Brew],
            is_wsl: false,
            is_apple_silicon: true,
        };

        let config = crate::config::Config {
            packages: std::collections::HashMap::new(),
            special_installs: std::collections::HashMap::new(),
            runtimes: std::collections::HashMap::new(),
            frameworks: std::collections::HashMap::new(),
            system_languages: std::collections::HashMap::new(),
        };

        let _installer = Installer::new(config, system_info, false);
    }
}
