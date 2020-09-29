//! We need more examples and documentation lmao

// Import macro from bitflags, it used to declare Classes and ModeBits.
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

// TODO: read_only stuff???
