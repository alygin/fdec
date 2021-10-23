use std::cmp::PartialEq;
use std::convert::TryInto;
use std::mem::size_of;
use std::ops::{BitOr, Div, Shl, Shr, Rem};
use std::fmt::{Display, Debug};

pub trait PrimNumber:
    Copy
    + Clone
    + Display
    + Debug
    + PartialEq
    + PartialOrd
    + Div<Output = Self>
    + BitOr<Output = Self>
    + Rem<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    const BITS: usize = size_of::<Self>() * 8;
    const ZERO: Self;
    const TEN: Self;
}

#[doc(hidden)]
macro_rules! impl_prim_number {
    ($type:ty) => {
        impl PrimNumber for $type {
            const ZERO: Self = 0;
            const TEN: Self = 10;
        }        
    };
}

impl_prim_number!(u8);
impl_prim_number!(u16);
impl_prim_number!(u32);
impl_prim_number!(u64);
impl_prim_number!(u128);

pub trait NumberUnit: PrimNumber {
    type BigUnit: PrimNumber + From<Self> + TryInto<Self>;
    fn hi(bu: Self::BigUnit) -> Self;
    fn lo(bu: Self::BigUnit) -> Self;
}

#[doc(hidden)]
macro_rules! impl_number_unit {
    ($unit_type:ty, $big_type:ty) => {
        impl NumberUnit for $unit_type {
            type BigUnit = $big_type;

            #[inline(always)]
            fn hi(bu: Self::BigUnit) -> Self {
                (bu >> Self::BITS) as Self
            }    

            #[inline(always)]
            fn lo(u: Self::BigUnit) -> Self {
                u as Self
            }
        }        
    };
}

impl_number_unit!(u8, u16);
impl_number_unit!(u16, u32);
impl_number_unit!(u32, u64);
impl_number_unit!(u64, u128);

/// Checks if all the units in the given magnitude are zeros.
pub fn is_magnitude_zero<U: NumberUnit, const N: usize>(magnitude: &[U; N]) -> bool {
    for d in magnitude.iter() {
        if *d != U::ZERO {
            return false;
        }
    }
    true
}

/// Returns the number of the significant units in the slice.
#[inline(always)]
pub fn weight<U: NumberUnit>(mag: &[U]) -> usize {
    for (i, u) in mag.iter().enumerate().rev() {
        if *u != U::ZERO {
            return i + 1;
        }
    }
    0
}

/// Removes trailing zeros
#[inline]
pub fn normalize_digits<U: NumberUnit>(m: &mut Vec<U>) {
    while let Some(n) = m.last() {
        if *n != U::ZERO {
            break;
        }
        m.pop();
    }
}

/// Creates a big unit from two parts.
#[inline(always)]
pub fn big_unit<U: NumberUnit>(hi: U, lo: U) -> <U::BigUnit as BitOr>::Output {
    (U::BigUnit::from(lo)) | ((U::BigUnit::from(hi) << U::BITS))
}

pub fn div_rem_digit<U: NumberUnit>(mut digits: Vec<U>, b: U) -> (Vec<U>, U) {
    let mut rem = 0;
    for d in digits.iter_mut().rev() {
        let (w, r) = div_wide(rem, *d, b);
        *d = w;
        rem = r;
    }
    normalize_digits(&mut digits);
    (digits, rem)
}

#[inline]
fn div_wide<U: NumberUnit>(hi: U, lo: U, divisor: U) -> (U, U) {
    debug_assert!(hi < divisor);
    let val = big_unit(hi, lo);
    let bdiv = U::BigUnit::from(divisor);
    (val / bdiv, val % bdiv)
}
