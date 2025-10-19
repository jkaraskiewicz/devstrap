# devstrap Roadmap

This document outlines potential future features, enhancements, and ideas for devstrap. These are aspirational goals and community suggestions, not committed deliverables.

## Priority Legend

- 🎯 **High Priority** - Next 3-6 months, quick wins with high user impact
- 🔧 **Medium Priority** - 6-12 months, important features requiring more work
- 📋 **Low Priority** - 12+ months, nice to have but lower ROI
- ⚠️ **Reconsider** - Low value or misaligned with devstrap's core mission

## Complexity Estimates

- ⚡ **Easy** - 1-3 days
- ⚙️ **Medium** - 1-2 weeks
- 🔥 **Hard** - 3+ weeks

---

## 🎯 Phase 1: Polish Core Experience (Next 2-3 Months)

**Goal:** Improve existing features based on tutorial testing feedback. Total: ~3 weeks of focused work.

### CLI Improvements ⚡ Easy-Medium, High Impact

Tutorial testing revealed users struggle with package discovery and troubleshooting.

- [ ] 🎯 `devstrap doctor` command to diagnose issues ⚡ (2 days)
  - Check PATH configuration
  - Verify package managers installed
  - Validate state file integrity
  - Suggest fixes for common errors

- [ ] 🎯 `devstrap search <package>` to find available packages ⚡ (1 day)
  - Fuzzy search through builtin packages
  - Show package descriptions
  - Fixes "Package 'tree' not supported" confusion from testing

- [ ] 🎯 `devstrap info <package>` for detailed package information ⚡ (2 days)
  - Display package metadata
  - Show installation methods per OS
  - List runtime dependencies

- [ ] 📋 Interactive mode for configuration creation ⚙️ (1 week)
  - Wizard for initial setup
  - Template selection (web-dev, mobile, data-science, minimal)

### Configuration Enhancements ⚡ Easy, Medium Impact

Current error messages are basic; clean separation in `src/config/` makes this straightforward.

- [ ] 🎯 Config validation with detailed error messages ⚡ (3 days)
  - Use `thiserror` for detailed error types
  - Add validation module: `src/config/validation.rs`
  - Suggest fixes (e.g., "Did you mean 'ripgrep' instead of 'rg'?")

- [ ] 🎯 Support for multiple config files (merge configs) ⚡ (2 days)
  - `devstrap --config base.toml --config overrides.toml`
  - Config merging in `src/config/loader.rs`

- [ ] 📋 Config inheritance (base config + overrides) ⚙️
- [ ] 📋 Environment-specific configs (dev/prod) ⚙️

### Lockfile Enhancements ⚙️ Medium, High Impact

Lockfile already exists; testing revealed users need better workflows.

- [ ] 🎯 Add `devstrap update` command to refresh lockfile ⚙️ (5 days)
  - Add `update` subcommand to `cli.rs`
  - New usecase: `src/usecase/update.rs`
  - Reuse existing lockfile parsing

- [ ] 🎯 Show diff when lockfile changes ⚡ (2 days)
  - Color-coded diff output
  - Show version upgrades/downgrades

- [ ] 🎯 Validate lockfile against current installations ⚡ (2 days)
  - Warn if lockfile out of sync
  - Suggest `devstrap update`

- [ ] 🔧 Support selective updates: `devstrap update python` ⚙️

### Testing Infrastructure ⚙️ Medium, High Impact

Tutorial infrastructure exists; critical for v2.0 stability.

- [ ] 🎯 Integration tests for all package managers ⚙️ (1 week)
  - Extend `tests/tutorial/` approach
  - Docker containers: APT ✅, Homebrew, DNF, Pacman, YUM
  - Automated test suite in CI

- [ ] 🎯 Cross-platform testing matrix ⚙️ (3 days)
  - macOS (Homebrew)
  - Ubuntu (APT) ✅
  - Arch (Pacman)
  - Fedora (DNF)

- [ ] 📋 Performance regression tests ⚙️
- [ ] 📋 Security audit of installation scripts ⚙️

### New Ideas from Tutorial Testing

Items not in original roadmap but discovered during testing:

- [ ] 🎯 Better error recovery ⚡ (3 days)
  - Retry logic with exponential backoff
  - Suggest fixes: "Package manager cache may be stale. Run `brew update` first"
  - Tutorial testing showed package manager failures are cryptic

- [ ] 🎯 Installation progress indicators ⚡ (2 days)
  - Progress bars for multi-package installations
  - Time estimates
  - Current output is verbose but hard to track

