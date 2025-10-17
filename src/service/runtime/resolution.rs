//! Runtime version resolution helpers

use crate::domain::Lockfile;
use crate::common::error::Result;
use super::resolver::VersionResolver;

/// Resolve a runtime version, using lockfile if available
pub fn resolve_runtime_version(
    name: &str,
    requested: &str,
    manager: &str,
    lockfile: &mut Lockfile,
) -> Result<String> {
    if lockfile.needs_resolution(name, requested) {
        let resolved = VersionResolver::resolve(name, requested, Some(manager))?;
        lockfile.set_runtime(
            name.to_string(),
            requested.to_string(),
            resolved.clone(),
            manager.to_string(),
        );
        Ok(resolved)
    } else {
        Ok(lockfile
            .get_runtime_version(name)
            .unwrap_or(requested)
            .to_string())
    }
}
