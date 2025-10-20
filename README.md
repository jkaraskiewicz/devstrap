# devstrap 🦀

**Universal development environment bootstrapper** - Universal, reliable, and type-safe

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚀 Why devstrap?

A modern, declarative development environment setup tool that prioritizes **reliability** and **ease of use**:

- **Type Safety**: Compile-time guarantees prevent runtime errors
- **Memory Safety**: No null pointer dereferences or memory leaks
- **Reliable**: Sequential installation prevents lock file conflicts
- **Single Binary**: No dependencies, just drop the binary and run
- **Better Error Handling**: Clear, actionable error messages with context
- **Maintainability**: Clean modular architecture with comprehensive tests

## ✨ Features

- 🔄 **Declarative sync model** - Config as single source of truth
- 🎯 **Type-safe configuration** parsing with validation
- 🗑️ **Safe pruning** - Remove packages not in config (only devstrap-installed)
- 🔄 **Sequential package installation** (prevents lock conflicts)
- 🎖️ **Priority-based installation** method selection
- 📦 **Multiple package managers**: brew, apt, cargo, npm, pipx, pacman, dnf
- 🚀 **Runtime version management**: Python, Node.js, Java, Kotlin, Rust, Go, Ruby, and more
- 🔒 **Version lockfile** for reproducible installations
- 📊 **State tracking** - Know what devstrap installed vs user-installed
- 🏃 **Dry-run mode** for safe previewing
- 🖥️ **Cross-platform**: macOS and Linux
- 💅 **Beautiful CLI** with colored output
- ✅ **CI/automation support** with `--yes` flag

## 📚 Table of Contents

