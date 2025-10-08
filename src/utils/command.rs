//! Command execution utilities
//!
//! Functions for running external commands and capturing output.

use crate::error::Result;
use anyhow::{anyhow, Context};
use std::process::{Command, Output};

/// Run a command and return result
///
/// # Arguments
/// * `command` - Command to run
/// * `args` - Command arguments
///
/// # Returns
/// Result with command output or error
pub fn run_command(command: &str, args: &[&str]) -> Result<Output> {
    let cmd_str = format!("{} {}", command, args.join(" "));

    let output = Command::new(command)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute command: {cmd_str}"))?;

    if !output.status.success() {
        let exit_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Command '{cmd_str}' failed with exit code {exit_code}: {stderr}"
        ));
    }

    Ok(output)
}

/// Run a command and return stdout as string
pub fn run_command_output(command: &str, args: &[&str]) -> Result<String> {
    let output = run_command(command, args)?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
