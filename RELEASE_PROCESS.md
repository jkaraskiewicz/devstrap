# devstrap Release Process

This document describes the complete process for releasing a new version of devstrap.

## Overview

devstrap uses **GitHub Actions** to automatically build and release binaries for multiple platforms when you push a version tag.

## Release Types (Semantic Versioning)

Follow [Semantic Versioning](https://semver.org/):

- **Major (X.0.0)**: Breaking changes, incompatible API changes
- **Minor (x.Y.0)**: New features, backwards-compatible
- **Patch (x.y.Z)**: Bug fixes, backwards-compatible

## Complete Release Checklist

### 1. Update Version in Cargo.toml

```bash
# Edit Cargo.toml
# Change: version = "2.0.0"
# To:     version = "2.0.1" (or appropriate version)
```

### 2. Update CHANGELOG.md

Add a new version section with:
- Release date (YYYY-MM-DD format)
- Changes categorized under: Added, Changed, Fixed, Removed
- Update the version links at the bottom

Example:
```markdown
## [2.0.1] - 2025-10-20

### Fixed
- CLI now requires explicit `sync` command instead of defaulting to sync
- Running `devstrap` without arguments now shows help text

[2.0.1]: https://github.com/jkaraskiewicz/devstrap/releases/tag/v2.0.1
```

### 3. Update Cargo.lock

**CRITICAL**: This step is easy to forget but will cause the build to fail!

```bash
cargo update
```

This updates `Cargo.lock` with the new version number from `Cargo.toml`.

### 4. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md Cargo.lock
git commit -m "chore: Prepare v2.0.1 release

- Bump version to 2.0.1
- Document changes in CHANGELOG
- Update Cargo.lock"
```

### 5. Create and Push Git Tag

```bash
# Create the tag (must match version in Cargo.toml, with 'v' prefix)
git tag v2.0.1

# Push commits and tag
git push origin master
git push origin v2.0.1
```

### 6. Wait for GitHub Actions

The release workflow will:
1. **Create release** (draft mode)
2. **Build binaries** for 4 platforms in parallel:
   - Linux x86_64 (musl, static)
   - macOS Apple Silicon (ARM64)
   - macOS Intel (x86_64)
   - Windows x86_64 (MSVC)
3. **Generate SHA256 checksums**
4. **Upload assets** to the draft release

Monitor progress:
```bash
# List recent workflow runs
gh run list --repo jkaraskiewicz/devstrap --limit 3

# View specific run
gh run view <run-id> --repo jkaraskiewicz/devstrap

# Watch logs in real-time
gh run watch <run-id> --repo jkaraskiewicz/devstrap
```

Typical build time: **2-3 minutes**

### 7. Publish the Release

The workflow creates a **draft release**. Review and publish it:

```bash
# Publish the draft release
gh release edit v2.0.1 --repo jkaraskiewicz/devstrap --draft=false
```

Or publish via GitHub web UI:
1. Go to https://github.com/jkaraskiewicz/devstrap/releases
2. Find the draft release
3. Click "Edit"
4. Click "Publish release"

### 8. Verify Release

```bash
# Check release was published
gh release view v2.0.1 --repo jkaraskiewicz/devstrap

# Verify all 8 assets are present (4 binaries + 4 checksums)
gh release view v2.0.1 --repo jkaraskiewicz/devstrap --json assets
```

Expected assets:
- `devstrap-v2.0.1-x86_64-unknown-linux-musl.tar.gz` + `.sha256`
- `devstrap-v2.0.1-aarch64-apple-darwin.tar.gz` + `.sha256`
- `devstrap-v2.0.1-x86_64-apple-darwin.tar.gz` + `.sha256`
- `devstrap-v2.0.1-x86_64-pc-windows-msvc.zip` + `.zip.sha256`

## Common Issues and Solutions

### Issue 1: "Cargo.lock needs to be updated"

**Error:**
```
error: the lock file needs to be updated but --locked was passed to prevent this
```

**Cause:** Forgot to run `cargo update` after bumping version in `Cargo.toml`

**Fix:**
```bash
# Update Cargo.lock
cargo update

# Commit it
git add Cargo.lock
git commit -m "fix: Update Cargo.lock for v2.0.1 release"
git push origin master

# Delete and recreate the tag
git tag -d v2.0.1
git push origin :refs/tags/v2.0.1
gh release delete v2.0.1 --repo jkaraskiewicz/devstrap --yes

# Recreate tag and push
git tag v2.0.1
git push origin v2.0.1
```

### Issue 2: Tag/version mismatch

**Error:**
```
Tag version (2.0.1) does not match Cargo.toml
```

**Fix:** Ensure the tag version (without 'v') matches `Cargo.toml` exactly

### Issue 3: OpenSSL build errors (Linux musl)

**Cause:** Using default reqwest with OpenSSL

**Fix:** Already solved - we use `rustls-tls` feature instead

## Quick Reference Commands

```bash
# Update version and release
vim Cargo.toml              # Bump version
vim CHANGELOG.md            # Add release notes
cargo update                # Update lockfile ⚠️ DON'T FORGET!
git add Cargo.toml CHANGELOG.md Cargo.lock
git commit -m "chore: Prepare vX.Y.Z release"
git tag vX.Y.Z
git push origin master && git push origin vX.Y.Z

# Monitor release
gh run list --repo jkaraskiewicz/devstrap --limit 3
gh run view <run-id> --repo jkaraskiewicz/devstrap

# Publish release
gh release edit vX.Y.Z --repo jkaraskiewicz/devstrap --draft=false

# Verify
gh release view vX.Y.Z --repo jkaraskiewicz/devstrap
```

## Platform-Specific Notes

### Linux (x86_64-unknown-linux-musl)
- **Static binary** - no dependencies required
- Uses musl libc instead of glibc for maximum portability
- Works on any Linux distro (Ubuntu, Debian, Alpine, etc.)

### macOS (aarch64-apple-darwin / x86_64-apple-darwin)
- Native binaries for Apple Silicon and Intel Macs
- No code signing (users may need to allow in Security settings)

### Windows (x86_64-pc-windows-msvc)
- Built with MSVC toolchain
- Distributed as `.zip` file
- Users may need to add to PATH manually

## Versioning Strategy

**Patch releases (X.Y.Z → X.Y.Z+1):**
- Bug fixes
- Documentation updates
- CLI behavior improvements (like v2.0.1)

**Minor releases (X.Y.Z → X.Y+1.0):**
- New features
- New package managers
- New commands (non-breaking)

**Major releases (X.Y.Z → X+1.0.0):**
- Breaking CLI changes
- Breaking config format changes
- Removal of deprecated features

## Automation Details

The release is automated via `.github/workflows/release.yml`:

1. **Triggered by:** Pushing a tag matching `v*.*.*`
2. **Creates:** Draft release with installation instructions
3. **Builds:** 4 platform binaries in parallel
4. **Uploads:** Binaries + SHA256 checksums
5. **Ready for:** Manual review and publishing

## Post-Release

After publishing a release:

1. ✅ Announce on social media / Discord / Slack (if applicable)
2. ✅ Update README.md installation links (if using specific versions)
3. ✅ Create a new `[Unreleased]` section in CHANGELOG.md for next release

## Emergency: Deleting a Bad Release

If you need to delete a release:

```bash
# Delete the GitHub release
gh release delete vX.Y.Z --repo jkaraskiewicz/devstrap --yes

# Delete the git tag locally and remotely
git tag -d vX.Y.Z
git push origin :refs/tags/vX.Y.Z
```

Then fix the issue and re-release with the same version (or bump to X.Y.Z+1).

---

**Last Updated:** 2025-10-20
**Current Version:** v2.0.1
