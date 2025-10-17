//! External service interfaces

pub mod os_detection;
pub mod package_manager;
pub mod pm_detection;
pub mod runtime;

pub use os_detection::detect_distro;
pub use pm_detection::detect_default_package_manager;
