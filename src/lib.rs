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
    fn into_custom(&self) -> Self;
}

impl IntoCustom for Class {
    fn into_custom(&self) -> Self {
        use Class::*;
        match *self {
            Owner => Custom(0b100),
            Group => Custom(0b010),
            Other => Custom(0b001),
            Everyone => Custom(0b111),
            same @ Custom(_) => same,
        }
    }
}

impl IntoCustom for ModeBits {
    fn into_custom(&self) -> Self {
        use ModeBits::*;
        match *self {
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

    fn bitand(self, _other: Self) -> Self::Output {
        todo!()
        // Self::AllSet
    }
}

//
impl ModeBits {
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
