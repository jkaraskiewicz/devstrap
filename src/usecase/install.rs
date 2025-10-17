//! Package installation core coordinator
//!
//! Provides the Installer struct and group-level installation coordination.

use super::orchestration::{prepare_packages, report_errors};
use crate::service::package_manager::{installer, update_package_manager};
use crate::domain::Config;
use crate::domain::SystemInfo;
use crate::common::error::Result;
use crate::domain::Package;
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
        // Update package manager cache before installing packages
        // Only update if we have a system package manager (not Cargo/npm/pipx)
        use crate::domain::PackageManager;
        if let Some(pm) = self.system_info.default_package_manager {
            if matches!(
                pm,
                PackageManager::Apt
                    | PackageManager::Brew
                    | PackageManager::Pacman
                    | PackageManager::Dnf
                    | PackageManager::Yum
            ) {
                update_package_manager(pm, self.dry_run)?;
            }
        }

        let groups = self.config.get_package_groups();

        for (idx, _group) in groups.iter().enumerate() {
            self.install_group_by_index(idx);
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
    fn install_group_by_index(&self, group_idx: usize) {
        let groups = self.config.get_package_groups();
        let Some(package_ids) = groups.get(group_idx) else {
            return;
        };

        if package_ids.is_empty() {
            return;
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
    }



    /// Install multiple packages sequentially
    fn install_packages(&self, packages: &[Package]) -> Vec<Result<()>> {
        packages
            .iter()
            .map(|package| {
                installer::install_package(
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
    use crate::domain::{Arch, Distro, Os, PackageManager};

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

        let config = crate::domain::Config {
            packages: vec![],
            package_versions: std::collections::HashMap::new(),
            special_installs: std::collections::HashMap::new(),
            runtimes: std::collections::HashMap::new(),
            system_languages: std::collections::HashMap::new(),
        };

        let _installer = Installer::new(config, system_info, false);
    }
}