- [ ] 🎯 Config templates via `devstrap init` ⚡ (3 days)
  - `devstrap init --template web-dev` → generates config
  - Templates: `web-dev`, `mobile-dev`, `data-science`, `minimal`
  - Tutorial showed users struggle with initial config

- [ ] 🎯 Diff preview for `--prune` ⚡ (2 days)
  - Side-by-side diff: "Will remove: fzf, jq, htop | Will keep: git, ripgrep"
  - Tutorial testing showed `--prune` is scary without preview

---

## 🔧 Phase 2: Package Management Enhancements (3-6 Months)

**Goal:** Expand package capabilities. Total: ~5 weeks.

### Enhanced Package Features ⚙️-🔥 Medium-Hard, High Impact

Tutorial revealed Ubuntu package naming issues (fd→fdfind); aliases would solve this.

- [ ] 🔧 Conditional package installation based on OS/architecture ⚙️ (1 week)
  - Requires extending `Package` domain model
  - Example: `fd = { linux = "fd-find", macos = "fd" }`

- [ ] 🔧 Package aliases and virtual packages ⚡ (3 days)
  - Solves fd→fdfind, bat→batcat confusion
  - Alias mapping in `src/builtin/packages.rs`

- [ ] 🔧 Pre/post installation hooks per package ⚙️ (1 week)
  - Hook execution needs security consideration
  - Example: `post_install = ["npm install -g typescript"]`

- [ ] 📋 Package dependency resolution ⚙️
  - Automatic installation of dependencies
  - Topological sort for install order

- [ ] 📋 Parallel package installation (with proper locking) 🔥
  - Conflicts with current sequential design
  - Requires significant refactoring

### Installation Methods ⚙️-🔥 Medium-Hard, Medium-High Impact

Many tools ship binaries on GitHub (ripgrep, fd, bat); would reduce package manager dependency.

- [ ] 🔧 Binary release downloads from GitHub ⚙️ (2 weeks)
  - Parse GitHub releases API
  - Architecture detection (x64/ARM64)
  - Checksum verification

- [ ] 🔧 Support for snap packages (Linux) ⚙️
- [ ] 🔧 Support for flatpak (Linux) ⚙️
- [ ] 📋 Support for scoop (Windows via WSL) ⚙️
- [ ] 📋 Support for chocolatey (Windows) ⚙️
- [ ] 📋 AppImage support ⚙️

---

## 🔧 Phase 3: Runtime Expansion (6-12 Months)

**Goal:** Support more programming languages. Total: ~3 weeks.

### Additional Runtimes ⚙️ Medium per runtime, High cumulative impact

Each runtime follows same pattern as existing ones in `src/domain/runtime/`.

#### High Priority Runtimes

- [ ] 🔧 PHP (version manager: phpenv or mise) ⚙️ (1 week)
  - Popular for web development
  - Use mise for version management

- [ ] 🔧 .NET/C# (dotnet CLI) ⚙️ (1 week)
  - Enterprise demand
  - Official dotnet installer

- [ ] 🔧 Zig ⚡ (3 days)
  - Growing adoption
  - Simple installation via mise

#### Lower Priority Runtimes

- [ ] 📋 Elixir (version manager: mise or kiex) ⚙️
- [ ] 📋 Scala (via SDKMAN) ⚙️
- [ ] 📋 Dart/Flutter ⚙️
- [ ] 📋 Haskell (ghcup) ⚙️
- [ ] 📋 OCaml (opam) ⚙️

### Version Range Support 🔥 Hard, Medium Impact

Complex semantic versioning logic.

- [ ] 📋 Support semantic version ranges like `"^20"` or `">=3.11,<4.0"` 🔥
- [ ] 📋 Allow `"latest-1"` for "one version behind latest" ⚙️
- [ ] 📋 Support constraint expressions: `">=18 <21"`, `"~3.11"` 🔥

---

## 📋 Phase 4: Advanced Features (12+ Months)

### Framework Support Expansion ⚙️-🔥 Medium-Hard, Low-Medium Impact

**Recommendation:** Focus on runtimes (Node/Python/Ruby) working perfectly; let framework tools handle the rest.

- [ ] 📋 React Native (requires Node + Android/iOS setup) 🔥
  - Massive scope: Android SDK, iOS, Xcode
  - Better served by runtime + `npm install` workflows

- [ ] 📋 Vue.js ecosystem ⚙️
- [ ] 📋 Spring Boot (via SDKMAN) ⚙️
- [ ] 📋 Django/Flask (Python frameworks) ⚡
- [ ] 📋 Rails (Ruby framework) ⚡
- [ ] 📋 Docker & container runtimes ⚙️

