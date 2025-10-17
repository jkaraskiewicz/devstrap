//! Runtime installation coordinator
//!
//! Coordinates installation of programming language runtimes and frameworks
//! using the appropriate version managers (mise, rustup, sdkman, etc.)

use crate::common::error::Result;
use crate::domain::{Config, Lockfile};
use crate::domain::runtime::RuntimeSpec;
use crate::service::runtime::{
    get_required_managers, install_manager, install_runtime_version,
    install_system_languages, is_manager_installed, resolve_runtime_version,
    set_default_runtime, VersionResolver,
};
use colored::Colorize;
use std::path::Path;

/// Runtime manager for installing language runtimes
pub struct RuntimeCoordinator {
    config: Config,
    lockfile: Lockfile,
    dry_run: bool,
}

impl RuntimeCoordinator {
    /// Create a new runtime coordinator
    #[must_use]
    pub fn new(config: Config, lockfile: Lockfile, dry_run: bool) -> Self {
        Self {
            config,
            lockfile,
            dry_run,
        }
    }

    /// Install all configured runtimes and frameworks
    pub fn install_all(&mut self) -> Result<()> {
        println!("\n{}", "═".repeat(60).cyan());
        println!("{}", "RUNTIME INSTALLATION".bold().cyan());
        println!("{}", "═".repeat(60).cyan());

        self.ensure_managers()?;
        install_system_languages(&self.config.system_languages, self.dry_run)?;

        for (name, spec) in &self.config.runtimes.clone() {
            self.install_runtime(name, spec)?;
        }

        Ok(())
    }

    /// Ensure required version managers are installed
    fn ensure_managers(&self) -> Result<()> {
        let managers_needed = get_required_managers(&self.config.runtimes);

        for manager in managers_needed {
            if !is_manager_installed(&manager) {
                println!("  {} Installing {} manager...", "↓".cyan(), manager);
                install_manager(&manager, self.dry_run)?;
            }
        }

        Ok(())
    }

    /// Install a runtime
    fn install_runtime(&mut self, name: &str, spec: &RuntimeSpec) -> Result<()> {
        println!("\n{} {}", "Installing runtime:".bold().cyan(), name.bold());

        let manager = spec
            .get_manager()
            .unwrap_or_else(|| VersionResolver::default_manager(name).to_string());

        let versions = spec.get_versions();
        let default_version = spec.get_default_version();

        for version in &versions {
            let requested = version.clone();
            let resolved = resolve_runtime_version(
                name,
                &requested,
                &manager,
                &mut self.lockfile,
            )?;

            println!(
                "  {} version {} (resolved: {})",
                "↓".cyan(),
                requested,
                resolved.green()
            );

            if !self.dry_run {
                install_runtime_version(name, &resolved, &manager)?;
            }
        }

        if !self.dry_run {
            set_default_runtime(name, &default_version, &manager)?;
        }

        Ok(())
    }

    /// Save the lockfile
    pub fn save_lockfile<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.lockfile.save(path)
    }
}
