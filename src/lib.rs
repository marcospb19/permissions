//! We need more examples and documentation lmao

// Our compilation will fail for big-endian systems
#[cfg(target_endian = "big")]
compile_error!(
    "Not safely tested on big_endian machines. Please, if you're in one, head to the repository https://github.com/marcospb19/unix_file_permissions and give feedback for us to pass all the tests".
);

#[allow(non_camel_case_types)]
/// c_int in unix is always i32
pub type c_int = i32;

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
