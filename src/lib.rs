//! The crate contains macros for generating fixed-size fixed-point numeric types
//! that exactly fit your domain. The types are fully equipped for performing mathematical
//! computations and are easy to use.
//!
//! With a simple macro call you get a numeric type that:
//!
//! - has no representation errors in the range, defined by the type parameters,
//! - supports arithmetic operations: `+`, `-`, `*`, `/`, `%`, `<<`, `>>`,
//! - comes with mathematical functions: `abs()`, `powi()`, `sqrt()`,
//! - has special values NaN and ±Infinity, and uses them instead of panicing,
//! - provides basic mathematical constants,
//! - seamlessly interacts with Rust's primitive types,
//! - converts values to/from byte arrays,
//! - creates values and performs math operations on stack, avoiding heap allocations.
//!
//! ## When to Use
//!
//! You should probably give fdec a try if:
//!
//! - you need primitive types like `i256` or `i1408`, which Rust doesn't provide,
//! - your business domain is not tolerant to representation errors that may add up during computations (like working with money in finance),
//! - other libraries that provide decimal numbers are not fast enough for you when it comes to doing math,
//! - you need to store lots of decimal numbers, and you'd prefer it to be memory-efficient,
//! - you're just curious to see how it works.
//!
//! # How to Use
//!
//! Add the dependency on `fdec` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fdec = "0.3.1"
//! ```
//!
//! Import it at your crate's root with the `macro_use` attribute:
//!
//! ```
//! #[macro_use]
//! extern crate fdec;
//! # fn main() {}
//! ```
//!
//! Add custom numeric types to your project by calling `fdec*` macros:
//!
//! ```
//! # #[macro_use] extern crate fdec;
//! fdec64! {               // Use 64-bit units
//!     module bigdec,      // Put all the generated code into the `bigdec` module
//!     name BigDec,        // The name for the generated type
//!     length 5,           // 5 * 64-bit units = 320 bits to store numbers
//!     scale 50            // Use 50 decimal places
//! }
//! ```
//!
//! # Example
//!
//! Here we define the `Decimal` structure that represents 160-bit numbers
//! with 30 decimal places.
//!
//! ```
//! #[macro_use]
//! extern crate fdec;
//!
//! fdec32! {            // Use 32-bit units
//!     module dec,      // Put all the generated code into the `dec` module
//!     name Decimal,    // Name the main struct `Decimal`
//!     length 5,        // 5 * 32-bit units = 160 bits to store numbers
//!     scale 30         // Use 30 decimal places
//! }
//!
//! use dec::*;          // Bring the generated stuff to the scope
//!
//! fn main() {
//!     // Use it
//!     let a = Decimal::one();
//!     let b = Decimal::from(14);
//!     let c = dec!(9);
//!     let result = a + 30 * (b / c).powi(3);
//!     println!("{} + 30 * ({} / {})^3 = {}", a, b, c, result);
//!     // 1 + 30 * (14 / 9)^3 = 113.92181069958847736625514403278
//! }
//! ```
//!
//! [More examples](https://github.com/alygin/fdec/tree/master/examples) come with the crate's
//! source code.
//!
//! See the [`Number`] trait to find out what the generated types are capable of.

extern crate lazy_static;

use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Rem, Shr, Sub};
use std::str::FromStr;

#[doc(hidden)]
pub use lazy_static::*;

#[cfg(test)]
mod binomial;
#[doc(hidden)]
pub mod consts;
mod number;
mod prim;

/// Trait of types that can create values from other types with scaling.
pub trait WithScale<T> {
    /// Creates a number from the given value, applying the given scale to it.
    fn with_scale(v: T, scale: usize) -> Self;
}

/// Trait that is implemented by all numeric types, generated by `fdec`.
pub trait Number:
    Default
    + Display
    + Debug
    + PartialEq
    + PartialOrd
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + FromStr
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + WithScale<u8>
    + WithScale<u16>
    + WithScale<u32>
    + WithScale<u64>
    + Shr<usize>
{
    /// Number of decimal places in numbers.
    const SCALE: usize;

    /// Number of units that are used to store numbers.
    const LENGTH: usize;

    /// Returns the zero value.
    fn zero() -> Self;

    /// Returns the `1` value.
    fn one() -> Self;

    /// Returns the smallest positive value.
    fn ulp() -> Self;

    /// Returns the largest normal value.
    fn max() -> Self;

    /// Returns the smallest normal value.
    fn min() -> Self;

    /// Returns the value that represents positive infinity.
    fn infinity() -> Self;

    /// Returns the value that represents negative infinity.
    fn neg_infinity() -> Self;

    /// Returns the value that represents NaN (Not a Number).
    fn nan() -> Self;

    /// Returns `true` if the number is negative (including -Infinity) and `false` if the number is zero or positive.
    fn is_sign_negative(&self) -> bool;

    /// Returns `true` if the number is zero or positive (including +Infinity) and `false` if the number is negative.
    fn is_sign_positive(&self) -> bool;

    /// Returns `true` if this value is positive infinity or negative infinity and `false` otherwise.
    fn is_infinite(&self) -> bool;

    /// Returns `true` if this value is NaN (Not a Number) and `false` otherwise.
    fn is_nan(&self) -> bool;

    /// Returns `true` if this values is NaN or ±Infinity.
    fn is_special(&self) -> bool;

    /// Returns `true` if the number is zero.
    fn is_zero(&self) -> bool;

    /// Returns the absolute value of the number.
    fn abs(&self) -> Self;

    /// Returns the integral part of the number.
    fn trunc(&self) -> Self;

    /// Returns the fraction part of the number.
    fn fract(&self) -> Self;

    /// Returns the square root of the number.
    fn sqrt(&self) -> Self;

    /// Returns the number raised to the given integer power.
    fn powi(&self, n: i32) -> Self;
}

