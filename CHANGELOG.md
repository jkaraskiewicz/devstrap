# Changelog

All notable changes to devstrap will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions release workflow with multi-platform support
  - Linux x86_64 (musl, static binaries)
  - macOS Apple Silicon (ARM64)
  - macOS Intel (x86_64)
  - Windows x86_64 (MSVC)
- Comprehensive CUJ-based tutorial in README.md
- Docker test infrastructure for tutorial validation
- Strategic roadmap with phased priorities (173 items)

### Changed
- Renamed TODO.md to ROADMAP.md with detailed prioritization

### Fixed
- Package manager cache updates now run automatically before installations

## [2.0.0] - 2025-01-XX

### Added
- Sync-based architecture with flexible package configuration
- Package manager cache update support
- State file tracking for installed packages
- Dry-run mode for previewing changes
- Prune functionality for removing packages
- Lockfile support for version pinning
- Runtime version management (Node.js, Python, Ruby, Java, Rust, Go, Kotlin)
- Dotfiles management
- Multi-package manager support (APT, Homebrew, DNF, Pacman, YUM)

### Changed
- Complete refactoring following PROGRAMMING.md layer-based architecture
- Domain-driven design with clear separation of concerns

## [1.0.0] - Previous Release

Initial release with basic functionality.

---

## Release Process

To create a new release:

1. Update version in `Cargo.toml`
2. Update this CHANGELOG.md with release date and changes
3. Commit changes: `git commit -am "chore: Prepare v2.0.0 release"`
4. Create and push tag: `git tag v2.0.0 && git push origin v2.0.0`
5. GitHub Actions will automatically build and create a draft release
6. Review and publish the draft release on GitHub

[Unreleased]: https://github.com/jkaraskiewicz/devstrap/compare/v2.0.0...HEAD
[2.0.0]: https://github.com/jkaraskiewicz/devstrap/releases/tag/v2.0.0
[1.0.0]: https://github.com/jkaraskiewicz/devstrap/releases/tag/v1.0.0
