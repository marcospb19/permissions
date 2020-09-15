use crate::bits_t;

use std::ops::{BitAnd, BitOr};

const READ_BIT: u8 = 0b100;
const WRITE_BIT: u8 = 0b010;
const EXECUTE_BIT: u8 = 0b001;
const ALL_MODES_BITS: u8 = 0b111;

/// Permission bits itself inside of the blablabla u32 k
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum ModeBits {
    Null,
    Read,
    Write,
    Execute,
    AllBits,
    Custom(bits_t), // Combination of multiple mode bits
}

impl BitOr<ModeBits> for ModeBits {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let merge_result = self.bits() | other.bits();

        let owner = self.is_read_set();
        let group = self.is_write_set();
        let other = self.is_execute_set();

        let result = match (owner, group, other) {
            (true, false, false) => ModeBits::Read,
            (false, true, false) => ModeBits::Write,
            (false, false, true) => ModeBits::Execute,
            (false, false, false) => ModeBits::Null,
            (true, true, true) => ModeBits::AllBits,
            _ => ModeBits::Custom(merge_result),
        };

        result
    }
}

impl Default for ModeBits {
    fn default() -> Self {
        ModeBits::Null
    }
}

// impl BitAnd<ModeBits> for ModeBits {
//     type Output = Self;

//     fn bitand(self, other: Self) -> Self::Output {
//         let self_mode = Self::into_custom(self).mode();
//         let other_mode = Self::into_custom(other).mode();

//         let result = self_mode | other_mode;
//         ModeBits::from(result)
//     }
// }

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

    pub fn is_read_set(&self) -> bool {
        match self.bits() & READ_BIT {
            0 => false,
            _ => true,
        }
    }

    pub fn is_write_set(&self) -> bool {
        match self.bits() & WRITE_BIT {
            0 => false,
            _ => true,
        }
    }

    pub fn is_execute_set(&self) -> bool {
        match self.bits() & EXECUTE_BIT {
            0 => false,
            _ => true,
        }
    }
}
