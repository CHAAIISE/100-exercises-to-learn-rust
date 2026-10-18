// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn new(value: u16) -> Self {
        Self { value }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: SaturatingU16) -> SaturatingU16 {
        SaturatingU16{ value: self.value.saturating_add(other.value) }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u16) -> SaturatingU16 {
        self + SaturatingU16::from(other)
    }
}


impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> SaturatingU16 {
        self + SaturatingU16::from(other)
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> SaturatingU16 {
        self + *other
    }
}

impl From<u16> for SaturatingU16 {
    fn from(v: u16) -> Self {
        SaturatingU16 {
            value: v
        }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(v: u8) -> Self {
        SaturatingU16 {
            value: v.into()
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(v: &u8) -> Self {
        SaturatingU16 {
            value: (*v).into()
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(v: &u16) -> Self {
        SaturatingU16 {
            value: (*v).into()
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}