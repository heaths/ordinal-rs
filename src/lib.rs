// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

//! # Ordinal formatting
//!
//! Format numbers as ordinals efficiently.
//! You can get the ordinal suffix e.g., "st", "nd", "rd", or "th" without allocations.
//!
//! ## Examples
//!
//! Get an ordinal suffix without allocating.
//!
//! ```
//! use ordinal_trait::Ordinal as _;
//! assert_eq!(12.suffix(), "th");
//! ```
//!
#![cfg_attr(
    feature = "std",
    doc = r##"
Format a number as an ordinal, allocating a new `String`:

```
use ordinal_trait::Ordinal as _;
assert_eq!(12.to_ordinal(), "12th");
```

Get a number representing an ordinal you can use with comparisons and formatting.

```
use ordinal_trait::Ordinal as _;
let n = 12.to_number();
assert_eq!(*n, 12);
assert_eq!(format!("{n}"), "12th");
```
"##
)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

#[cfg(feature = "std")]
mod number {
    use super::*;
    use core::ops::Deref;

    /// Represent numbers as ordinals when displayed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordinal_trait::Ordinal as _;
    /// let n = 12.to_number();
    /// assert_eq!(*n, 12);
    /// assert_eq!(format!("{n}"), "12th");
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Number<T: Ordinal + Copy>(pub(crate) T);

    impl<T: Ordinal + Copy> Number<T> {
        /// Create a `Number` from the `value`.
        ///
        /// # Examples
        ///
        /// This can be used in constant initialization:
        ///
        /// ```
        /// use ordinal_trait::Number;
        /// const TWELVE: Number<i32> = Number::new(12);
        /// ```
        pub const fn new(value: T) -> Self {
            Number(value)
        }
    }

    impl<T: Ordinal + Copy> Deref for Number<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Ordinal + Copy> fmt::Display for Number<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}{}", self.0, self.0.suffix())
        }
    }

    impl<T: Ordinal + Copy> From<T> for Number<T> {
        fn from(value: T) -> Self {
            Number(value)
        }
    }

    #[test]
    fn test_number() {
        const TWO: Number<i32> = Number::new(2);
        let twelve = Number::from(12);

        assert!(TWO < twelve);
        assert_eq!(*twelve, 12);
        assert_eq!(twelve.to_string(), String::from("12th"));
    }
}

#[cfg(feature = "std")]
pub use number::Number;

/// Format numbers as ordinals e.g., 1st, 12th, 21st, etc.
pub trait Ordinal: fmt::Display + Copy {
    /// Get a [`Number`] formattable as an ordinal string.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordinal_trait::Ordinal as _;
    /// let n = 12.to_number();
    /// assert_eq!(12, 12);
    /// assert_eq!(format!("{n}"), "12th");
    /// ```
    #[cfg(feature = "std")]
    fn to_number(self) -> Number<Self> {
        Number(self)
    }

    /// Format a number as an ordinal. Implementations should not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordinal_trait::Ordinal as _;
    /// assert_eq!(12.to_ordinal(), "12th");
    /// ```
    #[cfg(feature = "std")]
    fn to_ordinal(self) -> String {
        format!("{}{}", self, self.suffix())
    }

    /// Gets the suffix for the number.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordinal_trait::Ordinal as _;
    /// assert_eq!(12.suffix(), "th");
    /// ```
    fn suffix(self) -> &'static str;
}

macro_rules! impl_ordinal {
    ($($t:ty)*) => { $(
        impl $crate::Ordinal for $t {
            fn suffix(self) -> &'static str {
                let n = Abs::abs(self);
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

#[cfg(feature = "std")]
#[test]
fn test_fmt() {
    assert_eq!(0u8.to_ordinal(), "0th");
    assert_eq!(1u16.to_ordinal(), "1st");
    assert_eq!(2u32.to_ordinal(), "2nd");
    assert_eq!(3u64.to_ordinal(), "3rd");
    assert_eq!(4u128.to_ordinal(), "4th");
    assert_eq!(5usize.to_ordinal(), "5th");
    assert_eq!(6i8.to_ordinal(), "6th");
    assert_eq!(7i16.to_ordinal(), "7th");
    assert_eq!(8i32.to_ordinal(), "8th");
    assert_eq!(9i64.to_ordinal(), "9th");
    assert_eq!(10i128.to_ordinal(), "10th");
    assert_eq!(11isize.to_ordinal(), "11th");

    assert_eq!((-0i8).to_ordinal(), "0th");
    assert_eq!((-1i16).to_ordinal(), "-1st");
    assert_eq!((-2i32).to_ordinal(), "-2nd");
    assert_eq!((-3i64).to_ordinal(), "-3rd");
    assert_eq!((-4i128).to_ordinal(), "-4th");
    assert_eq!((-5isize).to_ordinal(), "-5th");
    assert_eq!((-6i8).to_ordinal(), "-6th");
    assert_eq!((-7i16).to_ordinal(), "-7th");
    assert_eq!((-8i32).to_ordinal(), "-8th");
    assert_eq!((-9i64).to_ordinal(), "-9th");
    assert_eq!((-10i128).to_ordinal(), "-10th");
    assert_eq!((-11isize).to_ordinal(), "-11th");

    assert_eq!(19u8.to_ordinal(), "19th");
    assert_eq!(20u8.to_ordinal(), "20th");
    assert_eq!(21u8.to_ordinal(), "21st");
    assert_eq!(22u8.to_ordinal(), "22nd");
    assert_eq!(23u8.to_ordinal(), "23rd");
    assert_eq!(24u8.to_ordinal(), "24th");

    assert_eq!(100u8.to_ordinal(), "100th");
    assert_eq!(101u8.to_ordinal(), "101st");

    assert_eq!(111u8.to_ordinal(), "111th");
    assert_eq!(112u8.to_ordinal(), "112th");

    assert_eq!(1001u32.to_ordinal(), "1001st");
    assert_eq!(1002u32.to_ordinal(), "1002nd");
    assert_eq!(1003u32.to_ordinal(), "1003rd");
    assert_eq!(1004u32.to_ordinal(), "1004th");

    assert_eq!(10001001u128.to_ordinal(), "10001001st");
    assert_eq!(10001002u128.to_ordinal(), "10001002nd");
    assert_eq!(10001003u128.to_ordinal(), "10001003rd");
    assert_eq!(10001004u128.to_ordinal(), "10001004th");

    assert_eq!(10001111u128.to_ordinal(), "10001111th");
    assert_eq!(10001111u128.to_ordinal(), "10001111th");
    assert_eq!(10001111u128.to_ordinal(), "10001111th");
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

#[cfg(all(doctest, feature = "std"))]
#[doc = include_str!("../README.md")]
struct ReadMe;
