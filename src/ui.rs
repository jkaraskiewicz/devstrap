//! UI display utilities
//!
//! Handles banners, system information printing, and user prompts.

use crate::detect::SystemInfo;
use colored::Colorize;

/// Show ASCII banner
pub fn show_banner() {
    let banner = r"
    __           __      __
   / /___  _  __/ /_  __/ /________ _____
  / / __ \| |/_/ / / / / /  / ___/ /  __ `/ __ \
 / / /_/ />  </ / /_/ / /  (__  ) / /_/ / /_/ /
/ /\____/_/|_/_/\__,_/_/  /____/_/\__,_/ .___/
                                      /_/
        Universal Dev Environment Setup v2.0
        Blazing fast • Type safe • Reliable
    ";

    println!("{}", banner.cyan().bold());
}

/// Print system information
pub fn print_system_info(info: &SystemInfo) {
    println!("\n{}", "System Information:".bold());
    println!("  OS:               {:?}", info.os);
    println!("  Distribution:     {:?}", info.distro);
    println!("  Architecture:     {:?}", info.arch);
    println!(
        "  Package Manager:  {}",
        info.default_package_manager
            .map_or("None", |pm| pm.display_name())
    );

    if info.is_apple_silicon {
        println!("  {} Apple Silicon detected", "⚡".yellow());
    }

    if info.is_wsl {
        println!("  {} Running in WSL", "🐧".cyan());
    }

    println!(
        "  Available tools:  {}",
        info.available_package_managers
            .iter()
            .map(super::detect::enums::PackageManager::display_name)
            .collect::<Vec<_>>()
            .join(", ")
    );
}

/// Prompt user for confirmation
#[must_use]
pub fn confirm(prompt: &str) -> bool {
    use std::io::{self, Write};

    print!("{} [y/N]: ", prompt.bold());
    // Ignore flush errors - if stdout fails, we continue anyway
    let _ = io::stdout().flush();

    let mut input = String::new();
    // If we can't read input, default to "no" for safety
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Show post-installation instructions
pub fn show_post_install_instructions() {
    let instructions = r"
📝 Next Steps:

1. Restart your shell or run:
    exec zsh

2. Install language versions (if needed):
    pyenv install <version> && pyenv global <version>
    nvm install <version> && nvm use <version> && nvm alias default <version>
    sdk install java <version>
    ruby-install ruby <version>

3. Configure API keys in ~/.env.local and source it from ~/.zshrc:
    [ -f ~/.env.local ] && source ~/.env.local

4. Copy SSH keys to ~/.ssh/ if needed

5. Review your dotfiles and customize as needed

6. To update tools later:
    cargo install --force <tool>
    npm update -g
    rustup update

For more information, see the README.md file.

Happy coding! 🚀
    ";

    println!("{instructions}");
}
