//! # NOTE
//!
//! If you want Windows support, check [faccess] instead.
//!
//! ---
//!
//! [![Crates.io](https://img.shields.io/crates/v/permissions.svg)](https://crates.io/crates/permissions)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/permissions/blob/main/LICENSE)
//! [![Docs.rs](https://docs.rs/permissions/badge.svg)](https://docs.rs/permissions)
//!
//! Useful filesystem queries for file permissions:
//!
//! See [`functions`].
//! - [`is_executable`]
//! - [`is_readable`]
//! - [`is_writable`]
//! - [`is_removable`]
//! - [`is_creatable`]
//! - [`access_syscall`] supports a custom bitfield of permissions
//!
//! # Examples:
//! ```
//! use permissions::*;
//!
//! fn main() -> std::io::Result<()> {
//!    // Functions accept `AsRef<Path>`
//!    assert!(is_readable("src/")?);
//!    assert!(is_writable("src/")?);
//!    assert!(is_writable("src/lib.rs")?);
//!    assert!(is_executable("/usr/bin/cat")?);
//!    assert!(is_removable("src/lib.rs")?);
//!    assert!(is_creatable("src/file.rs")?);
//!
//!    Ok(())
//! }
//! ```
//!
//! # Alternatives
//! - [faccess] - it provides Windows support and a trait-based API.
//!
//! [faccess]: https://github.com/Freaky/faccess

#![warn(missing_docs)]

pub mod functions;
pub use functions::*;
