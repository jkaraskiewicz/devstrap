//! Runtime version installation

use crate::error::Result;
use crate::utils::run_command;
use anyhow::anyhow;

/// Install a specific runtime version
pub(super) fn install_runtime_version(runtime: &str, version: &str, manager: &str) -> Result<()> {
    match manager {
        "mise" => {
            run_command("mise", &["install", runtime, version])?;
        }
        "rustup" => {
            run_command("rustup", &["toolchain", "install", version])?;
        }
        "fnm" => {
            run_command("fnm", &["install", version])?;
        }
        "sdkman" => {
            run_command(
                "bash",
                &[
                    "-c",
                    &format!("source ~/.sdkman/bin/sdkman-init.sh && sdk install {runtime} {version}"),
                ],
            )?;
        }
        "pyenv" => {
            run_command("pyenv", &["install", version])?;
        }
        "rbenv" => {
            run_command("rbenv", &["install", version])?;
        }
        _ => return Err(anyhow!("Unknown manager: {manager}")),
    }

    Ok(())
}

/// Set default runtime version
pub(super) fn set_default_runtime(runtime: &str, version: &str, manager: &str) -> Result<()> {
    match manager {
        "mise" => {
            run_command("mise", &["use", "--global", runtime, version])?;
        }
        "rustup" => {
            run_command("rustup", &["default", version])?;
        }
        "fnm" => {
            run_command("fnm", &["default", version])?;
        }
        "sdkman" => {
            run_command(
                "bash",
                &[
                    "-c",
                    &format!("source ~/.sdkman/bin/sdkman-init.sh && sdk default {runtime} {version}"),
                ],
            )?;
        }
        "pyenv" => {
            run_command("pyenv", &["global", version])?;
        }
        "rbenv" => {
            run_command("rbenv", &["global", version])?;
        }
        _ => return Err(anyhow!("Unknown manager: {manager}")),
    }

    Ok(())
}
