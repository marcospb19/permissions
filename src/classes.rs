use crate::c_int;

bitflags! {
    /// Classes of permissions are: Owner, Group, Other, or a combination of them
    pub struct Classes: c_int {
        const EMPTY = 0b000;
        const OWNER = 0b100;
        const GROUP = 0b010;
        const OTHER = 0b001;
        const ALL_CLASSES = Self::OWNER.bits | Self::GROUP.bits | Self::OTHER.bits;
    }
}

impl Classes {
    pub fn is_owner_set(&self) -> bool {
        self.bits() & Self::OWNER.bits() != 0
    }

    pub fn is_group_set(&self) -> bool {
        self.bits() & Self::GROUP.bits() != 0
    }

    pub fn is_other_set(&self) -> bool {
        self.bits() & Self::OTHER.bits() != 0
    }
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test() {
//         println!("oi");
//     }

//     //     #[test]

//     // // main just for testing
//     // #[allow(unused_imports)]
//     // use unix_file_permissions::{
//     //     Classes::{AllClasses, Group, Other, Owner},
//     //     ModeBits::{AllBits, Execute, Null, Read, Write},
//     //     PermissionBits,
//     // };

//     // use std::{fs, io, os::unix::fs::PermissionsExt};

//     // fn main() -> io::Result<()> {
//     //     let a = fs::File::open("src/api.rs")?.metadata()?.permissions();
//     //     let _b = PermissionBits::from(a.mode());
//     //     let c = PermissionBits::blank().set(AllClasses, Read | Execute);

//     //     println!("{:#?}", c);
//     //     Ok(())
//     // }
// }
