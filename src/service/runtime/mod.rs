//! Runtime and version manager service interfaces

pub mod framework_install;
pub mod resolution;
pub mod resolver;
pub mod runtime_install;
pub mod setup;
pub mod system_lang;

pub use framework_install::install_framework;
pub use resolution::resolve_runtime_version;
pub use resolver::VersionResolver;
pub use runtime_install::{install_runtime_version, set_default_runtime};
pub use setup::{get_required_managers, install_manager, is_manager_installed};
pub use system_lang::install_system_languages;
