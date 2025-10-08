//! Path manipulation utilities
//!
//! Functions for path expansion and directory operations.

use crate::error::Result;
use anyhow::{anyhow, Context};
use std::path::PathBuf;

/// Expand tilde in path to home directory
///
/// # Arguments
/// * `path` - Path potentially containing tilde
///
/// # Returns
/// Result with expanded path
pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    shellexpand::tilde(path)
        .parse::<PathBuf>()
        .with_context(|| format!("Failed to expand path: {path}"))
}

/// Ensure a directory exists, creating it if necessary
///
/// # Arguments
/// * `path` - Directory path
///
/// # Returns
/// Result indicating success or failure
pub fn ensure_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

/// Get the home directory
pub fn home_dir() -> Result<PathBuf> {
    home::home_dir().ok_or_else(|| anyhow!("Unable to determine home directory"))
}

/// Get the local bin directory (~/.local/bin)
pub fn local_bin_dir() -> Result<PathBuf> {
    let home = home_dir()?;
    Ok(home.join(".local").join("bin"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let expanded = expand_tilde("~/.config").unwrap();
        assert!(!expanded.to_string_lossy().contains('~'));
    }

    #[test]
    fn test_home_dir() {
        let home = home_dir().unwrap();
        assert!(home.exists());
    }

    #[test]
    fn test_local_bin_dir() {
        let local_bin = local_bin_dir().unwrap();
        assert!(local_bin.to_string_lossy().contains(".local/bin"));
    }
}
