//! Domain models and entities

pub mod config;
pub mod enums;
pub mod framework;
pub mod lockfile;
pub mod method;
pub mod package;
pub mod package_config;
pub mod priority;
pub mod runtime;
pub mod system;

#[cfg(test)]
mod package_config_tests;

pub use config::Config;
pub use enums::{Arch, Distro, Os, PackageManager};
pub use lockfile::Lockfile;
pub use method::InstallMethod;
pub use package::Package;
pub use package_config::PackageConfig;
pub use priority::determine_best_method;
pub use system::SystemInfo;
