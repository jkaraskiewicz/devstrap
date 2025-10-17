//! Framework installation helpers

use crate::common::error::Result;
use crate::common::run_command;
use colored::Colorize;

/// Install a framework
pub fn install_framework(name: &str, resolved_version: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("    {} Would install {}", "[DRY-RUN]".yellow(), name);
        return Ok(());
    }

    match name {
        "angular" => {
            run_command(
                "npm",
                &["install", "-g", &format!("@angular/cli@{resolved_version}")],
            )?;
        }
        "react" => {
            run_command(
                "npm",
                &["install", "-g", &format!("create-react-app@{resolved_version}")],
            )?;
        }
        "vue" => {
            run_command(
                "npm",
                &["install", "-g", &format!("@vue/cli@{resolved_version}")],
            )?;
        }
        "android" => {
            // Android SDK installation is complex, would need separate logic
            println!(
                "    {} Android SDK installation not yet implemented",
                "⚠".yellow()
            );
        }
        _ => {
            println!("    {} Unknown framework: {}", "⚠".yellow(), name);
        }
    }

    Ok(())
}
