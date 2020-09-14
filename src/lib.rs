#[allow(non_camel_case_types)]
type mode_t = u32;

use std::ops::BitAnd;

// use unix_file_permissions::{
//     Class::{Everyone, Group, Other, Owner},
//     ModeBits::{AllSet, Execute, Null, Read, Write},
//     PermissionBits,
// };

/// Class of permissions in `Unix`
#[derive(Copy, Clone, Debug)]
pub enum Class {
    Owner,
    Group,
    Other,
    Everyone,
    // A custom class combination
    Custom(mode_t),
}

/// Permission bits itself inside of the blablabla u32 k
#[derive(Copy, Clone, Debug)]
pub enum ModeBits {
    Null,
    Read,
    Write,
    Execute,
    AllSet,
    // A custom bits combination
    Custom(mode_t),
}

pub(crate) trait IntoCustom {
    fn into_custom(self) -> Self;
}

impl IntoCustom for Class {
    fn into_custom(self) -> Self {
        use Class::*;
        match self {
            Owner => Custom(0b100),
            Group => Custom(0b010),
            Other => Custom(0b001),
            Everyone => Custom(0b111),
            same @ Custom(_) => same,
        }
    }
}

impl IntoCustom for ModeBits {
    fn into_custom(self) -> Self {
        use ModeBits::*;
        match self {
            Null => Custom(0_000),
            Read => Custom(0b100),
            Write => Custom(0b010),
            Execute => Custom(0b001),
            // All set, 7 = 4 + 2 + 1
            AllSet => Custom(0b111),
            same @ Custom(_) => same,
        }
    }
}

impl BitAnd<ModeBits> for ModeBits {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let self_mode = Self::into_custom(self).mode();
        let other_mode = Self::into_custom(other).mode();

        let result = self_mode | other_mode;
        ModeBits::from(result)
    }
}

// impl BitAnd<ModeBits> for Class {
//     type Output = Self;

//     fn bitand(self, _other: Self) -> Self::Output {
//         todo!()
//         // Self::AllSet
//     }
// }

impl From<mode_t> for ModeBits {
    fn from(integer: mode_t) -> Self {
        // Each bit
        let read = integer & 0b100 == 0b100;
        let write = integer & 0b010 == 0b010;
        let execute = integer & 0b001 == 0b001;

        match (read, write, execute) {
            (true, false, false) => Self::Read,
            (false, true, false) => Self::Write,
            (false, false, true) => Self::Execute,
            //
            (true, true, true) => Self::AllSet,
            (false, false, false) => Self::Null,
            _ => Self::Custom(integer & 0b111),
        }
    }
}

//
impl ModeBits {
    pub fn mode(&self) -> mode_t {
        let result = self.into_custom();
        match result {
            Self::Custom(mode) => mode,
            _ => unreachable!(),
        }
    }

    // Very redundant, should I remove this?
    // It can be replaced by
    //
    // if let ModeBits::AllSet = mode {...}
    //
    pub fn is_all_set(&self) -> bool {
        match self {
            ModeBits::AllSet => true,
            _ => false,
        }
    }

    pub fn is_read_set(&self) -> bool {
        // Irrefutable pattern... kindof lol! any other way of writing this?
        if let Self::Custom(mode) = self.into_custom() {
            mode & 0b100 == 0b100
        } else {
            unreachable!()
        }
    }

    pub fn is_write_set(&self) -> bool {
        if let Self::Custom(mode) = self.into_custom() {
            mode & 0b010 == 0b010
        } else {
            unreachable!()
        }
    }

    pub fn is_execute_set(&self) -> bool {
        if let Self::Custom(mode) = self.into_custom() {
            mode & 0b001 == 0b001
        } else {
            unreachable!()
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            ModeBits::Null => true,
            _ => false,
        }
    }

    pub fn is_unset(/* args */) {
        // !self.is_set(args)
    }
}

struct PermissionBits {
    pub owner_bits: ModeBits,
    // ...
}

// impl Display? nah

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = ModeBits::from(0o770);
    }
}
