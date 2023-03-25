//! [![Crates.io](https://img.shields.io/crates/v/permissions.svg)](https://crates.io/crates/permissions)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/permissions/blob/main/LICENSE)
//! [![Docs.rs](https://docs.rs/permissions/badge.svg)](https://docs.rs/permissions)
//!
//! Useful filesystem queries for file permissions:
//!
//! See [`functions`](https://docs.rs/permissions/latest/permissions/functions/index.html).
//! - [`is_executable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_executable.html)
//! - [`is_readable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_readable.html)
//! - [`is_writable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_writable.html)
//! - [`is_removable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_removable.html)
//! - [`is_creatable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_creatable.html)
//!
//! # `Windows` support
//! This library now supports `Windows`, however, it hasn't been fully tested.
//!
//! Please, open an issue if you find any problems.
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
//!    #[cfg(unix)]
//!    assert!(is_executable("/usr/bin/cat")?);
//!    assert!(is_removable("src/lib.rs")?);
//!    assert!(is_creatable("src/file.rs")?);
//!
//!    Ok(())
//! }
//! ```
//!
//! # Future
//! I plan on adding a `rwx` permission interface, but I never needed it.
//!
//! If you need it, create an issue.
//!
//! # Helping/Contributing:
//! - Use it and give feedback.
//! - Suggest how to improve.
//! - Open an issue or PR in the repository.
//! - Help improve this README or other docs (even little details).

#![warn(missing_docs)]

pub mod functions;
pub use functions::*;
