//! List available packages command

use crate::builtin;
use colored::Colorize;

/// Display all available packages with descriptions
pub fn list_packages() {
    println!("{}", "Available packages:".bold().cyan());
    println!("{}", "═".repeat(60).cyan());

    let package_ids = builtin::get_all_package_ids();

    for package_id in package_ids {
        if let Some(pkg) = builtin::get_package(package_id) {
            let desc = pkg
                .description
                .as_deref()
                .unwrap_or("No description available");
            println!("  • {} - {}", package_id.green().bold(), desc.dimmed());
        }
    }

    println!("\n{}", "Usage in config.toml:".bold());
    println!("  packages = [\"git\", \"ripgrep\", \"bat\"]");
    println!("  # Or nested for ordering:");
    println!("  packages = [[\"git\"], [\"ripgrep\", \"bat\"]]");
}
