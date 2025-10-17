//! Package manager service interfaces

pub mod installer;
pub mod methods;

// Re-export update function for use by installation coordinator
pub use methods::update_package_manager;
