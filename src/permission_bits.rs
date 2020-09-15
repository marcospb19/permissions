use crate::{bits_t, mode_bits::ModeBits, Classes};

use std::{convert::TryFrom, fs, io, os::unix::fs::PermissionsExt};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct PermissionBits {
    pub owner_bits: ModeBits,
    pub group_bits: ModeBits,
    pub other_bits: ModeBits,
}

impl PermissionBits {
    pub fn blank() -> Self {
        PermissionBits {
            owner_bits: ModeBits::Null,
            group_bits: ModeBits::Null,
            other_bits: ModeBits::Null,
        }
    }

    pub fn new(owner_bits: ModeBits, group_bits: ModeBits, other_bits: ModeBits) -> Self {
        PermissionBits {
            owner_bits,
            group_bits,
            other_bits,
        }
    }

    pub fn bits(&self) -> bits_t {
        let mut result: bits_t = 0;
        result |= (self.owner_bits.bits() as bits_t) << 3 << 3;
        result |= (self.group_bits.bits() as bits_t) << 3;
        result |= self.other_bits.bits() as bits_t;
        result
    }

    pub fn set(&mut self, classes: Classes, bits: ModeBits) -> Self {
        if classes.is_owner_set() {
            // println!("oi");
            // println!("{:#?}", self.owner_bits | bits);
            self.owner_bits = self.owner_bits | bits;
        }
        if classes.is_group_set() {
            self.group_bits = self.group_bits | bits;
        }
        if classes.is_other_set() {
            self.other_bits = self.other_bits | bits;
        }

        self.clone()
    }
}

impl TryFrom<fs::File> for PermissionBits {
    type Error = io::Error;

    fn try_from(file: fs::File) -> Result<Self, Self::Error> {
        let fs_permissions = file.metadata()?.permissions();
        let result = PermissionBits::from(fs_permissions);
        Ok(result)
    }
}

impl From<fs::Permissions> for PermissionBits {
    fn from(fs_permissions: fs::Permissions) -> Self {
        PermissionBits::from(fs_permissions.mode())
    }
}

#[allow(unused_parens)]
impl From<bits_t> for PermissionBits {
    fn from(mode: bits_t) -> Self {
        let owner = (mode & 0o700) >> 3 >> 3;
        let group = (mode & 0o070) >> 3;
        let other = (mode & 0o007);

        let owner = ModeBits::from(owner as bits_t);
        let group = ModeBits::from(group as bits_t);
        let other = ModeBits::from(other as bits_t);

        PermissionBits::new(owner, group, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bits_t,
        Classes::{AllClasses, Group, Other, Owner},
        ModeBits::{AllBits, Execute, Null, Read, Write},
        PermissionBits,
    };

    #[test]
    fn test_set() {
        let mut perms = PermissionBits::blank();
        perms.set(Owner, Read);
        perms.set(Group, Write);
        perms.set(Other, Execute);
        let mode = perms.bits();
        // assert_eq!(mode, 0o421)
    }
}
