#[allow(non_camel_case_types)]
type bits_t = u8;

pub mod classes;
pub mod mode_bits;
pub mod permission_bits;

pub use classes::Classes;
pub use mode_bits::ModeBits;
pub use permission_bits::PermissionBits;

// #[cfg(test)]
// mod tests {
//     use std::io;

//     #[test]
//     fn test() -> io::Result<()> {
//         #[allow(unused_imports)]
//         use super::{
//             Classes::{AllClasses, Group, Other, Owner},
//             ModeBits::{AllBits, Execute, Null, Read, Write},
//             PermissionBits,
//         };

//         use std::fs::File;

//         let file: std::fs::File = File::open("foo.txt")?;
//         // // Troca isso por um wrapper
//         // // let mut perms = file.metadata()?.permissions();
//         // let permissions = PermissionBits::try_from(file)?;

//         // if permissions.get(Owner, Execute) {
//         //     println!("Owner of file can execute");
//         // }

//         // if permissions.get(Other, Read | Write) {
//         //     println!("Other users can read and write to this file!");
//         // }

//         // // Let's change permissions with set and unset
//         // let permissions = permissions
//         //     .unset(Owner, Execute)
//         //     .set(Group, Read | Write)
//         //     .reset(Other, Read);

//         // perms.set_readonly(true);
//         // file.set_permissions(perms)?;
//         Ok(())
//     }
// }
