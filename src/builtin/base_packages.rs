//! Base package sets for package managers

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Built-in base packages per package manager
///
/// These are the essential packages installed first on each system
/// before any user-specified packages.
pub static BUILTIN_BASE_PACKAGES: Lazy<HashMap<&'static str, Vec<&'static str>>> =
    Lazy::new(|| {
        let mut base = HashMap::new();

        base.insert(
            "apt",
            vec![
                "git",
                "curl",
                "wget",
                "build-essential",
                "software-properties-common",
                "nodejs",
                "cargo",
            ],
        );

        base.insert("brew", vec!["git", "curl", "wget", "node", "rust"]);

        base.insert(
            "pacman",
            vec!["git", "curl", "wget", "base-devel", "nodejs", "rust"],
        );

        base.insert(
            "dnf",
            vec![
                "git",
                "curl",
                "wget",
                "@development-tools",
                "nodejs",
                "cargo",
            ],
        );

        base.insert(
            "yum",
            vec![
                "git", "curl", "wget", "gcc", "gcc-c++", "make", "nodejs", "cargo",
            ],
        );

        base
    });
