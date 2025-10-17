//! Built-in package definitions
//!
//! Contains hardcoded package definitions and base packages for different
//! package managers. Users only specify which packages to install, not how.

mod base_packages;
mod packages;

use crate::domain::PackageConfig;

pub use base_packages::BUILTIN_BASE_PACKAGES;
pub use packages::BUILTIN_PACKAGES;

/// Get a package configuration by ID
#[must_use]
pub fn get_package(package_id: &str) -> Option<&PackageConfig> {
    BUILTIN_PACKAGES.get(package_id)
}

/// Get all available package IDs sorted alphabetically
#[must_use]
pub fn get_all_package_ids() -> Vec<&'static str> {
    let mut ids: Vec<&str> = BUILTIN_PACKAGES.keys().copied().collect();
    ids.sort_unstable();
    ids
}

/// Get base packages for a specific package manager
#[must_use]
pub fn get_base_packages(pm: &str) -> Option<&Vec<&'static str>> {
    BUILTIN_BASE_PACKAGES.get(pm)
}
