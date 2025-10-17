//! Error handling for devstrap
//!
//! Uses anyhow for application-level error handling with context.
//! Per `PROGRAMMING_V2.md`: Applications should use anyhow, not thiserror.

pub use anyhow::{anyhow, bail, Context, Result};
