# devstrap - Future Plans & Ideas

## Runtime & Framework Management

### Version Range Support (Future)
- [ ] Support semantic version ranges like `"^20"` or `">=3.11,<4.0"`
- [ ] Allow `"latest-1"` for "one version behind latest"
- [ ] Support constraint expressions: `">=18 <21"`, `"~3.11"`

### Additional Runtimes/Frameworks
- [ ] PHP (version manager: phpenv or mise)
- [ ] Elixir (version manager: mise or kiex)
- [ ] Scala (via SDKMAN)
- [ ] Dart/Flutter
- [ ] .NET/C# (dotnet CLI)
- [ ] Zig
- [ ] Haskell (ghcup)
- [ ] OCaml (opam)

### Framework Support Expansion
- [ ] React Native (requires Node + Android/iOS setup)
- [ ] Vue.js ecosystem
- [ ] Spring Boot (via SDKMAN)
- [ ] Django/Flask (Python frameworks)
- [ ] Rails (Ruby framework)
- [ ] Docker & container runtimes

### Lockfile Enhancements
- [ ] Add `devstrap update` command to refresh lockfile
- [ ] Support selective updates: `devstrap update python`
- [ ] Show diff when lockfile changes
- [ ] Validate lockfile against current installations

## Package Management

### Enhanced Package Features
- [ ] Pre/post installation hooks per package
- [ ] Conditional package installation based on OS/architecture
- [ ] Package aliases and virtual packages
- [ ] Package dependency resolution
- [ ] Parallel package installation (with proper locking)

### Installation Methods
- [ ] Support for snap packages (Linux)
- [ ] Support for flatpak (Linux)
- [ ] Support for scoop (Windows via WSL)
- [ ] Support for chocolatey (Windows)
- [ ] AppImage support
- [ ] Binary release downloads from GitHub

## Dotfiles & Configuration

### Dotfile Management
- [ ] Encrypted dotfile support (for secrets)
- [ ] Dotfile templating (OS-specific variations)
- [ ] Selective dotfile installation
- [ ] Dotfile backup before overwriting
- [ ] Git-based dotfile syncing

### Shell Integration
- [ ] Fish shell support
- [ ] PowerShell support
- [ ] Nu shell support
- [ ] Automatic shell detection and configuration

## Developer Experience

### CLI Improvements
- [ ] Interactive mode for configuration creation
- [ ] Wizard for initial setup
- [ ] `devstrap doctor` command to diagnose issues
- [ ] `devstrap search <package>` to find available packages
- [ ] `devstrap info <package>` for detailed package information

### Performance & Reliability
- [ ] Installation caching to avoid re-downloads
- [ ] Resume interrupted installations
- [ ] Rollback on installation failure
- [ ] Health checks after installation
- [ ] Benchmark mode to compare installation times

### Configuration
- [ ] Support for multiple config files (merge configs)
- [ ] Config inheritance (base config + overrides)
- [ ] Environment-specific configs (dev/prod)
- [ ] Config validation with detailed error messages

## Platform Support

### Operating Systems
- [ ] Windows native support (without WSL)
- [ ] FreeBSD support
- [ ] Alpine Linux support
- [ ] NixOS integration

### Architecture
- [ ] RISC-V support
- [ ] Better ARM/ARM64 detection and handling

## Integrations

### CI/CD
- [ ] GitHub Actions integration
- [ ] GitLab CI templates
- [ ] CircleCI orbs
- [ ] Docker image with devstrap pre-installed

### IDE & Editors
- [ ] VS Code extension for config editing
- [ ] IntelliJ plugin
- [ ] Config schema for autocomplete

### Cloud & Containers
- [ ] Dockerfile generation from config
- [ ] Cloud-init script generation
- [ ] Vagrant provisioner
- [ ] Ansible playbook generation

## Documentation & Community

### Documentation
- [ ] Interactive documentation website
- [ ] Video tutorials
- [ ] Example configurations repository
- [ ] Migration guides from other tools

### Community
- [ ] Plugin system for custom installers
- [ ] Community package registry
- [ ] Config sharing platform
- [ ] Template marketplace

## Testing & Quality

### Test Coverage
- [ ] Integration tests for all package managers
- [ ] Cross-platform testing matrix
- [ ] Performance regression tests
- [ ] Security audit of installation scripts

### Code Quality
- [ ] Reduce all functions to <30 lines (currently <50)
- [ ] Increase test coverage to >90%
- [ ] Add property-based testing
- [ ] Fuzz testing for config parsing

## Security

### Security Features
- [ ] Checksum verification for downloads
- [ ] GPG signature verification
- [ ] Supply chain security scanning
- [ ] Sandboxed installation option
- [ ] Security audit log

### Compliance
- [ ] SBOM (Software Bill of Materials) generation
- [ ] License compliance checking
- [ ] Vulnerability scanning integration

## Other Ideas

- [ ] Remote configuration support (fetch from URL)
- [ ] Configuration inheritance from templates
- [ ] Profile switching (work/personal/project-specific)
- [ ] Shell history integration (save common commands)
- [ ] Automatic dependency detection from project files
- [ ] Integration with mise/asdf existing configs
- [ ] Homebrew tap for easier installation
- [ ] Snap/Flatpak packaging for devstrap itself
