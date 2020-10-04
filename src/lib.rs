//! This crate provide a friendly interface for permissions of the current
//! process.
//!
//! `is_file_removable`, and any `is_file` function, checks if the current has
//! the permissions required to conclude the operation.
//!
//! Functions available:
//! ```ignore
//! is_file_executable
//! is_file_readable
//! is_file_writablea
//! is_file_removable
//! ```
//!
//! # Example:
//! ```rust
//! use permissions::is_file_removable;
//!
//! use std::{fs, io, path::PathBuf};
//!
//! fn main() -> io::Result<()> {
//!     let path = "path";
//!     fs::File::create(path)?;
//!
//!     if is_file_removable(path)? {
//!         fs::remove_file(path)?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! This crate also offers ModeBits for friendly `rwx` permissions, yet to be
//! documented

pub mod prelude;

// Import macro from bitflags, it's used to declare Classes and ModeBits.
#[macro_use]
extern crate bitflags;

mod classes;
mod functions;
mod mode_bits;
mod permission_bits;

/// Classes of permissions in `Unix` _(Owner, Group, Other)_
pub use classes::Classes;
/// Very useful permission check functions.
pub use functions::*;
/// Mode permission bits _(Read, Write, Execute)_.
pub use mode_bits::ModeBits;
/// PermissionBits...
pub use permission_bits::PermissionBits;
