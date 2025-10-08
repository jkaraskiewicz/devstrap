//! Runtime manager coordinator
//!
//! Coordinates installation of programming language runtimes and frameworks
//! using the appropriate version managers (mise, rustup, sdkman, etc.)

mod framework_install;
mod manager_helpers;
mod runtime_install;
mod system_lang;
mod version_resolution;

use super::resolver::VersionResolver;
use crate::config::{Config, FrameworkSpec, Lockfile, RuntimeSpec};
use crate::error::Result;
use colored::Colorize;
use std::path::Path;

/// Runtime manager for installing language runtimes
pub struct RuntimeManager {
    config: Config,
    lockfile: Lockfile,
    dry_run: bool,
}

impl RuntimeManager {
    /// Create a new runtime manager
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
        system_lang::install_system_languages(&self.config.system_languages, self.dry_run)?;

        for (name, spec) in &self.config.runtimes.clone() {
            self.install_runtime(name, spec)?;
        }

        for (name, spec) in &self.config.frameworks.clone() {
            self.install_framework(name, spec)?;
        }

        Ok(())
    }

    /// Ensure required version managers are installed
    fn ensure_managers(&self) -> Result<()> {
        let managers_needed = manager_helpers::get_required_managers(&self.config.runtimes);

        for manager in managers_needed {
            if !manager_helpers::is_manager_installed(&manager) {
                println!("  {} Installing {} manager...", "↓".cyan(), manager);
                manager_helpers::install_manager(&manager, self.dry_run)?;
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
            let resolved = version_resolution::resolve_runtime_version(
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
                runtime_install::install_runtime_version(name, &resolved, &manager)?;
            }
        }

        if !self.dry_run {
            runtime_install::set_default_runtime(name, &default_version, &manager)?;
        }

        Ok(())
    }

    /// Install a framework
    fn install_framework(&mut self, name: &str, spec: &FrameworkSpec) -> Result<()> {
        println!(
            "\n{} {}",
            "Installing framework:".bold().cyan(),
            name.bold()
        );

        let version = spec.get_version();
        let resolved =
            VersionResolver::resolve(name, &version, None).unwrap_or_else(|_| version.clone());

        self.lockfile
            .set_framework(name.to_string(), version.clone(), resolved.clone());

        println!("  {} version {}", "↓".cyan(), resolved.green());

        framework_install::install_framework(name, &resolved, self.dry_run)?;

        Ok(())
    }

    /// Save the lockfile
    pub fn save_lockfile<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.lockfile.save(path)
    }
}
