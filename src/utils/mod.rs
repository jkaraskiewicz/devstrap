//! Utility functions for devstrap
//!
//! This module provides utility functions organized by functionality.

pub mod command;
pub mod detection;
pub mod path;
pub mod uninstall;

// Re-export commonly used functions for backward compatibility
pub use command::{run_command, run_command_output};
pub use detection::detect_installation_method;
pub use path::{ensure_dir, expand_tilde, home_dir, local_bin_dir};
pub use uninstall::uninstall_package;