### Performance & Reliability ⚙️ Medium, Medium Impact

- [ ] 📋 Installation caching to avoid re-downloads ⚙️
  - Cache directory: `~/.cache/devstrap/`
  - Checksum verification

- [ ] 📋 Resume interrupted installations ⚙️
  - Save installation state
  - Detect partial installs

- [ ] 📋 Rollback on installation failure ⚙️
  - Snapshot before changes
  - Restore on error

- [ ] 📋 Health checks after installation ⚡
  - Verify binaries in PATH
  - Run `--version` checks

- [ ] 📋 Benchmark mode to compare installation times ⚡

### Dotfile Management 🔥 Hard, Medium Impact

**Recommendation:** Focus on integration with existing dotfile managers (chezmoi, stow) rather than building advanced features.

- [ ] 📋 Encrypted dotfile support (for secrets) 🔥
  - GPG encryption
  - Secure key management

- [ ] 📋 Dotfile templating (OS-specific variations) ⚙️
  - Jinja2-style templating
  - OS/architecture variables

- [ ] 📋 Selective dotfile installation ⚡
- [ ] 📋 Dotfile backup before overwriting ⚡
- [ ] 📋 Git-based dotfile syncing ⚙️

### Shell Integration ⚙️ Medium, Low-Medium Impact

- [ ] 📋 Fish shell support ⚙️
- [ ] 📋 PowerShell support ⚙️
- [ ] 📋 Nu shell support ⚙️
- [ ] 📋 Automatic shell detection and configuration ⚡

---

## ⚠️ Reconsider / Low Value

Ideas that don't align well with devstrap's core mission or have low ROI.

### Platform Support 🔥 Hard, Low Impact

**Recommendation:** Wait for user demand before investing.

- [ ] ⚠️ Windows native support (without WSL) 🔥
  - Requires complete rewrite of package manager logic
  - Chocolatey/Scoop have different paradigms

- [ ] ⚠️ FreeBSD support ⚙️
  - Small user base
  - Different package managers (pkg)

- [ ] ⚠️ Alpine Linux support ⚙️
  - Niche distribution
  - APK package manager

- [ ] ⚠️ NixOS integration 🔥
  - Fundamentally different paradigm (declarative)
  - Conflicts with devstrap's imperative model

### Architecture

- [ ] ⚠️ RISC-V support ⚙️
  - Extremely niche
  - Limited tooling availability

- [ ] 📋 Better ARM/ARM64 detection and handling ⚡
  - This one is worth doing

### Community Features 🔥 Very Hard, Uncertain Value

**Recommendation:** Create `awesome-devstrap-configs` repo instead. GitHub already serves as config sharing platform.

- [ ] ⚠️ Plugin system for custom installers 🔥
  - Massive infrastructure cost
  - Security concerns with third-party code

- [ ] ⚠️ Community package registry 🔥
  - Hosting, moderation, security
  - Conflicts with "batteries included" philosophy

- [ ] ⚠️ Config sharing platform 🔥
  - GitHub already does this

- [ ] ⚠️ Template marketplace 🔥
  - Low value vs effort

### IDE & Editors ⚙️ Medium, Low Impact

**Recommendation:** Provide JSON Schema for autocomplete; let IDEs handle rest. TOML already has excellent IDE support.

- [ ] ⚠️ VS Code extension for config editing ⚙️
  - Low ROI for development time
  - TOML extensions already exist

- [ ] ⚠️ IntelliJ plugin ⚙️
  - Same rationale as VS Code

- [ ] 🎯 Config schema for autocomplete ⚡ (1 day)
  - **This one is worth doing!**
  - JSON Schema provides IDE autocomplete
  - Minimal effort, high value

### CI/CD ⚙️ Medium, Medium Impact

- [ ] 📋 GitHub Actions integration ⚡
  - Example workflow in docs
  - Already tested in TUTORIAL.md

- [ ] 📋 GitLab CI templates ⚡
- [ ] 📋 CircleCI orbs ⚙️
- [ ] 📋 Docker image with devstrap pre-installed ⚡
  - `Dockerfile.tutorial-test` already exists!

### Cloud & Containers ⚙️ Medium, Low-Medium Impact

- [ ] 📋 Dockerfile generation from config ⚙️
  - Interesting but niche
  - Users can write Dockerfiles manually

- [ ] 📋 Cloud-init script generation ⚙️
- [ ] 📋 Vagrant provisioner ⚙️
- [ ] 📋 Ansible playbook generation ⚙️

### Documentation & Community

