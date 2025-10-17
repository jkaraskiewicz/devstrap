//! Cross-cutting concerns and utilities

pub mod command;
pub mod detection;
pub mod error;
pub mod path;
pub mod ui;
pub mod uninstall;

pub use command::run_command;
pub use detection::detect_installation_method;
pub use error::Result;
pub use path::{expand_tilde, home_dir, local_bin_dir};
pub use ui::{confirm, print_system_info, show_banner, show_post_install_instructions};
pub use uninstall::uninstall_package;
