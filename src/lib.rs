// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use core::fmt;

/// Format numbers as ordinals e.g., 1st, 12th, 21st, etc.
pub trait Ordinal: fmt::Display {
    /// Format a number as an ordinal. Implementations should not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordinal::Ordinal;
    /// assert_eq!("12th", 12.ordinal());
    /// ```
    fn ordinal(&self) -> String {
        format!("{}{}", self, self.suffix())
    }

    /// Gets the suffix for the number.
    fn suffix(&self) -> &'static str;
}

macro_rules! impl_ordinal {
    ($($t:ty)*) => { $(
        impl $crate::Ordinal for $t {
            fn suffix(&self) -> &'static str {
                let n = Abs::abs(*self);
                let n = (n % 20) as u8;
                if (11..=13).contains(&n) {
                    return "th";
                }

                match (n % 10) {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                }
            }
        }
    )* }
}

impl_ordinal!(u8 u16 u32 u64 u128 usize);
impl_ordinal!(i8 i16 i32 i64 i128 isize);

trait Abs<T> {
    fn abs(self) -> T;
}

macro_rules! impl_abs {
    (signed $($t:ty)*) => { $(
        impl $crate::Abs<$t> for $t {
            fn abs(self) -> $t {
                self.abs()
            }
        }
    )* };

    (unsigned $($t:ty)*) => { $(
        impl $crate::Abs<$t> for $t {
            fn abs(self) -> $t {
                self
            }
        }
    )* };
}

impl_abs!(unsigned u8 u16 u32 u64 u128 usize);
impl_abs!(signed i8 i16 i32 i64 i128 isize);

#[test]
fn test_fmt() {
    assert_eq!(0u8.ordinal(), "0th");
    assert_eq!(1u16.ordinal(), "1st");
    assert_eq!(2u32.ordinal(), "2nd");
    assert_eq!(3u64.ordinal(), "3rd");
    assert_eq!(4u128.ordinal(), "4th");
    assert_eq!(5usize.ordinal(), "5th");
    assert_eq!(6i8.ordinal(), "6th");
    assert_eq!(7i16.ordinal(), "7th");
    assert_eq!(8i32.ordinal(), "8th");
    assert_eq!(9i64.ordinal(), "9th");
    assert_eq!(10i128.ordinal(), "10th");
    assert_eq!(11isize.ordinal(), "11th");

    assert_eq!((-0i8).ordinal(), "0th");
    assert_eq!((-1i16).ordinal(), "-1st");
    assert_eq!((-2i32).ordinal(), "-2nd");
    assert_eq!((-3i64).ordinal(), "-3rd");
    assert_eq!((-4i128).ordinal(), "-4th");
    assert_eq!((-5isize).ordinal(), "-5th");
    assert_eq!((-6i8).ordinal(), "-6th");
    assert_eq!((-7i16).ordinal(), "-7th");
    assert_eq!((-8i32).ordinal(), "-8th");
    assert_eq!((-9i64).ordinal(), "-9th");
    assert_eq!((-10i128).ordinal(), "-10th");
    assert_eq!((-11isize).ordinal(), "-11th");

    assert_eq!(19u8.ordinal(), "19th");
    assert_eq!(20u8.ordinal(), "20th");
    assert_eq!(21u8.ordinal(), "21st");
    assert_eq!(22u8.ordinal(), "22nd");
    assert_eq!(23u8.ordinal(), "23rd");
    assert_eq!(24u8.ordinal(), "24th");

    assert_eq!(100u8.ordinal(), "100th");
    assert_eq!(101u8.ordinal(), "101st");

    assert_eq!(111u8.ordinal(), "111th");
    assert_eq!(112u8.ordinal(), "112th");

    assert_eq!(1001u32.ordinal(), "1001st");
    assert_eq!(1002u32.ordinal(), "1002nd");
    assert_eq!(1003u32.ordinal(), "1003rd");
    assert_eq!(1004u32.ordinal(), "1004th");

    assert_eq!(10001001u128.ordinal(), "10001001st");
    assert_eq!(10001002u128.ordinal(), "10001002nd");
    assert_eq!(10001003u128.ordinal(), "10001003rd");
    assert_eq!(10001004u128.ordinal(), "10001004th");

    assert_eq!(10001111u128.ordinal(), "10001111th");
    assert_eq!(10001111u128.ordinal(), "10001111th");
    assert_eq!(10001111u128.ordinal(), "10001111th");
}

#[test]
fn test_suffix() {
    assert_eq!(0u8.suffix(), "th");
    assert_eq!(1u16.suffix(), "st");
    assert_eq!(2u32.suffix(), "nd");
    assert_eq!(3u64.suffix(), "rd");
    assert_eq!(4u128.suffix(), "th");
    assert_eq!(5usize.suffix(), "th");
    assert_eq!(6i8.suffix(), "th");
    assert_eq!(7i16.suffix(), "th");
    assert_eq!(8i32.suffix(), "th");
    assert_eq!(9i64.suffix(), "th");
    assert_eq!(10i128.suffix(), "th");
    assert_eq!(11isize.suffix(), "th");

    assert_eq!((-0i8).suffix(), "th");
    assert_eq!((-1i16).suffix(), "st");
    assert_eq!((-2i32).suffix(), "nd");
    assert_eq!((-3i64).suffix(), "rd");
    assert_eq!((-4i128).suffix(), "th");
    assert_eq!((-5isize).suffix(), "th");
    assert_eq!((-6i8).suffix(), "th");
    assert_eq!((-7i16).suffix(), "th");
    assert_eq!((-8i32).suffix(), "th");
    assert_eq!((-9i64).suffix(), "th");
    assert_eq!((-10i128).suffix(), "th");
    assert_eq!((-11isize).suffix(), "th");

    assert_eq!(19u8.suffix(), "th");
    assert_eq!(20u8.suffix(), "th");
    assert_eq!(21u8.suffix(), "st");
    assert_eq!(22u8.suffix(), "nd");
    assert_eq!(23u8.suffix(), "rd");
    assert_eq!(24u8.suffix(), "th");

    assert_eq!(100u8.suffix(), "th");
    assert_eq!(101u8.suffix(), "st");

    assert_eq!(111u8.suffix(), "th");
    assert_eq!(112u8.suffix(), "th");

    assert_eq!(1001u32.suffix(), "st");
    assert_eq!(1002u32.suffix(), "nd");
    assert_eq!(1003u32.suffix(), "rd");
    assert_eq!(1004u32.suffix(), "th");

    assert_eq!(10001001u128.suffix(), "st");
    assert_eq!(10001002u128.suffix(), "nd");
    assert_eq!(10001003u128.suffix(), "rd");
    assert_eq!(10001004u128.suffix(), "th");

    assert_eq!(10001111u128.suffix(), "th");
    assert_eq!(10001111u128.suffix(), "th");
    assert_eq!(10001111u128.suffix(), "th");
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadMe;
