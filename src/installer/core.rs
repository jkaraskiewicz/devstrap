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

    /// Install all packages sequentially by group
    ///
    /// Groups are processed sequentially, and packages within each group
    /// are also installed sequentially to avoid lock file conflicts.
    pub fn install_all(&self) -> Result<()> {
        let groups = self.config.get_package_groups();

        for (idx, _group) in groups.iter().enumerate() {
            self.install_group_by_index(idx)?;
        }

        Ok(())
    }

    /// Install packages from a specific group by index
    ///
    /// # Arguments
    /// * `group_idx` - Index of the group in the packages array
    ///
    /// # Returns
    /// Result indicating success or failure
    fn install_group_by_index(&self, group_idx: usize) -> Result<()> {
        let groups = self.config.get_package_groups();
        let package_ids = match groups.get(group_idx) {
            Some(group) => group,
            None => return Ok(()),
        };

        if package_ids.is_empty() {
            return Ok(());
        }

        println!(
            "
{} {}",
            "Installing group:".bold().cyan(),
            format!("#{}", group_idx + 1).bold()
        );

        let packages = prepare_packages(package_ids, &self.system_info);
        let results = self.install_packages(&packages);
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
            packages: vec![],
            package_versions: std::collections::HashMap::new(),
            special_installs: std::collections::HashMap::new(),
            runtimes: std::collections::HashMap::new(),
            system_languages: std::collections::HashMap::new(),
        };

        let _installer = Installer::new(config, system_info, false);
    }
}