/// Represents errors that can be produced when strings are parsed to numbers.
#[derive(PartialEq, Eq, Debug)]
pub enum ParseNumberError {
    /// String has invalid format and cannot be parsed.
    InvalidFormat,
    /// String represents a value that doesn't fit into the numeric type.
    Overflow,
}

/// Represents errors that can be produces when byte arrays are converted to numbers.
#[derive(PartialEq, Eq, Debug)]
pub enum FromBytesError {
    /// Flags-byte has invalid value.
    InvalidFlags,
}

/// Generates a fixed-size fixed-point numeric type that uses `u8`'s as building blocks.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec8! {              // Use 8-bit units as building blocks
///     module decimal,   // Name of the module that will contain all the generated code
///     name Dec,         // Name of the numeric type to be generated
///     length 7,         // 56-bit number (7 * 8-bit units)
///     scale 8           // 8 decimal places
/// }
///
/// # fn main() {
/// use std::str::FromStr;
/// use decimal::*;
///
/// let a = Dec::from(13);
/// let b = Dec::from_str("2.47").unwrap();
/// assert_eq!(a + b, Dec::with_scale(1547, 2));
/// # }
/// ```
///
/// The `scale` parameter can be omitted. In this case, the generated type represents integer
/// numbers:
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec8! {          // Use 8-bit units as building blocks
///     module int,   // Name of the module that will contain all the generated code
///     name Int,     // Name of the numeric type to be generated
///     length 10     // 80-bit number (10 * 8-bit units)
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! fdec8 {
    (module $modname:ident, name $name:ident, length $mlen:expr) => {
        fdec8!(module $modname, name $name, length $mlen, scale 0);
    };
    (module $modname:ident, name $name:ident, length $mlen:expr, scale $scale:expr) => {
        /// Module that contains the generated numeric type
        #[allow(non_upper_case_globals)]
        #[macro_use]
        pub mod $modname {
            fdec!(
                u8, u16, i16, 8, 100_u8, 2, 0xff,
                module $modname, name $name, length $mlen, scale $scale
            );
            impl_big_primitive_interop!($name, u16, i16, i16);
            impl_big_primitive_interop!($name, u32, i32, i32);
            impl_big_primitive_interop!($name, u64, i64, i64);
        }
    };
}

/// Generates a fixed-size fixed-point numeric type that uses `u16`'s as building blocks.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec16! {             // Use 16-bit units as building blocks
///     module decimal,   // Name of the module that will contain all the generated code
///     name Dec,         // Name of the numeric type to be generated
///     length 6,         // 96-bit number (6 * 16-bit units)
///     scale 12          // 12 decimal places
/// }
///
/// # fn main() {
/// use std::str::FromStr;
/// use decimal::*;
///
/// let a = Dec::from(13);
/// let b = Dec::from_str("2.47").unwrap();
/// assert_eq!(a + b, Dec::with_scale(1547, 2));
/// # }
/// ```
///
/// The `scale` parameter can be omitted. In this case, the generated type represents integer
/// numbers:
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec16! {         // Use 16-bit units as building blocks
///     module int,   // Name of the module that will contain all the generated code
///     name Int,     // Name of the numeric type to be generated
///     length 5      // 80-bit number (5 * 16-bit units)
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! fdec16 {
    (module $modname:ident, name $name:ident, length $mlen:expr) => {
        fdec16!(module $modname, name $name, length $mlen, scale 0);
    };
    (module $modname:ident, name $name:ident, length $mlen:expr, scale $scale:expr) => {
        /// Module that contains the generated numeric type
        #[allow(non_upper_case_globals)]
        #[macro_use]
        pub mod $modname {
            fdec!(
                u16, u32, i32, 16, 10_000_u16, 4, 0xffff,
                module $modname, name $name, length $mlen, scale $scale
            );
            impl_unit_primitive_interop!($name, u16, i16, i16);
            impl_big_primitive_interop!($name, u32, i32, i32);
            impl_big_primitive_interop!($name, u64, i64, i64);
        }
    };
}

/// Generates a fixed-size fixed-point numeric type that uses `u32`'s as building blocks.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec32! {             // Use 32-bit units as building blocks
///     module decimal,   // Name of the module that will contain all the generated code
///     name Dec,         // Name of the numeric type to be generated
///     length 4,         // 128-bit number (4 * 32-bit units)
///     scale 8           // 8 decimal places
/// }
///
/// # fn main() {
/// use std::str::FromStr;
/// use decimal::*;
///
/// let a = Dec::from(13);
/// let b = Dec::from_str("2.47").unwrap();
/// assert_eq!(a + b, Dec::with_scale(1547, 2));
/// # }
/// ```
///
/// The `scale` parameter can be omitted. In this case, the generated type represents integer
/// numbers:
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec32! {         // Use 32-bit units as building blocks
///     module int,   // Name of the module that will contain all the generated code
///     name Int,     // Name of the numeric type to be generated
///     length 4      // 128-bit number (4 * 32-bit units)
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! fdec32 {
    (module $modname:ident, name $name:ident, length $mlen:expr) => {
        fdec32!(module $modname, name $name, length $mlen, scale 0);
    };
    (module $modname:ident, name $name:ident, length $mlen:expr, scale $scale:expr) => {
        /// Module that contains the generated numeric type
        #[allow(non_upper_case_globals)]
        #[macro_use]
        pub mod $modname {
            fdec!(
                u32, u64, i64, 32, 1_000_000_000_u32, 9, 0xffffffff,
                module $modname, name $name, length $mlen, scale $scale
            );
            impl_unit_primitive_interop!($name, u16, i16, i16);
            impl_unit_primitive_interop!($name, u32, i32, i32);
            impl_big_primitive_interop!($name, u64, i64, i64);
        }
    };
}

/// Generates a fixed-size fixed-point numeric type that uses `u64`'s as building blocks.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec64! {             // Use 64-bit units as building blocks
///     module decimal,   // Name of the module that will contain all the generated code
///     name Dec,         // Name of the numeric type to be generated
///     length 15,        // 960-bit number (15 * 64-bit units)
///     scale 100         // 100 decimal places
/// }
///
/// # fn main() {
/// use std::str::FromStr;
/// use decimal::*;
///
/// let a = Dec::from(13);
/// let b = Dec::from_str("2.47").unwrap();
/// assert_eq!(a + b, Dec::with_scale(1547, 2));
/// # }
/// ```
///
/// The `scale` parameter can be omitted. In this case, the generated type represents integer
/// numbers:
///
/// ```
/// # #[macro_use] extern crate fdec;
/// fdec64! {         // Use 64-bit units as building blocks
///     module int,   // Name of the module that will contain all the generated code
///     name Int,     // Name of the numeric type to be generated
///     length 5      // 320-bit number (5 * 64-bit units)
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! fdec64 {
    (module $modname:ident, name $name:ident, length $mlen:expr) => {
        fdec64!(module $modname, name $name, length $mlen, scale 0);
    };
    (module $modname:ident, name $name:ident, length $mlen:expr, scale $scale:expr) => {
        /// Module that contains the generated numeric type
        #[allow(non_upper_case_globals)]
        #[macro_use]
        pub mod $modname {
            fdec!(
                u64, u128, i128, 64, 10_000_000_000_000_000_000_u64, 19, 0xffffffffffffffff,
                module $modname, name $name, length $mlen, scale $scale
            );
            impl_unit_primitive_interop!($name, u16, i16, i16);
            impl_unit_primitive_interop!($name, u32, i32, i32);
            impl_unit_primitive_interop!($name, u64, i64, i64);
        }
    };
}

/// Basic information about the string to be parsed
#[doc(hidden)]
pub struct StrInfo<'a> {
    str: &'a str,         // String without leading or trailing zeros
    neg: bool,            // Indicates that the number is negative
    point: Option<usize>, // Position of the decimal point
}

impl<'a> StrInfo<'a> {
    #[inline(always)]
    pub fn new(str: &'a str, neg: bool, point: Option<usize>) -> StrInfo<'a> {
        StrInfo { str, neg, point }
    }

    #[inline(always)]
    pub fn str(&'a self) -> &'a str {
        self.str
    }

    #[inline(always)]
    pub fn neg(&self) -> bool {
        self.neg
    }

    #[inline(always)]
    pub fn point(&self) -> Option<usize> {
        self.point
    }
}
