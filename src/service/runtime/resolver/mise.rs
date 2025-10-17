//! Mise version resolution

use crate::common::error::Result;
use anyhow::{anyhow, Context};
use std::process::Command;

/// Resolve version using mise
pub(super) fn resolve(runtime: &str, version: &str) -> Result<String> {
    let output = Command::new("mise")
        .args(["ls-remote", runtime])
        .output()
        .context("Failed to execute mise ls-remote")?;

    if !output.status.success() {
        return Err(anyhow!("mise ls-remote failed"));
    }

    let versions = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = versions.lines().collect();

    match version {
        "latest" => lines
            .last()
            .map(|s| s.trim().to_string())
            .ok_or_else(|| anyhow!("No versions found for {runtime}")),
        _ => Ok(version.to_string()),
    }
}
