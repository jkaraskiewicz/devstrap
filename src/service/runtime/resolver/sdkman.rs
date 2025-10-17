//! SDKMAN version resolution

use crate::common::error::Result;
use anyhow::{anyhow, Context};
use std::process::Command;

/// Resolve version using sdkman
pub(super) fn resolve(runtime: &str, version: &str) -> Result<String> {
    let output = Command::new("bash")
        .args([
            "-c",
            &format!("source ~/.sdkman/bin/sdkman-init.sh && sdk list {runtime}"),
        ])
        .output()
        .context("Failed to execute sdk list")?;

    if !output.status.success() {
        return Err(anyhow!("sdk list failed"));
    }

    let versions = String::from_utf8_lossy(&output.stdout);

    match version {
        "latest" => {
            // Parse sdkman output to find the latest version
            versions
                .lines()
                .find(|l| l.contains(">>>"))
                .and_then(|l| l.split_whitespace().nth(1))
                .map(String::from)
                .ok_or_else(|| anyhow!("No latest version found for {runtime}"))
        }
        _ => Ok(version.to_string()),
    }
}
