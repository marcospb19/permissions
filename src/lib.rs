#[allow(non_camel_case_types)]
/// c_int in unix is always i32
pub type c_int = i32;

#[macro_use]
extern crate bitflags;

/// Classes of permissions in `Unix`
pub mod classes;
pub use classes::Classes;
/// ModeBits enum.
pub mod mode_bits;
pub use mode_bits::ModeBits;
/// PermissionBits struct.
pub mod permission_bits;

// pub use classes::Classes;
// pub use mode_bits::ModeBits;
// pub use permission_bits::PermissionBits;

// pub mod functions;

// Import macro bitflags!

// #[allow(unused_imports)]
// use super::{
//     Classes::{AllClasses, Group, Other, Owner},
//     ModeBits::{AllBits, Execute, Null, Read, Write},
//     PermissionBits,
// };

// use std::fs::File;

// let file: std::fs::File = File::open("foo.txt")?;
// // Troca isso por um wrapper
// // let mut perms = file.metadata()?.permissions();
// let permissions = PermissionBits::try_from(file)?;

// if permissions.get(Owner, Execute) {
//     println!("Owner of file can execute");
// }

// if permissions.get(Other, Read | Write) {
//     println!("Other users can read and write to this file!");
// }

// // Let's change permissions with set and unset
// let permissions = permissions
//     .unset(Owner, Execute)
//     .set(Group, Read | Write)
//     .reset(Other, Read);

// perms.set_readonly(true);
// file.set_permissions(perms)?;
