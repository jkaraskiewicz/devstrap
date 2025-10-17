//! System language installation (gcc, clang, etc.)

use crate::common::error::Result;
use crate::common::run_command;
use colored::Colorize;

/// Install system-provided languages
pub fn install_system_languages(
    system_languages: &std::collections::HashMap<String, bool>,
    dry_run: bool,
) -> Result<()> {
    if system_languages.is_empty() {
        return Ok(());
    }

    let enabled_langs: Vec<String> = system_languages
        .iter()
        .filter(|(_, &enabled)| enabled)
        .map(|(name, _)| name.clone())
        .collect();

    if enabled_langs.is_empty() {
        return Ok(());
    }

    println!(
        "\n{} {}",
        "Installing system languages:".bold().cyan(),
        enabled_langs.join(", ")
    );

    for lang in &enabled_langs {
        install_system_language(lang, dry_run)?;
    }

    Ok(())
}

/// Install a single system language
fn install_system_language(lang: &str, dry_run: bool) -> Result<()> {
    let packages = match lang {
        "c" | "gcc" => vec!["gcc", "build-essential"],
        "cpp" | "g++" => vec!["g++", "build-essential"],
        "clang" => vec!["clang"],
        _ => vec![lang],
    };

    for pkg in packages {
        println!("  {} {}", "↓".cyan(), pkg);
        if !dry_run {
            // Use system package manager
            if which::which("apt").is_ok() {
                run_command("sudo", &["apt", "install", "-y", pkg])?;
            } else if which::which("brew").is_ok() {
                run_command("brew", &["install", pkg])?;
            } else if which::which("dnf").is_ok() {
                run_command("sudo", &["dnf", "install", "-y", pkg])?;
            } else if which::which("pacman").is_ok() {
                run_command("sudo", &["pacman", "-S", "--noconfirm", pkg])?;
            }
        }
    }

    Ok(())
}
