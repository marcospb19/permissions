use crate::{c_int, Classes, ModeBits};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct PermissionBits {
    pub owner_bits: ModeBits,
    pub group_bits: ModeBits,
    pub other_bits: ModeBits,
}

impl PermissionBits {
    /// Returns None if there are extra bits set, other than the last 9 bits
    /// expected (3 for the mode of each class)
    pub fn from_bits(bits: c_int) -> Option<Self> {
        // Discard all bits that can appear in 0o777 and check for anything left
        if bits >> 9 != 0 {
            None
        } else {
            Some(PermissionBits::from_bits_truncated(bits))
        }
    }

    pub fn from_bits_truncated(bits: c_int) -> Self {
        let owner: c_int = bits & 0o700;
        let group: c_int = bits & 0o070;
        let other: c_int = bits & 0o007;

        // Shift everything to the last 3 bits
        let owner = ModeBits::from_bits(owner >> 3 >> 3).unwrap();
        let group = ModeBits::from_bits(group >> 3).unwrap();
        let other = ModeBits::from_bits(other).unwrap();

        PermissionBits::new(owner, group, other)
    }

    pub fn empty() -> Self {
        PermissionBits {
            owner_bits: ModeBits::empty(),
            group_bits: ModeBits::empty(),
            other_bits: ModeBits::empty(),
        }
    }

    pub fn new(owner_bits: ModeBits, group_bits: ModeBits, other_bits: ModeBits) -> Self {
        PermissionBits {
            owner_bits,
            group_bits,
            other_bits,
        }
    }

    /// improve this help lalalalalalala
    /// Prepare a 0o777 style from 3 0b111 style lalala
    pub fn bits(&self) -> c_int {
        let mut result: c_int = 0;
        result |= self.owner_bits.bits() << 3 << 3;
        result |= self.group_bits.bits() << 3;
        result |= self.other_bits.bits();
        result
    }

    pub fn insert(&mut self, classes: Classes, bits: ModeBits) -> Self {
        if classes.is_owner_set() {
            self.owner_bits |= bits;
        }
        if classes.is_group_set() {
            self.group_bits |= bits;
        }
        if classes.is_other_set() {
            self.other_bits |= bits;
        }

        *self
    }

    // pub const fn empty() -> ModeBits
    // [src][−]
    // Returns an empty set of flags

    // pub const fn all() -> ModeBits
    // [src][−]
    // Returns the set containing all flags.

    // pub const fn bits(&self) -> c_int
    // [src][−]
    // Returns the raw value of the flags currently stored.

    // pub fn from_bits(bits: c_int) -> Option<ModeBits>
    // [src][−]
    // Convert from underlying bit representation, unless that representation
    // contains bits that do not correspond to a flag.

    // pub const fn from_bits_truncate(bits: c_int) -> ModeBits
    // [src][−]
    // Convert from underlying bit representation, dropping any bits that do not
    // correspond to flags.

    // pub const unsafe fn from_bits_unchecked(bits: c_int) -> ModeBits
    // [src][−]
    // Convert from underlying bit representation, preserving all bits (even those
    // not corresponding to a defined flag).

    // pub const fn is_empty(&self) -> bool
    // [src][−]
    // Returns true if no flags are currently stored.

    // pub const fn is_all(&self) -> bool
    // [src][−]
    // Returns true if all flags are currently set.

    // pub const fn intersects(&self, other: ModeBits) -> bool
    // [src][−]
    // Returns true if there are flags common to both self and other.

    // pub const fn contains(&self, other: ModeBits) -> bool
    // [src][−]
    // Returns true all of the flags in other are contained within self.

    // pub fn insert(&mut self, other: ModeBits)
    // [src][−]
    // Inserts the specified flags in-place.

    // pub fn remove(&mut self, other: ModeBits)
    // [src][−]
    // Removes the specified flags in-place.

    // pub fn toggle(&mut self, other: ModeBits)
    // [src][−]
    // Toggles the specified flags in-place.

    // pub fn set(&mut self, other: ModeBits, value: bool)
    // [src][−]
    // Inserts or removes the specified flags depending on the passed value.

    // que tal eu fazer o insert, remove e outros kd o resto olha la a referencia po
    // kk
    // pub fn insert(&mut self, classes: Classes, bits: ModeBits) -> Self {
    //     if classes.is_owner_set() {
    //         self.owner_bits |= bits;
    //     }
    //     if classes.is_group_set() {
    //         self.group_bits |= bits;
    //     }
    //     if classes.is_other_set() {
    //         self.other_bits |= bits;
    //     }

    //     *self
    // }
}

// impl From<fs::Permissions> for PermissionBits {
//     fn from(fs_permissions: fs::Permissions) -> Self {
//         // u32 as i32, safe?
//         PermissionBits::from(fs_permissions.mode() as c_int)
//     }
// }
