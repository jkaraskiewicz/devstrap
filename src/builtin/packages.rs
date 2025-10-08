//! Package definitions

use crate::config::PackageConfig;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Built-in package definitions
pub static BUILTIN_PACKAGES: Lazy<HashMap<&'static str, PackageConfig>> = Lazy::new(|| {
    let mut packages = HashMap::new();

    // Core utilities
    packages.insert("git", PackageConfig::new().description("Distributed version control system").name("git"));
    packages.insert("curl", PackageConfig::new().description("Command-line tool for transferring data with URLs").name("curl"));
    packages.insert("wget", PackageConfig::new().description("Non-interactive network downloader").name("wget"));

    // Modern CLI tools
    packages.insert("ripgrep", PackageConfig::new().description("Fast text search tool (better grep)").name("ripgrep").cargo("ripgrep"));
    packages.insert("bat", PackageConfig::new().description("Cat clone with syntax highlighting").name("bat").cargo("bat"));
    packages.insert("fd", PackageConfig::new().description("Fast file finder (better find)").name("fd-find").cargo("fd-find").brew("fd").apt("fd-find").pacman("fd").dnf("fd-find"));
    packages.insert("fzf", PackageConfig::new().description("Fuzzy finder for command line").name("fzf"));
    packages.insert("eza", PackageConfig::new().description("Modern replacement for ls").name("eza").cargo("eza"));
    packages.insert("zoxide", PackageConfig::new().description("Smarter cd command with directory history").name("zoxide").cargo("zoxide"));

    // Editors
    packages.insert("neovim", PackageConfig::new().description("Hyperextensible Vim-based text editor").name("neovim"));
    packages.insert("vim", PackageConfig::new().description("Vi IMproved - enhanced vi editor").name("vim"));
    packages.insert("emacs", PackageConfig::new().description("Extensible, customizable text editor").name("emacs"));

    // Terminal multiplexers
    packages.insert("tmux", PackageConfig::new().description("Terminal multiplexer for managing multiple sessions").name("tmux"));
    packages.insert("screen", PackageConfig::new().description("Terminal multiplexer with session management").name("screen"));

    // Development tools
    packages.insert("jq", PackageConfig::new().description("Lightweight command-line JSON processor").name("jq"));
    packages.insert("htop", PackageConfig::new().description("Interactive process viewer (better top)").name("htop"));
    packages.insert("btop", PackageConfig::new().description("Resource monitor with beautiful interface").name("btop"));

    // Language version managers
    packages.insert("nvm", PackageConfig::new().description("Node Version Manager for managing multiple Node.js versions").github("nvm-sh/nvm"));
    packages.insert("pyenv", PackageConfig::new().description("Python version manager for switching between versions").name("pyenv"));
    packages.insert("rbenv", PackageConfig::new().description("Ruby version manager for switching between versions").name("rbenv"));

    // Build tools
    packages.insert("make", PackageConfig::new().description("Build automation tool for compiling programs").name("make"));
    packages.insert("cmake", PackageConfig::new().description("Cross-platform build system generator").name("cmake"));

    // Node.js tools
    packages.insert("nodejs", PackageConfig::new().description("JavaScript runtime built on Chrome's V8 engine").name("nodejs").brew("node").apt("nodejs").pacman("nodejs").dnf("nodejs"));

    packages
});
