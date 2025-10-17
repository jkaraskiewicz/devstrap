//! Pyenv version resolution

use crate::common::error::Result;
use anyhow::{anyhow, Context};
use std::process::Command;

/// Resolve version using pyenv
pub(super) fn resolve(version: &str) -> Result<String> {
    let output = Command::new("pyenv")
        .args(["install", "--list"])
        .output()
        .context("Failed to execute pyenv install --list")?;

    if !output.status.success() {
        return Err(anyhow!("pyenv install --list failed"));
    }

    let versions = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = versions
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.contains('-'))
        .collect();

    match version {
        "latest" => lines
            .last()
            .map(|s| (*s).to_string())
            .ok_or_else(|| anyhow!("No versions found")),
        _ => Ok(version.to_string()),
    }
}
