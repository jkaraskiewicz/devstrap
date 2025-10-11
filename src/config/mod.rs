//! Configuration parsing and validation
//!
//! This module handles parsing the TOML configuration file and validating
//! all package definitions, installation groups, and dotfile configurations.

pub mod config;
pub mod framework;
pub mod lockfile;
pub mod package;
pub mod statefile;
mod package_builder;
mod package_tests;
pub mod parse;
pub mod runtime;

// Re-export all public types
pub use config::*;
pub use framework::*;
pub use lockfile::*;
pub use package::*;
pub use runtime::*;
pub use statefile::*;
