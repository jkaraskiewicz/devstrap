//! FNM (Fast Node Manager) version resolution

use crate::error::Result;
use anyhow::{anyhow, Context};
use std::process::Command;

/// Resolve version using fnm
pub(super) fn resolve(version: &str) -> Result<String> {
    let output = Command::new("fnm")
        .args(["ls-remote"])
        .output()
        .context("Failed to execute fnm ls-remote")?;

    if !output.status.success() {
        return Err(anyhow!("fnm ls-remote failed"));
    }

    let versions = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = versions.lines().collect();

    match version {
        "latest" => lines
            .last()
            .and_then(|s| s.split_whitespace().next())
            .map(String::from)
            .ok_or_else(|| anyhow!("No versions found")),
        "lts" => lines
            .iter()
            .filter(|l| l.contains("LTS"))
            .next_back()
            .and_then(|s| s.split_whitespace().next())
            .map(String::from)
            .ok_or_else(|| anyhow!("No LTS version found")),
        _ => Ok(version.to_string()),
    }
}
