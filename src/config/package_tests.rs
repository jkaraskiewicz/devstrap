//! Tests for PackageConfig

#[cfg(test)]
mod tests {
    use super::super::package::PackageConfig;

    #[test]
    fn test_package_config_available_methods() {
        let pkg = PackageConfig::new()
            .name("ripgrep")
            .cargo("ripgrep");

        let methods = pkg.available_methods();
        assert!(methods.contains(&"cargo".to_string()));
        assert!(methods.contains(&"brew".to_string()));
    }

    #[test]
    fn test_package_config_name_for_method() {
        let pkg = PackageConfig::new()
            .name("ripgrep")
            .cargo("ripgrep");

        assert_eq!(pkg.name_for_method("cargo"), Some("ripgrep"));
        assert_eq!(pkg.name_for_method("brew"), Some("ripgrep"));
    }
}
