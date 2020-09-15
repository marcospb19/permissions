use crate::{mode_bits::ModeBits, Classes};
use std::os::unix::fs::PermissionsExt;

use std::{convert::TryFrom, fs, io};

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

    pub fn set(&mut self, classes: Classes, bits: ModeBits) -> Self {
        if classes.is_owner_set() {
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
impl From<u32> for PermissionBits {
    fn from(mode: u32) -> Self {
        let owner = (mode & 0o700) >> 3 >> 3;
        let group = (mode & 0o070) >> 3;
        let other = (mode & 0o007);

        let owner = ModeBits::from(owner as u8);
        let group = ModeBits::from(group as u8);
        let other = ModeBits::from(other as u8);

        PermissionBits::new(owner, group, other)
    }
}
