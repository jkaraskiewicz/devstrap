//! Integration tests for devstrap
//!
//! Tests the full workflow including config parsing, system detection,
//! and dry-run execution.

use devstrap::{Config, Installer, SystemInfo};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_system_detection() {
    let system_info = SystemInfo::detect().expect("Failed to detect system info");

    // Should detect some OS
    assert!(matches!(
        system_info.os,
        devstrap::Os::MacOs | devstrap::Os::Linux
    ));

    // Should detect some architecture
    assert!(matches!(
        system_info.arch,
        devstrap::Arch::X86_64 | devstrap::Arch::Arm64 | devstrap::Arch::Armv7
    ));

    // Should have at least one available package manager
    assert!(!system_info.available_package_managers.is_empty());
}

#[test]
fn test_config_parsing() {
    let config_content = r#"
[packages]
base = ["ripgrep", "bat"]
dev_tools = ["fzf"]
    "#;

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let config = Config::from_file(&config_path).expect("Failed to parse config");

    assert_eq!(config.packages.len(), 2);
    assert!(config.packages.contains_key("base"));
    assert!(config.packages.contains_key("dev_tools"));

    let base_packages = config.get_group_packages("base");
    assert_eq!(base_packages.len(), 2);
    assert!(base_packages.contains(&"ripgrep".to_string()));
    assert!(base_packages.contains(&"bat".to_string()));
}

#[test]
fn test_config_validation_missing_package() {
    let config_content = r#"
[packages]
base = ["ripgrep", "missing_package_that_doesnt_exist"]
    "#;

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let result = Config::from_file(&config_path);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .contains("missing_package_that_doesnt_exist"));
    assert!(error.to_string().contains("not a supported package"));
}

#[test]
fn test_dry_run_installation() {
    let config_content = r#"
[packages]
test_group = ["ripgrep", "bat"]
    "#;

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, config_content).unwrap();

    let config = Config::from_file(&config_path).expect("Failed to parse config");
    let system_info = SystemInfo::detect().expect("Failed to detect system");

    // Create installer in dry-run mode
    let installer = Installer::new(config, system_info, true);

    // Should not error in dry-run mode
    let result = installer.install_group("test_group");
    assert!(result.is_ok());
}

#[test]
fn test_package_name_for_method() {
    use devstrap::PackageConfig;

    let package = PackageConfig {
        description: None,
        name: Some("ripgrep".to_string()),
        cargo: Some("ripgrep".to_string()),
        npm: Some("@ripgrep/ripgrep".to_string()),
        pipx: None,
        github: None,
        brew: None,
        apt: None,
        pacman: None,
        dnf: None,
    };

    assert_eq!(package.name_for_method("cargo"), Some("ripgrep"));
    assert_eq!(package.name_for_method("npm"), Some("@ripgrep/ripgrep"));
    assert_eq!(package.name_for_method("brew"), Some("ripgrep"));
}

#[test]
fn test_available_methods() {
    use devstrap::PackageConfig;

    let package = PackageConfig {
        description: None,
        name: Some("ripgrep".to_string()),
        cargo: Some("ripgrep".to_string()),
        npm: None,
        pipx: None,
        github: None,
        brew: None,
        apt: None,
        pacman: None,
        dnf: None,
    };

    let methods = package.available_methods();
    assert!(methods.contains(&"cargo".to_string()));
    assert!(methods.contains(&"brew".to_string()));
    assert!(!methods.contains(&"npm".to_string()));
}
