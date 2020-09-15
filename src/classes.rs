use crate::bits_t;

use std::ops::BitOr;

const OWNER_BIT: u8 = 0b100;
const GROUP_BIT: u8 = 0b010;
const OTHER_BIT: u8 = 0b001;
const ALL_CLASSES_BITS: u8 = 0b111;

/// Classes of permissions in `Unix`
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
        let merge_result = self.bits() | other.bits();

        let owner = self.is_owner_set();
        let group = self.is_group_set();
        let other = self.is_other_set();

        let result = match (owner, group, other) {
            (true, false, false) => Self::Owner,
            (false, true, false) => Self::Group,
            (false, false, true) => Self::Other,
            (true, true, true) => Self::AllClasses,
            _ => Self::Custom(merge_result),
        };

        result
    }
}
