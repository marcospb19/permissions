use crate::bits_t;

use std::ops::BitOr;

const OWNER_BIT: bits_t = 0b100;
const GROUP_BIT: bits_t = 0b010;
const OTHER_BIT: bits_t = 0b001;
const ALL_CLASSES_BITS: bits_t = 0b111;

/// Classes of permissions in `Unix`
/// Classes of permissions are: Owner, Group, Other, or a combination of them

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Classes {
    Owner,
    Group,
    Other,
    AllClasses,
    Custom(bits_t), // Combination of multiple classes
}

impl Classes {
    pub fn is_owner_set(&self) -> bool {
        match self {
            Classes::Owner => true,
            _ => false,
        }
    }

    pub fn is_group_set(&self) -> bool {
        match self {
            Classes::Group => true,
            _ => false,
        }
    }

    pub fn is_other_set(&self) -> bool {
        match self {
            Classes::Other => true,
            _ => false,
        }
    }

    pub fn normalize(self) -> Self {
        let owner = self.is_owner_set();
        let group = self.is_group_set();
        let other = self.is_other_set();

        let result = match (owner, group, other) {
            (true, false, false) => Self::Owner,
            (false, true, false) => Self::Group,
            (false, false, true) => Self::Other,
            (true, true, true) => Self::AllClasses,
            _ => Self::Custom(self.bits()),
        };

        result
    }

    // Private function to pass internal bits options around
    fn bits(self) -> bits_t {
        match self {
            Classes::Owner => OWNER_BIT,
            Classes::Group => GROUP_BIT,
            Classes::Other => OTHER_BIT,
            Classes::AllClasses => ALL_CLASSES_BITS,
            Classes::Custom(bits) => bits,
        }
    }
}

impl BitOr<Classes> for Classes {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        // Merge both bits
        let bits = self.bits() | other.bits();
        let temp = Classes::Custom(bits);
        Classes::normalize(temp)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("oi");
    }

    //     #[test]

    // // main just for testing
    // #[allow(unused_imports)]
    // use unix_file_permissions::{
    //     Classes::{AllClasses, Group, Other, Owner},
    //     ModeBits::{AllBits, Execute, Null, Read, Write},
    //     PermissionBits,
    // };

    // use std::{fs, io, os::unix::fs::PermissionsExt};

    // fn main() -> io::Result<()> {
    //     let a = fs::File::open("src/api.rs")?.metadata()?.permissions();
    //     let _b = PermissionBits::from(a.mode());
    //     let c = PermissionBits::blank().set(AllClasses, Read | Execute);

    //     println!("{:#?}", c);
    //     Ok(())
    // }
}