- [ ] 📋 Interactive documentation website ⚙️
  - Current README.md is excellent
  - Tutorial integration complete

- [ ] 📋 Video tutorials ⚙️
- [ ] 📋 Example configurations repository ⚡
  - Easy: Create `devstrap-configs` repo

- [ ] 📋 Migration guides from other tools ⚡
  - Homebrew Brewfile → devstrap
  - apt-get list → devstrap

---

## Testing & Quality

### Test Coverage

- [ ] 🎯 Integration tests for all package managers ⚙️ (see Phase 1)
- [ ] 🎯 Cross-platform testing matrix ⚙️ (see Phase 1)
- [ ] 📋 Performance regression tests ⚙️
- [ ] 📋 Security audit of installation scripts ⚙️

### Code Quality

- [ ] 📋 Reduce all functions to <30 lines (currently <50) ⚙️
  - Gradual refactoring
  - Follow PROGRAMMING.md guidelines

- [ ] 📋 Increase test coverage to >90% ⚙️
  - Current coverage unknown
  - Add unit tests for domain logic

- [ ] 📋 Add property-based testing 🔥
  - Use `proptest` crate
  - Test config parsing edge cases

- [ ] 📋 Fuzz testing for config parsing 🔥
  - Use `cargo-fuzz`
  - Find malformed config crashes

---

## Security

### Security Features ⚙️-🔥 Medium-Hard, High Impact

- [ ] 🔧 Checksum verification for downloads ⚙️ (1 week)
  - SHA256 checksums
  - Verify GitHub binary downloads

- [ ] 🔧 GPG signature verification ⚙️ (1 week)
  - Verify signed releases
  - Trust chain validation

- [ ] 📋 Supply chain security scanning 🔥
  - SBOM integration
  - Dependency vulnerability checks

- [ ] 📋 Sandboxed installation option 🔥
  - Run installers in containers
  - Requires significant architecture change

- [ ] 📋 Security audit log ⚡
  - Log all installations/removals
  - Tamper-proof logging

### Compliance

- [ ] 📋 SBOM (Software Bill of Materials) generation ⚙️
  - CycloneDX or SPDX format
  - List all installed packages/versions

- [ ] 📋 License compliance checking ⚙️
  - Detect package licenses
  - Warn about incompatible licenses

- [ ] 📋 Vulnerability scanning integration ⚙️
  - Integrate with OSV, Snyk, or GitHub Security

---

## Other Ideas

- [ ] 📋 Remote configuration support (fetch from URL) ⚙️
  - `devstrap --config https://example.com/config.toml`
  - Security concerns (HTTPS only, checksums)

- [ ] 📋 Configuration inheritance from templates ⚙️
  - Related to template system

- [ ] 📋 Profile switching (work/personal/project-specific) ⚙️
  - `devstrap --profile work`
  - Multiple configs in `~/.config/devstrap/profiles/`

- [ ] 📋 Shell history integration (save common commands) ⚙️
  - Interesting but niche

- [ ] 📋 Automatic dependency detection from project files ⚙️
  - Parse `package.json`, `Cargo.toml`, etc.
  - Generate config automatically

- [ ] 📋 Integration with mise/asdf existing configs ⚙️
  - Import `.tool-versions`
  - Migration from mise → devstrap

- [ ] 🎯 Homebrew tap for easier installation ⚡ (1 day)
  - `brew install jkaraskiewicz/devstrap/devstrap`
  - Create tap repository

- [ ] 📋 Snap/Flatpak packaging for devstrap itself ⚙️
  - Easier installation on Linux
  - Distribution via official stores

---

## Summary & Recommendations

**Total Items:** 173 ideas

**Prioritization:**
- 🎯 High Priority: 15 items (~9%) - Next 3-6 months
- 🔧 Medium Priority: 35 items (~20%) - 6-12 months
- 📋 Low Priority: 85 items (~49%) - 12+ months
- ⚠️ Reconsider: 38 items (~22%) - Low value/misaligned

**Estimated Work (High Priority Only):** ~11 weeks

### Next Immediate Steps

1. **Implement Phase 1 first** - Polish what exists before adding features
2. **Skip community/platform expansion** - Niche, massive complexity
3. **Validate with users** - Get feedback on `doctor`/`search` before building more

### Architecture Alignment

Current architecture (`domain/service/usecase/config`) is **perfectly positioned** to support high-priority items. The layer-based design makes adding commands, validation, and package features straightforward.

### Recommended First Feature

**`devstrap doctor`** - Diagnose environment issues, validate config, check dependencies. Will:
- Help users debug independently
- Reduce support burden
- Demonstrate polish and reliability
