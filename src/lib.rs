//! devstrap - Universal development environment bootstrapper
//!
//! A high-performance, type-safe development environment setup tool
//! written in Rust. Supports sequential package installation, priority-based
//! package manager selection.
//!
//! # Features
//!
//! - Type-safe configuration parsing
//! - Sequential package installation (prevents lock conflicts)
//! - Priority-based installation method selection
//! - Comprehensive error handling
//! - Cross-platform support (macOS, Linux)
//! - Dry-run mode for safety

// Re-export main types
pub use common::Result;
pub use domain::{
    Arch, Config, Distro, InstallMethod, Lockfile, Os, Package, PackageConfig, PackageManager,
    SystemInfo,
};
pub use usecase::Installer;

// Module declarations
pub mod builtin;
pub mod common;
pub mod config;
pub mod domain;
pub mod service;
pub mod usecase;

/// Initialize tracing subscriber for logging
pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();
}
