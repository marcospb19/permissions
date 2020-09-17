use crate::bits_t;

use std::{
    cmp::Ordering,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

const READ_BIT: bits_t = 0b100; // 4
const WRITE_BIT: bits_t = 0b010; // 2
const EXECUTE_BIT: bits_t = 0b001; // 1
const ALL_MODES_BITS: bits_t = 0b111; // 7

/// Bits for each permission mode `rwx`: `Read`, `Write`, `Execute`, or a
/// _combination_.
///
/// To check those 3 bits, try:
/// ```sh
/// ls -l .
/// ```
#[derive(Debug, Copy, Clone)]
pub enum ModeBits {
    Null,
    Read,
    Write,
    Execute,
    AllBits,
    Custom(bits_t), // Combination of multiple mode bits
}

impl Default for ModeBits {
    fn default() -> Self {
        ModeBits::Null
    }
}

impl From<bits_t> for ModeBits {
    fn from(integer: bits_t) -> Self {
        // Ignore anything above
        let integer = integer & ALL_MODES_BITS;

        let read = integer & READ_BIT != 0;
        let write = integer & WRITE_BIT != 0;
        let execute = integer & EXECUTE_BIT != 0;

        match (read, write, execute) {
            (true, false, false) => Self::Read,
            (false, true, false) => Self::Write,
            (false, false, true) => Self::Execute,
            (true, true, true) => Self::AllBits,
            (false, false, false) => Self::Null,
            _ => Self::Custom(integer),
        }
    }
}

/// Merge two
impl BitOr<ModeBits> for ModeBits {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        ModeBits::from(self.bits() | other.bits())
    }
}
impl BitOrAssign<ModeBits> for ModeBits {
    fn bitor_assign(&mut self, other: Self) {
        *self = self.bitor(other)
    }
}

impl BitAnd<ModeBits> for ModeBits {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        ModeBits::from(self.bits() & other.bits())
    }
}
impl BitAndAssign<ModeBits> for ModeBits {
    fn bitand_assign(&mut self, other: Self) {
        *self = self.bitand(other)
    }
}

// Eq and PartialEq
impl PartialEq for ModeBits {
    fn eq(&self, other: &Self) -> bool {
        self.bits() == other.bits()
    }
}
impl Eq for ModeBits {}

/// Reuse Ord for PartialOrd implementation
impl PartialOrd for ModeBits {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
/// Ord here exposes the Ord of the integer accessed by self.bits()
impl Ord for ModeBits {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bits().cmp(&other.bits())
    }
}

impl ModeBits {
    pub fn bits(&self) -> bits_t {
        match self {
            ModeBits::Null => 0b000,
            ModeBits::Read => 0b100,
            ModeBits::Write => 0b010,
            ModeBits::Execute => 0b001,
            ModeBits::AllBits => 0b111,
            ModeBits::Custom(bits) => *bits,
        }
    }

    pub fn normalize(self) -> Self {
        let read = self.is_read_set();
        let write = self.is_write_set();
        let execute = self.is_execute_set();

        let result = match (read, write, execute) {
            (true, false, false) => ModeBits::Read,
            (false, true, false) => ModeBits::Write,
            (false, false, true) => ModeBits::Execute,
            (false, false, false) => ModeBits::Null,
            (true, true, true) => ModeBits::AllBits,
            _ => ModeBits::Custom(self.bits()),
        };

        result
    }

    pub fn is_read_set(&self) -> bool {
        self.bits().is_read_set()
    }

    pub fn is_write_set(&self) -> bool {
        self.bits().is_write_set()
    }

    pub fn is_execute_set(&self) -> bool {
        self.bits().is_execute_set()
    }
}

trait Simbora {
    fn is_read_set(&self) -> bool;

    fn is_write_set(&self) -> bool;

    fn is_execute_set(&self) -> bool;
}
impl Simbora for bits_t {
    fn is_read_set(&self) -> bool {
        match self & READ_BIT {
            0 => false,
            _ => true,
        }
    }

    fn is_write_set(&self) -> bool {
        match self & WRITE_BIT {
            0 => false,
            _ => true,
        }
    }

    fn is_execute_set(&self) -> bool {
        match self & EXECUTE_BIT {
            0 => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ModeBits::{self, *};

    #[test]
    fn test_explicit_variants_against_custom() {
        assert_eq!(Read, Custom(0b100));
        assert_eq!(Write, Custom(0b010));
        assert_eq!(Execute, Custom(0b001));
        assert_eq!(AllBits, Custom(0b111));
    }

    #[test]
    // Test a | b and creating ModeBits in all the ways possible
    fn test_bitor_and_all_constructors() {
        for i in 0..=7 {
            for j in 0..=7 {
                let merged = Custom(i) | Custom(j);
                assert_eq!(merged, Custom(i | j));
                assert_eq!(merged, ModeBits::from(i | j));
                assert_eq!(merged, ModeBits::from(i) | ModeBits::from(j));
                assert_eq!(merged.bits(), i | j);

                // Test not equal
                if i != j {
                    assert_ne!(Custom(i), Custom(j));
                    assert_ne!(ModeBits::from(i), ModeBits::from(j));
                }
            }
        }
    }

    #[test]
    fn test_useless_bitor() {
        assert_eq!(Read | Read, Read);
        assert_eq!(Write | Write, Write);
        assert_eq!(Execute | Execute, Execute);
        assert_eq!(AllBits | AllBits, AllBits);
    }
}
