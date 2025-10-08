//! Rustup version resolution

/// Resolve version using rustup
pub(super) fn resolve(version: &str) -> String {
    // rustup uses channels, not versions
    match version {
        "latest" | "stable" => "stable".to_string(),
        "beta" => "beta".to_string(),
        "nightly" => "nightly".to_string(),
        _ => version.to_string(),
    }
}
