//! Version manager installation helpers

use crate::domain::runtime::RuntimeSpec;
use crate::common::error::Result;
use super::resolver::VersionResolver;
use crate::common::run_command;
use anyhow::anyhow;
use colored::Colorize;
use std::collections::HashMap;

/// Check if a version manager is installed
pub fn is_manager_installed(manager: &str) -> bool {
    which::which(manager).is_ok()
}

/// Install a version manager
pub fn install_manager(manager: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("    {} Would install {}", "[DRY-RUN]".yellow(), manager);
        return Ok(());
    }

    match manager {
        "mise" => {
            run_command("curl", &["https://mise.run", "|", "sh"])?;
        }
        "rustup" => {
            run_command(
                "curl",
                &[
                    "--proto",
                    "=https",
                    "--tlsv1.2",
                    "-sSf",
                    "https://sh.rustup.rs",
                    "|",
                    "sh",
                    "-s",
                    "--",
                    "-y",
                ],
            )?;
        }
        "fnm" => {
            run_command(
                "curl",
                &["-fsSL", "https://fnm.vercel.app/install", "|", "bash"],
            )?;
        }
        _ => return Err(anyhow!("Don't know how to install manager: {manager}")),
    }

    Ok(())
}

/// Get list of required version managers from runtime configuration
pub fn get_required_managers(
    runtimes: &HashMap<String, RuntimeSpec>,
) -> Vec<String> {
    let mut managers = Vec::new();

    for (runtime, spec) in runtimes {
        let mgr = spec
            .get_manager()
            .unwrap_or_else(|| VersionResolver::default_manager(runtime).to_string());

        if !managers.contains(&mgr) {
            managers.push(mgr);
        }
    }

    managers
}