- [Why devstrap?](#-why-devstrap)
- [Features](#-features)
- [Quick Start](#-quick-start)
- [Tutorial: Common Scenarios](#-tutorial-common-scenarios)
  - [Setting Up a New Machine](#scenario-1-setting-up-a-new-machine)
  - [Cleaning Up Unused Packages](#scenario-2-cleaning-up-unused-packages)
  - [Team Onboarding](#scenario-3-team-onboarding)
  - [CI/CD Integration](#scenario-4-cicd-integration)
  - [Safely Previewing Changes](#scenario-5-safely-previewing-changes)
- [Installation](#-installation)
- [Usage](#-usage)
- [Configuration Guide](#-configuration-guide)
- [Runtime & Framework Management](#-runtime--framework-management)
- [Architecture](#-architecture)
- [Testing](#-testing)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)

## 📋 Requirements

- Rust 1.75 or higher (for building from source)
- OR: Just download the pre-built binary

## 🎮 Quick Start

### 1. Build the Binary

```bash
git clone https://github.com/jkaraskiewicz/devstrap
cd devstrap
cargo build --release
```

The optimized binary will be at `target/release/devstrap` (~2-4 MB).

### 2. Create Your Configuration

Create a `config.toml` file with a simple flat array:

```toml
# Simple flat array - packages installed in order
packages = ["git", "curl", "ripgrep", "bat", "fzf"]
```

**Or** use nested arrays for explicit ordering:

```toml
# Nested arrays - install first group, then second group
packages = [
    ["git", "curl"],           # Install these first
    ["ripgrep", "bat", "fzf"]  # Then install these
]
```

That's it! devstrap knows how to install each package on your system.

### 3. See Available Packages

```bash
# List all supported packages
./target/release/devstrap list
```

### 4. Run devstrap

```bash
# Preview what will be installed/changed
./target/release/devstrap --dry-run

# Sync your system with the config
./target/release/devstrap

# Or sync with auto-confirm (for CI)
./target/release/devstrap --yes

# Remove packages not in config
./target/release/devstrap --prune

# Update lockfile to latest versions
./target/release/devstrap --refresh
```

## 📖 Tutorial: Common Scenarios

New to devstrap? Follow these step-by-step guides for common real-world scenarios:

### Scenario 1: Setting Up a New Machine

You just got a new laptop and need your complete development environment.

**Step 1:** Install devstrap
```bash
git clone https://github.com/jkaraskiewicz/devstrap
cd devstrap
cargo build --release
cp target/release/devstrap ~/.local/bin/
```

**Step 2:** Create your config
```bash
cat > ~/config.toml <<EOF
packages = [
    ["git", "curl"],
    ["ripgrep", "fd", "bat", "fzf"],
    ["cmake", "make"]
]
EOF
```

**Step 3:** Preview and install
```bash
# Always preview first!
devstrap --config ~/config.toml --dry-run

# Install everything
devstrap --config ~/config.toml --yes
```

**Result:** All your tools installed and ready to use! ✅

---

### Scenario 2: Cleaning Up Unused Packages

You've been trying different tools and want to remove the ones you don't use anymore.

**Current config:**
```toml
packages = [["git", "curl", "ripgrep", "fd", "bat", "fzf", "jq", "htop"]]
```

**Updated config (removed jq and htop):**
```toml
packages = [["git", "curl", "ripgrep", "fd", "bat", "fzf"]]
```

**Remove unused packages:**
```bash
# Preview what will be removed
devstrap sync --prune --dry-run

# Execute removal
devstrap sync --prune --yes
```

**Safety:** `--prune` only removes packages that devstrap installed. Your manually installed packages are never touched! ✅

---

### Scenario 3: Team Onboarding

Your team needs standardized development environments.

**Team lead creates `devstrap-config.toml`:**
```toml
# Team Development Environment
packages = [
    ["git", "curl"],
    ["ripgrep", "fd", "fzf"],
    ["cmake", "make"]
]

[system_languages]
c = true
cpp = true
```

**New developer setup:**
```bash
# Clone project
git clone https://github.com/company/project
cd project

# Install devstrap
curl -sSL https://your-company.com/install-devstrap.sh | bash

# One command setup!
devstrap --config devstrap-config.toml --yes
```

**Everyone gets identical environments!** ✅

---

### Scenario 4: CI/CD Integration

Set up devstrap in your GitHub Actions workflow.

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup environment with devstrap
        run: |
          # Install devstrap
          curl -sSL https://get-devstrap.sh | bash

          # Non-interactive setup
          devstrap --config .devstrap/ci-config.toml --yes

      - name: Run tests
        run: make test
```

**CI benefits:**
- Reproducible environments ✅
- Fast with caching ✅
- Same tools as local development ✅

---

### Scenario 5: Safely Previewing Changes

Before making any changes, use `--dry-run` to see what will happen.

```bash
# Preview installing new tools
devstrap --config new-config.toml --dry-run

# Preview removing packages
devstrap sync --prune --dry-run

# Preview version updates
devstrap sync --refresh --dry-run
```

**Sample dry-run output:**
```
⚠ DRY RUN MODE - No changes will be made

Sync Plan:
  ✓ To install:
    • ripgrep
    • fd

[DRY-RUN] Would run: sudo apt-get update
[DRY-RUN] Would install ripgrep
[DRY-RUN] Would install fd
```

**Always dry-run first!** ✅

---

### Quick Tips

**Ubuntu/Debian Users:**
Some packages have different names:
- `fd` → `fdfind`
- `bat` → `batcat`

This is normal! The packages install correctly, just use the Ubuntu binary names.

**State File Location:**
For `--prune` to work, devstrap needs to track installations. The state file is created at:
- `~/.config/devstrap/devstrap.state` (default)
- Or in the same directory as your config file

Ensure your config is in a writable location!

**Lockfile for Teams:**
Commit `devstrap.lock` to version control:
```bash
git add devstrap.lock config.toml
git commit -m "chore: Lock development tool versions"
```

This ensures everyone gets the same package versions!

---

### Need Help?

- Check the [Troubleshooting](#-troubleshooting) section below for common issues
- Run `devstrap --verbose` for detailed debugging output
- Use `devstrap --dry-run` to preview changes safely
- See [ROADMAP.md](ROADMAP.md) for planned features and future enhancements

---

## 🔧 Installation

### Option 1: Download Pre-Built Binary (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/jkaraskiewicz/devstrap/releases):

**macOS (Apple Silicon - M1/M2/M3):**
```bash
curl -LO https://github.com/jkaraskiewicz/devstrap/releases/latest/download/devstrap-v2.0.0-aarch64-apple-darwin.tar.gz
tar xzf devstrap-v2.0.0-aarch64-apple-darwin.tar.gz
sudo mv devstrap-v2.0.0-aarch64-apple-darwin/devstrap /usr/local/bin/
```

**macOS (Intel):**
```bash
curl -LO https://github.com/jkaraskiewicz/devstrap/releases/latest/download/devstrap-v2.0.0-x86_64-apple-darwin.tar.gz
tar xzf devstrap-v2.0.0-x86_64-apple-darwin.tar.gz
sudo mv devstrap-v2.0.0-x86_64-apple-darwin/devstrap /usr/local/bin/
```

**Linux (x86_64):**
```bash
curl -LO https://github.com/jkaraskiewicz/devstrap/releases/latest/download/devstrap-v2.0.0-x86_64-unknown-linux-musl.tar.gz
tar xzf devstrap-v2.0.0-x86_64-unknown-linux-musl.tar.gz
sudo mv devstrap-v2.0.0-x86_64-unknown-linux-musl/devstrap /usr/local/bin/
```

**Windows (x86_64):**

Download the `.zip` file from [releases](https://github.com/jkaraskiewicz/devstrap/releases), extract it, and add `devstrap.exe` to your PATH.

**Verify installation:**
```bash
devstrap --version
```

### Option 2: Build from Source

```bash
# Clone and build
git clone https://github.com/jkaraskiewicz/devstrap
cd devstrap
cargo build --release

# Install to ~/.local/bin
mkdir -p ~/.local/bin
cp target/release/devstrap ~/.local/bin/

# Or install system-wide
sudo cp target/release/devstrap /usr/local/bin/
```

### Option 3: Install with Cargo

```bash
cargo install --path .
```

## 📚 Usage

### Basic Commands

```bash
# Sync system with config (default command)
devstrap
devstrap sync

# Preview changes (dry-run)
devstrap --dry-run

# Auto-confirm for CI/automation
devstrap --yes

# Remove packages not in config
devstrap sync --prune

# Update lockfile to latest versions
devstrap sync --refresh

# Combine flags
devstrap sync --prune --refresh --yes

# List all available packages
devstrap list

# Use custom config file
devstrap --config /path/to/config.toml
```

### Sync Model

devstrap uses a **declarative sync model** similar to Terraform or Nix. Your `config.toml` is the single source of truth, and devstrap makes your system match it.

**Three Files System:**

1. **config.toml** - What you WANT (desired state)
2. **devstrap.lock** - Resolved runtime versions (reproducibility)
3. **devstrap.state** - What devstrap HAS INSTALLED (tracking)

**How Sync Works:**

```bash
# devstrap calculates the diff:
# - What needs to be installed (in config but not installed)
# - What can be removed (installed by devstrap but not in config)

devstrap sync
```

**Sync Flags:**

```bash
# --prune: Remove packages not in config
# Only removes packages that devstrap installed (safe)
devstrap sync --prune

# --refresh: Update lockfile to latest versions
# Re-resolves "latest", "lts", "stable" to actual newest versions
devstrap sync --refresh

# Combine both for a complete refresh
devstrap sync --prune --refresh
```bash
# Update all packages and runtimes to latest versions
devstrap update

# Update a specific package
devstrap update ripgrep

# Update a specific runtime
devstrap update node

# Preview update changes (dry-run)
devstrap update --dry-run

# Auto-confirm updates for CI
devstrap update --yes
```

**How Updates Work:**

- **Packages**: Uses the same installation method to upgrade to the latest version available from the package manager
- **Runtimes**: Clears the version lockfile and re-resolves to the latest versions based on your config.toml (e.g., "latest", "lts", "stable")
- **Lockfile**: After updating runtimes, devstrap.lock is regenerated with the new resolved versions

**Update Best Practices:**

```bash
# Always preview updates first
devstrap update --dry-run

# Review what will change, then run
devstrap update

# Update specific tools when needed
devstrap update fzf
devstrap update python
```

### CLI Options

```
Commands:
  sync                 Synchronize system with config (default behavior)
    --prune            Remove packages not in config (use with caution)
    --refresh          Update lockfile to actual latest versions
  list                 List all available packages
  help                 Print help message

Global Options:
  -c, --config <CONFIG>  Path to config.toml file [default: config.toml]
      --dry-run          Dry run - show what would be done without making changes
  -v, --verbose          Verbose output
  -y, --yes              Skip confirmation prompts (for CI/automated environments)
  -h, --help             Print help
  -V, --version          Print version
```

### Common Workflows

#### Development Setup

```bash
# Preview changes
devstrap --dry-run

# Sync with verbose output for debugging
devstrap --verbose

# Discover available packages
devstrap list

# Refresh to latest versions
devstrap sync --refresh --dry-run  # Preview first
devstrap sync --refresh            # Then update lockfile
```

#### CI/Automation

```bash
# Non-interactive sync
devstrap --yes

# Sync with pruning in CI
devstrap sync --prune --yes

# Use different config for CI
devstrap --config ci-config.toml --yes

# Full refresh in CI
devstrap sync --refresh --config ci-config.toml --yes
```

#### Managing Packages

```bash
# Add packages: Edit config.toml and add them to the array
packages = ["git", "curl", "ripgrep", "bat", "NEW_PACKAGE"]

# Then sync
devstrap sync

# Remove packages: Delete from config.toml, then prune
packages = ["git", "curl"]  # removed ripgrep and bat

devstrap sync --prune
```

## 📝 Configuration Guide

### Package Configuration

Your config file uses a simple `packages` array:

**Simple Format (Flat Array):**
```toml
# Packages installed in order
packages = ["git", "curl", "ripgrep", "bat", "fzf"]
```

**Advanced Format (Nested Arrays):**
```toml
# Groups installed sequentially
# Useful for explicit ordering (e.g., dependencies first)
packages = [
    ["git", "curl"],           # Core tools first
    ["ripgrep", "bat", "fzf"]  # Then search/view tools
]
```

- Packages are installed **sequentially** in the order they appear
- devstrap automatically chooses the best installation method for each package
- Both formats work identically - use whichever fits your workflow

### Discovering Packages

See all available packages:

```bash
devstrap list
```

This shows all built-in packages that devstrap knows how to install.

### Installation Method Priority

devstrap automatically selects the best installation method:

1. **System Default Package Manager** (priority 10)
   - APT on Ubuntu/Debian
   - Homebrew on macOS
   - DNF on Fedora
   - Pacman on Arch

2. **NPM** (priority 8)
3. **Cargo** (priority 6)
4. **Pipx** (priority 4)
5. **System** (priority 2) - Already installed
6. **GitHub** (priority 1) - Download from releases

### Side-by-Side Installation

If a package is already installed via system packages, devstrap will install it alongside using the preferred method without removing the system version:

```
↻ git (installing via APT alongside system version)
```

## 🎯 Runtime & Framework Management

devstrap now supports automatic installation and version management for programming language runtimes and frameworks!

### Supported Runtimes

- **Python** (via mise/pyenv)
- **Node.js** (via fnm/mise)
- **Java & Kotlin** (via SDKMAN)
- **Rust** (via rustup)
- **Go** (via mise)
- **Ruby** (via rbenv/mise)
- **TypeScript** (via npm)
- **System languages**: C, C++ (gcc, g++, clang)

### Version Shortcuts

Instead of specifying exact versions, use these convenient shortcuts:

- `"latest"` - Latest stable version
- `"lts"` - Latest Long Term Support version (Node.js, Java)
- `"stable"` - Stable release channel (Rust)
- `"nightly"` / `"beta"` - Pre-release channels
- Specific versions like `"3.11.0"`, `"20.10.0"`

### Configuration Examples

#### Simple Runtime Configuration

```toml
[runtimes]
python = "latest"           # Latest Python version
node = "lts"                # Latest LTS Node.js
rust = "stable"             # Rust stable channel
go = "latest"               # Latest Go version
ruby = "3.2.0"              # Specific Ruby version
```

#### Advanced Runtime Configuration

```toml
# Install multiple Java versions with a default
[runtimes.java]
versions = ["17", "21"]     # Install both Java 17 and 21
default = "21"              # Use Java 21 as default
manager = "sdkman"          # Explicitly use SDKMAN

# TypeScript requires Node.js
[runtimes.typescript]
version = "latest"
requires = "node"
```

#### System Languages

```toml
# Install compilers via system package manager
[system_languages]
c = true
cpp = true
clang = true
```

### Version Lockfile

devstrap creates a `devstrap.lock` file to pin resolved versions:

```toml
[runtimes.python]
requested = "latest"
resolved = "3.12.0"
manager = "mise"
resolved_at = "2025-01-08T10:30:00Z"

[runtimes.node]
requested = "lts"
resolved = "20.10.0"
manager = "fnm"
resolved_at = "2025-01-08T10:30:05Z"
```

This ensures everyone on your team gets the exact same versions.

### Complete Configuration Example

```toml
# Package installation - simple flat array
packages = ["git", "curl", "ripgrep", "bat", "fzf"]

# Or use nested arrays for explicit ordering
# packages = [
#     ["git", "curl"],
#     ["ripgrep", "bat", "fzf"]
# ]

# Runtime versions
[runtimes]
python = "latest"
node = "lts"
rust = "stable"

[runtimes.java]
versions = ["17", "21"]
default = "21"

# System compilers
[system_languages]
c = true
cpp = true
```

## 🏗️ Architecture

### Project Structure

```
devstrap/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── cli.rs               # CLI argument parsing
│   ├── init.rs              # App initialization
│   ├── config/
│   │   ├── mod.rs           # Module exports
│   │   ├── structs.rs       # Config structs
│   │   └── parse.rs         # Config parsing & validation
│   ├── installer/
│   │   ├── mod.rs           # Module exports
│   │   ├── core.rs          # Installer coordinator
│   │   └── methods.rs       # Package installation methods
│   ├── detect/
│   │   ├── mod.rs           # Module exports
│   │   ├── enums.rs         # OS/Arch/PM enums
│   │   └── system.rs        # System detection logic
│   ├── package.rs           # Package types and traits
│   ├── ui.rs                # UI/banner display
│   ├── error.rs             # Error types
│   └── utils.rs             # Utilities
├── tests/                   # Integration tests
├── Cargo.toml
└── README.md
```

### Key Design Principles

1. **Lean mod.rs files**: Module files only contain declarations and re-exports
2. **Separation of concerns**: Each module has a specific responsibility
3. **Sequential execution**: Prevents package manager lock conflicts
4. **Type safety**: Extensive use of Rust's type system
5. **Error handling**: Comprehensive error types with context

## 🧪 Testing

### Run Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_installer_creation
```

### Docker Testing

Test in an isolated environment:

```bash
# Build test container
docker build -f tests/fixtures/Dockerfile.test-root -t devstrap-test .

# Run tests
docker run --rm devstrap-test /bin/bash -c \
  "apt-get update > /dev/null 2>&1 && \
   /root/devstrap/target/release/devstrap \
   --config tests/fixtures/test-config.toml --yes"
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check without building
cargo check
```

## 🔄 Cross-Platform Support

### macOS

```bash
# Build for current architecture
cargo build --release

# Build universal binary (Intel + Apple Silicon)
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create \
    target/x86_64-apple-darwin/release/devstrap \
    target/aarch64-apple-darwin/release/devstrap \
    -output devstrap-universal
```

### Linux

```bash
# x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# ARM64
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

## 🐛 Troubleshooting

### Build Issues

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Runtime Issues

```bash
# Enable verbose logging
RUST_LOG=debug devstrap --verbose

# Check system detection
devstrap --dry-run

# Verify config syntax
devstrap --config config.toml --dry-run
```

### Common Issues

**Package installation fails:**
- Check if the package manager is installed
- Verify package name is correct for your system
- Try with `--verbose` for detailed error messages

**Permission denied:**
- Run with appropriate permissions (sudo for system package managers)
- Check file permissions on dotfiles

**Config parsing errors:**
- Validate TOML syntax
- Ensure all required fields are present
- Check package names in groups are defined

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with tests
4. Run quality checks:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```
5. Commit changes (`git commit -m 'Add amazing feature'`)
6. Push to branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Code Quality Standards

- ✅ All functions documented with rustdoc
- ✅ Comprehensive error handling (no `.unwrap()` in production)
- ✅ Unit tests for core logic
- ✅ Zero clippy warnings with pedantic rules
- ✅ Formatted with rustfmt
- ✅ Keep mod.rs files lean

## 📝 License

MIT License - see LICENSE file for details

## 🙏 Acknowledgments

- Rust community for excellent tooling and crates
- All open-source package managers supported
- Contributors and testers

## 🔗 Useful Links

- [Rust Programming Language](https://www.rust-lang.org/)
- [clap - CLI framework](https://docs.rs/clap/)
- [serde - Serialization](https://docs.rs/serde/)
- [colored - Terminal colors](https://docs.rs/colored/)

---

**Made with 🦀 and ❤️**
