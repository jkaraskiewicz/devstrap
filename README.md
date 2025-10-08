# devstrap 🦀

**Production-grade Rust development environment bootstrapper** - Universal, reliable, and type-safe

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚀 Why devstrap?

A modern development environment setup tool built with Rust that prioritizes **reliability** and **ease of use**:

- **Type Safety**: Compile-time guarantees prevent runtime errors
- **Memory Safety**: No null pointer dereferences or memory leaks
- **Reliable**: Sequential installation prevents lock file conflicts
- **Single Binary**: No dependencies, just drop the binary and run
- **Better Error Handling**: Clear, actionable error messages with context
- **Maintainability**: Clean modular architecture with comprehensive tests

## ✨ Features

- 🎯 **Type-safe configuration** parsing with validation
- 🔄 **Sequential package installation** (prevents lock conflicts)
- 🎖️ **Priority-based installation** method selection
- 📦 **Multiple package managers**: brew, apt, cargo, npm, pipx, pacman, dnf
- 🚀 **Runtime version management**: Python, Node.js, Java, Kotlin, Rust, Go, Ruby, and more
- 🎨 **Framework support**: Angular, React, Vue, Android SDK
- 🔒 **Version lockfile** for reproducible installations
- 🏃 **Dry-run mode** for safe previewing
- 🖥️ **Cross-platform**: macOS and Linux
- 📊 **Beautiful CLI** with colored output
- ✅ **CI/automation support** with `--yes` flag

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

Create a `config.toml` file:

```toml
# List packages you want installed, organized in groups
# Group names are custom - name them whatever you want!
# Groups are installed sequentially in the order they appear

[packages]
base = ["git", "curl", "wget"]
dev_tools = ["ripgrep", "bat", "fd", "fzf"]
editors = ["neovim"]
```

That's it! devstrap knows how to install each package on your system.

### 3. See Available Packages

```bash
# List all supported packages
./target/release/devstrap --list-packages
```

### 4. Run devstrap

```bash
# Preview what will be installed
./target/release/devstrap --dry-run

# Install everything
./target/release/devstrap

# Or install with auto-confirm (for CI)
./target/release/devstrap --yes
```

## 🔧 Installation

### Option 1: Build from Source

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

### Option 2: Install with Cargo

```bash
cargo install --path .
```

## 📚 Usage

### Basic Commands

```bash
# Full installation
devstrap

# Preview changes (dry-run)
devstrap --dry-run

# Auto-confirm for CI/automation
devstrap --yes

# List all available packages
devstrap --list-packages

# Use custom config file
devstrap --config /path/to/config.toml
```

### CLI Options

```
Options:
  -c, --config <CONFIG>  Path to config.toml file [default: config.toml]
      --dry-run          Dry run - show what would be done without making changes
      --list-packages    List all available packages and exit
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

# Install with verbose output for debugging
devstrap --verbose

# Discover available packages
devstrap --list-packages
```

#### CI/Automation

```bash
# Non-interactive installation
devstrap --yes

# Use different config for CI
devstrap --config ci-config.toml --yes
```

#### Selective Installation

Simply comment out groups in your `config.toml`:

```toml
[packages]
base = ["git", "ripgrep", "bat"]
# editors = ["neovim", "vim"]  # Skip editors
dev_tools = ["fzf", "fd"]
```

## 📝 Configuration Guide

### Package Groups

Your config file only needs one section: `[packages]`.

**Group names are completely custom** - name them whatever you want:

```toml
[packages]
base = ["git", "curl", "wget"]
dev_tools = ["ripgrep", "bat", "fd", "fzf"]
editors = ["neovim", "vim"]
my_awesome_tools = ["tmux", "htop", "jq"]
```

- Groups are installed **sequentially** in the order they appear
- Packages within each group are also installed sequentially
- devstrap automatically chooses the best installation method for each package

### Discovering Packages

See all available packages:

```bash
devstrap --list-packages
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

### Supported Frameworks

- **Angular** (via npm)
- **React** (create-react-app via npm)
- **Vue** (Vue CLI via npm)
- **Android SDK** (coming soon)

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

#### Framework Configuration

```toml
[frameworks]
angular = "latest"          # Latest Angular CLI

[frameworks.android]
sdk = "latest"
build-tools = "34.0.0"
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
# Package installation
[packages]
base = ["git", "curl", "wget"]
dev_tools = ["ripgrep", "bat", "fzf"]

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

# Frameworks
[frameworks]
angular = "latest"
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
