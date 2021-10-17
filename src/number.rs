/// Implements an `{Op}Assign` trait for an fdec type that already implements `{Op}`.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_op_assign {
    ($name:ident, $optrait:ident, $t:ty, $method:ident, $bmethod:ident) => {
        impl $optrait<$t> for $name {
            #[inline]
            fn $method(&mut self, rhs: $t) {
                let res = self.$bmethod(rhs);
                self.flags = res.flags;
                copy(&res.magnitude, &mut self.magnitude);
            }
        }
    };
}

/// The main macro that generates the main number structure and its implementation.
#[macro_export]
#[doc(hidden)]
macro_rules! fdec {
    ($unit:ty, $bigunit:ty, $ibigunit:ty, $ubits:expr, $ubase:expr, $ubpow:expr, $umask:expr,
     module $modname:ident, name $name:ident, length $mlen:expr, scale $scale:expr) => {

        use std::cmp::{PartialEq, PartialOrd, Ordering};
        use std::fmt::{self, Debug, Display, Formatter};
        use std::ops::*;
        use std::str::FromStr;

        #[doc(hidden)]
        pub use $crate::{Number, WithScale, ParseNumberError, FromBytesError, StrInfo};

        const M_LENGTH: usize = $mlen;                              // Length of the array (in units) that holds the number data

        const UNIT_BITS: usize = $ubits;                            // Number of bits in one unit
        const UNIT_BYTES: usize = $ubits / 8;                       // Number of bytest in one unit
        const UNIT_BASE: Unit = $ubase;                             // Max power of 10 that fits into a unit
        const UNIT_BASE_POWER: usize = $ubpow;                      // Power of UNIT_BASE
        const UNIT_MASK: BigUnit = $umask;
        const BYTE_ARRAY_LEN: usize = M_LENGTH * UNIT_BYTES + 1;    // Length of a byte array that can hold the number data. One extra byte is for flags.

        type Flags = $unit;                                         // Flags use unit type to keep struct alignment. Making it smaller doesn't make any difference at lower level.
        type Unit = $unit;
        type BigUnit = $bigunit;
        type IBigUnit = $ibigunit;

        const FLAG_NEGATIVE: Flags = 1;
        const FLAG_NAN: Flags = 2;
        const FLAG_INFINITY: Flags = 4;
        const FLAGS_NO: Flags = 0;
        const FLAGS_SPECIAL: Flags = FLAG_NAN | FLAG_INFINITY;

        const ZERO_UNIT: Unit = 0;
        const BIG_M_LENGTH: usize = 2 * M_LENGTH;
        const BIG_ONE: BigUnit = 1 << UNIT_BITS;
        const MAX_UNIT: BigUnit = Unit::max_value() as BigUnit;

        lazy_static! {
            static ref ULP: $name = { $name::from_unit(false, 1, $scale) };
            static ref ONE: $name = { $name::from_unit(false, 1, 0) };
        }

        /// Basic mathematical constants.
        pub mod consts {
            use std::str::FromStr;
            use super::*;
            lazy_static! {
                /// Euler's number (e)
                pub static ref E: $name = parse_str_with_scale(fdec::consts::E);
                /// 1/π
                pub static ref FRAC_1_PI: $name = parse_str_with_scale(fdec::consts::FRAC_1_PI);
                /// 1/sqrt(2)
                pub static ref FRAC_1_SQRT_2: $name = parse_str_with_scale(fdec::consts::FRAC_1_SQRT_2);
                /// 2/π
                pub static ref FRAC_2_PI: $name = parse_str_with_scale(fdec::consts::FRAC_2_PI);
                /// 2/sqrt(π)
                pub static ref FRAC_2_SQRT_PI: $name = parse_str_with_scale(fdec::consts::FRAC_2_SQRT_PI);
                /// π/2
                pub static ref FRAC_PI_2: $name = parse_str_with_scale(fdec::consts::FRAC_PI_2);
                /// π/3
                pub static ref FRAC_PI_3: $name = parse_str_with_scale(fdec::consts::FRAC_PI_3);
                /// π/4
                pub static ref FRAC_PI_4: $name = parse_str_with_scale(fdec::consts::FRAC_PI_4);
                /// π/6
                pub static ref FRAC_PI_6: $name = parse_str_with_scale(fdec::consts::FRAC_PI_6);
                /// π/8
                pub static ref FRAC_PI_8: $name =  parse_str_with_scale(fdec::consts::FRAC_PI_8);
                /// ln(2)
                pub static ref LN_2: $name = parse_str_with_scale(fdec::consts::LN_2);
                /// ln(10)
                pub static ref LN_10: $name = parse_str_with_scale(fdec::consts::LN_10);
                /// log<sub>2</sub>(10)
                pub static ref LOG2_10: $name = parse_str_with_scale(fdec::consts::LOG2_10);
                /// log<sub>2</sub>(e)
                pub static ref LOG2_E: $name =  parse_str_with_scale(fdec::consts::LOG2_E);
                /// log<sub>10</sub>(2)
                pub static ref LOG10_2: $name =  parse_str_with_scale(fdec::consts::LOG10_2);
                /// log<sub>10</sub>(e)
                pub static ref LOG10_E: $name = parse_str_with_scale(fdec::consts::LOG10_E);
                /// Archimedes’ constant (π)
                pub static ref PI: $name = parse_str_with_scale(fdec::consts::PI);
                /// sqrt(2)
                pub static ref SQRT_2: $name = parse_str_with_scale(fdec::consts::SQRT_2);
                /// The full circle constant (τ)
                /// Equal to 2π
                pub static ref TAU: $name = parse_str_with_scale(fdec::consts::TAU);
            }

            // Converts the given string to a number with half-up rounding to the type scale.
            fn parse_str_with_scale(str: &str) -> $name {
                let dot_pos = str.find('.').unwrap();
                if $name::SCALE >= str.len() - dot_pos {
                    // Source string has no more digits than we need, only return what we have
                    return $name::from_str(str).unwrap();
                }
                let frac_end = dot_pos + $name::SCALE;
                let mut result = $name::from_str(&str[..frac_end+1]).unwrap();
                // If the next digit is >= 5, round the number up
                let next_char_pos = if $name::SCALE == 0 { dot_pos + 1 } else { dot_pos + 1 + $name::SCALE };
                let next_char = str.chars().nth(next_char_pos).unwrap();
                let next_digit = next_char.to_digit(10).unwrap();
                if next_digit >= 5 {
                    result = result + $name::ulp();
                }
                result
            }
        }

        /// A fixed-size fixed-point numeric type.
        #[derive(Copy, Clone)]
        pub struct $name {
            flags: Flags,
            magnitude: [Unit; M_LENGTH],    // Number magnitude in little-endian order
        }

        impl Number for $name {
            const SCALE: usize = $scale;
            const LENGTH: usize = $mlen;

            #[inline(always)]
            fn zero() -> Self {
                Self::ZERO
            }

            #[inline(always)]
            fn ulp() -> Self {
                *ULP
            }

            #[inline(always)]
            fn one() -> Self {
                *ONE
            }

            #[inline(always)]
            fn max() -> Self {
                Self::MAX
            }

            #[inline(always)]
            fn min() -> Self {
                Self::MIN
            }

            #[inline(always)]
            fn infinity() -> Self {
                Self::INFINITY
            }

            #[inline(always)]
            fn neg_infinity() -> Self {
                Self::NEG_INFINITY
            }

            #[inline(always)]
            fn nan() -> Self {
                Self::NAN
            }

            #[inline(always)]
            fn is_sign_negative(&self) -> bool {
                self.flags & FLAG_NEGATIVE != 0
            }

            #[inline(always)]
            fn is_sign_positive(&self) -> bool {
                self.flags & FLAG_NEGATIVE == 0
            }

            #[inline(always)]
            fn is_infinite(&self) -> bool {
                self.flags & FLAG_INFINITY != 0
            }

            #[inline(always)]
            fn is_nan(&self) -> bool {
                self.flags & FLAG_NAN != 0
            }

            #[inline(always)]
            fn is_special(&self) -> bool {
                self.flags & FLAGS_SPECIAL != 0
            }

            #[inline(always)]
            fn is_zero(&self) -> bool {
                if self.is_special() { false } else { is_magnitude_zero(&self.magnitude) }
            }

            fn abs(&self) -> Self {
                if self.is_special() {
                    if self.is_nan() { *self } else { Self::infinity() }
                } else {
                    if self.is_sign_positive() { *self } else { -*self }
                }
            }

            fn trunc(&self) -> Self {
                if self.is_special() || self.is_zero() {
                    *self
                } else {
                    *self - self.fract()
                }
            }

            fn fract(&self) -> Self {
                if self.is_special() || self.is_zero() {
                    return *self
                } else {
                    *self % $name::one()
                }
            }

            fn sqrt(&self) -> Self {
                if self.is_nan() || (self.is_infinite() && self.is_sign_positive()) {
                    return *self;
                } else if self.is_sign_negative() {
                    return Self::nan();
                }
                const MAX_ITER: u32 = 100;
                let mut iter = 0;
                let mut x = *self >> 1;
                let mut prev_x = Self::zero();
                while iter < MAX_ITER && x != prev_x {
                    prev_x = x;
                    x = (x + *self / x) >> 1;
                    iter += 1;
                };
                x
            }

            fn powi(&self, n: i32) -> Self {
                // Handle special cases
                if self.is_special() {
                    return if self.is_nan() {
                        *self
                    } else if n == 0 {
                        $name::one()
                    } else if n < 0 {
                        $name::zero()
                    } else if self.is_infinite() && (self.is_sign_positive() || n & 1 == 1) {
                        *self
                    } else {
                        $name::infinity()
                    };
                } else if n == 0 {
                    return $name::one();
                } else if n < 0 {
                    return if self.is_zero() {
                        $name::infinity()
                    } else if n == i32::min_value() {           // Prevent overflow when switching from min_value
                        let rev = $name::one() / *self;
                        rev.powi(-(n + 1)) / *self
                    } else {
                        let rev = $name::one() / *self;
                        rev.powi(-n)
                    };
                }
                // TODO: Can use a trick with cutting binary zeros from the magnitude here and putting
                // them back (multiplied by `n`) after the main calculation. That will reduce iterations count.
                let mut res = $name::one();
                let mut mul = *self;
                let mut m = n as u32;
                while m != 0 {
                    if m & 1 == 1 {
                        res = res * mul;
                    }
                    m >>= 1;
                    if m != 0 {
                        mul = mul * mul;
                    }
                }
                return res;
            }
        }

        // Result of converting a byte to flags.
        enum FlagsByteValue {
            Special($name),     // Flags byte contains a special value flag
            Simple(bool)        // Negative-flag value for a simple number value
        }

        impl $name {
            const NAN: $name = $name { flags: FLAG_NAN, magnitude: [0; M_LENGTH] };
            const ZERO: $name = $name { flags: FLAGS_NO, magnitude: [0; M_LENGTH] };
            const INFINITY: $name = $name { flags: FLAG_INFINITY, magnitude: [0; M_LENGTH] };
            const NEG_INFINITY: $name = $name { flags: FLAG_NEGATIVE | FLAG_INFINITY, magnitude: [0; M_LENGTH] };
            const MIN: $name = $name { flags: FLAG_NEGATIVE, magnitude: [Unit::max_value(); M_LENGTH] };
            const MAX: $name = $name { flags: FLAGS_NO, magnitude: [Unit::max_value(); M_LENGTH] };

            /// Creates a number with the given magnitude (in little-endian units order). If `neg` is `true`, a negative value
            /// will be created. Othewise, a positive number is created.
            #[inline(always)]
            pub const fn new(neg: bool, magnitude: [Unit; M_LENGTH]) -> Self {
                $name { flags: if neg { FLAG_NEGATIVE } else { FLAGS_NO }, magnitude }
            }

            #[inline(always)]
            fn with_flags(flags: Flags, magnitude: [Unit; M_LENGTH]) -> Self {
                debug_assert!(!(flags == FLAG_NEGATIVE && is_magnitude_zero(&magnitude)));
                $name { flags, magnitude }
            }

            fn from_unit(neg: bool, v: Unit, scale: usize) -> Self {
                debug_assert!(scale <= $name::SCALE);

                let mut d = $name::new(neg && v > 0, [0; M_LENGTH]);
                d.magnitude[0] = v;
                let overflow = d.move_point_right($name::SCALE - scale);
                if overflow {
                    if neg { $name::neg_infinity() } else { $name::infinity() }
                } else {
                    d
                }
            }

            /// Creates a number from its representation as a byte array in big-endian order.
            pub fn from_be_bytes(bytes: &[u8; BYTE_ARRAY_LEN]) -> Result<Self, FromBytesError> {
                let flags_byte = bytes[0];
                match $name::process_flags_byte(flags_byte)? {
                    FlagsByteValue::Special(sv) => Ok(sv),
                    FlagsByteValue::Simple(neg) => {
                        let mut magnitude: [Unit; M_LENGTH] = [0; M_LENGTH];
                        let mut unit_bytes: [u8; UNIT_BYTES] = [0; UNIT_BYTES];
                        let mut bytes_idx = BYTE_ARRAY_LEN;
                        for i in 0..M_LENGTH {
                            let bytes_end = bytes_idx;
                            bytes_idx -= UNIT_BYTES;
                            unit_bytes.clone_from_slice(&bytes[bytes_idx..bytes_end]);
                            magnitude[i] = Unit::from_be_bytes(unit_bytes);
                        }
                        Ok($name::new(neg, magnitude))
                    }
                }
            }

            /// Creates a number from its representation as a byte array in little-endian order.
            pub fn from_le_bytes(bytes: &[u8; BYTE_ARRAY_LEN]) -> Result<Self, FromBytesError> {
                let flags_byte = bytes[BYTE_ARRAY_LEN - 1];
                match $name::process_flags_byte(flags_byte)? {
                    FlagsByteValue::Special(sv) => Ok(sv),
                    FlagsByteValue::Simple(neg) => {
                        let mut magnitude: [Unit; M_LENGTH] = [0; M_LENGTH];
                        let mut unit_bytes: [u8; UNIT_BYTES] = [0; UNIT_BYTES];
                        let mut bytes_idx = 0;
                        for i in 0..M_LENGTH {
                            let bytes_end = bytes_idx + UNIT_BYTES;
                            unit_bytes.clone_from_slice(&bytes[bytes_idx..bytes_end]);
                            magnitude[i] = Unit::from_le_bytes(unit_bytes);
                            bytes_idx = bytes_end;
                        }
                        Ok($name::new(neg, magnitude))
                    }
                }
            }

            /// Creates a number from its representation as a byte array in native order.
            ///
            /// As the target platform’s native endianness is used, portable code should use `from_be_bytes()`
            /// or `from_le_bytes()`, as appropriate, instead.
            #[inline(always)]
            pub fn from_ne_bytes(bytes: &[u8; BYTE_ARRAY_LEN]) -> Result<Self, FromBytesError> {
                #[cfg(target_endian = "little")]
                {
                    $name::from_le_bytes(bytes)
                }
                #[cfg(target_endian = "big")]
                {
                    $name::from_be_bytes(bytes)
                }
            }

            /// Returns infinity of the same sign as this number.
            #[inline(always)]
            fn to_signed_infinity(&self) -> Self {
                if self.is_sign_positive() { Self::INFINITY } else { Self::NEG_INFINITY }
            }

            /// Returns the memory representation of this number as a byte array in big-endian (network) byte order.
            pub fn to_be_bytes(self) -> [u8; BYTE_ARRAY_LEN] {
                let mut bytes: [u8; BYTE_ARRAY_LEN] = [0; BYTE_ARRAY_LEN];
                bytes[0] = self.flags_byte();
                let mut bytes_idx = BYTE_ARRAY_LEN;
                for unit in self.magnitude {
                    let unit_bytes = unit.to_be_bytes();
                    bytes_idx -= UNIT_BYTES;
                    for j in 0..UNIT_BYTES {
                        bytes[bytes_idx + j] = unit_bytes[j];
                    }
                }
                bytes
            }

            /// Returns the memory representation of this number as a byte array in little-endian byte order.
            pub fn to_le_bytes(self) -> [u8; BYTE_ARRAY_LEN] {
                let mut bytes: [u8; BYTE_ARRAY_LEN] = [0; BYTE_ARRAY_LEN];
                bytes[BYTE_ARRAY_LEN - 1] = self.flags_byte();
                let mut bytes_idx = 0;
                for unit in self.magnitude {
                    let unit_bytes = unit.to_le_bytes();
                    for j in 0..UNIT_BYTES {
                        bytes[bytes_idx + j] = unit_bytes[j];
                    }
                    bytes_idx += UNIT_BYTES;
                }
                bytes
            }

            /// Returns the memory representation of this number as a byte array in native byte order.
            ///
            /// As the target platform’s native endianness is used, portable code should use `to_be_bytes()`
            /// or `to_le_bytes()`, as appropriate, instead.
            #[inline(always)]
            pub fn to_ne_bytes(self) -> [u8; BYTE_ARRAY_LEN] {
                #[cfg(target_endian = "little")]
                {
                    self.to_le_bytes()
                }
                #[cfg(target_endian = "big")]
                {
                    self.to_be_bytes()
                }
            }

            /// Multiplies the number by 10^n. Returns `true` if there was overflow.
            #[inline(always)]
            fn move_point_right(&mut self, n: usize) -> bool {
                for _ in 0..n {
                    if multiply_by_unit(&mut self.magnitude, 10) != 0 {
                        return true
                    }
                }
                false
            }

            /// Creates a number from its representation as a byte array in little-endian order.
            fn process_flags_byte(byte: u8) -> Result<FlagsByteValue, FromBytesError> {
                let flags = Flags::from(byte);
                let nan = flags & FLAG_NAN != 0;
                let infinite = flags & FLAG_INFINITY != 0;
                if nan && infinite {
                    return Err(FromBytesError::InvalidFlags);
                }
                if flags & !FLAG_NEGATIVE & !FLAG_INFINITY & !FLAG_NAN != 0 {
                    return Err(FromBytesError::InvalidFlags);   // Some unsupported flags are filled
                }
                if nan {
                    return Ok(FlagsByteValue::Special($name::nan()));
                }
                let neg = flags & FLAG_NEGATIVE != 0;
                if infinite {
                    let value = if neg { $name::neg_infinity() } else { $name::infinity() };
                    return Ok(FlagsByteValue::Special(value));
                }
                Ok(FlagsByteValue::Simple(flags & FLAG_NEGATIVE != 0))
            }

            // Returns the byte that holds the number flags.
            #[inline(always)]
            fn flags_byte(self) -> u8 {
                self.flags.to_be_bytes()[UNIT_BYTES - 1]
            }
        }

        impl Default for $name {
            #[inline(always)]
            fn default() -> Self {
                $name::ZERO
            }
        }

        //
        // Implementation of traits from std::fmt
        //

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                if self.is_special() {
                    return if self.is_infinite() {
                        if self.is_sign_negative() {
                            f.write_str("-Infinity")
                        } else {
                            f.write_str("Infinity")
                        }
                    } else {
                        f.write_str("NaN")
                    }
                }
                if self.is_sign_negative() {
                    f.write_str("-")?;
                }
                if self.is_zero() {
                    return f.write_str("0");
                }

                // Get digits to display
                let magn = self.magnitude.clone().to_vec();
                let digits = to_digits(magn);

                // Remove trailing zeros from the fraction part
                let mut lz = 0;
                while lz < $name::SCALE && digits[lz] == 0 {
                    lz += 1;
                }
                let di = &digits[lz..];
                let scale = $name::SCALE - lz;

                // Find point position and print leading zeros if necessary
                let point_pos = if scale < di.len() {
                    Some(di.len() - scale)
                } else {
                    f.write_str("0.")?;
                    for _ in di.len()..scale {
                        f.write_str("0")?;
                    }
                    None
                };

                // Print digits
                for (i, d) in di.iter().rev().enumerate() {
                    if Some(i) == point_pos {
                        f.write_str(".")?;
                    }
                    write!(f, "{}", d)?;
                }

                Ok(())
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{{ flags: {:?}, magnitude: {:?}; {} }}", self.flags, self.magnitude, self.to_string())
            }
        }

        //
        // Implementation of traits from std::cmp
        //

        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                if self.is_special() {
                    return if self.is_nan() {
                        false
                    } else {    // self is Infinity
                        !other.is_nan() && self.flags == other.flags
                    };
                } else if other.is_special() {
                    return false;
                }
                if self.flags & FLAG_NEGATIVE != other.flags & FLAG_NEGATIVE {
                    return false;
                }
                for (s, o) in self.magnitude.iter().zip(other.magnitude.iter()) {
                    if s != o {
                        return false;
                    }
                }
                true
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<Ordering> {
                if self.is_special() {
                    return if self.is_nan() || other.is_nan() {
                        None
                    } else if other.flags == self.flags {
                        Some(Ordering::Equal)
                    } else if self.is_sign_positive() {    // self is +Infinity
                        Some(Ordering::Greater)
                    } else {                               // self is -Infinity
                        Some(Ordering::Less)
                    };
                } else if other.is_special() {
                    return if other.is_nan() {
                        None
                    } else if other.is_sign_positive() {   // other is +Infinity
                        Some(Ordering::Less)
                    } else {                               // other is -Infinity
                        Some(Ordering::Greater)
                    };
                }

                // Both numbers are normal => compare magnitudes and signs
                let mut self_zero = true;
                let mut other_zero = true;
                let mut ord = Ordering::Equal;

                for (s, o) in self.magnitude.iter().rev().zip(other.magnitude.iter().rev()) {
                    if self_zero && *s != ZERO_UNIT {
                        self_zero = false;
                    }
                    if other_zero && *o != ZERO_UNIT {
                        other_zero = false;
                    }
                    if s > o {
                        ord = Ordering::Greater;
                        break;
                    } else if s < o {
                        ord = Ordering::Less    ;
                        break;
                    }
                }

                if self_zero && other_zero {
                    return Some(Ordering::Equal);
                }

                let self_positive = self.is_sign_positive();
                let other_positive = other.is_sign_positive();

                if self_positive && !other_positive {
                    Some(Ordering::Greater)
                } else if !self_positive && other_positive {
                    Some(Ordering::Less)
                } else if self_positive {
                    Some(ord)
                } else {
                    Some(ord.reverse())
                }
            }
        }

        //
        // Implementation of traits from std::str
        //

        impl FromStr for $name {
            type Err = ParseNumberError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let si = preparse(s, $name::SCALE)?;
                if si.str().is_empty() {
                    return Ok($name::ZERO);
                }

                let mut mag = [0; M_LENGTH];
                let mut idx = 0;
                while let Some((u, next_idx)) = get_unit(&si, idx) {
                    if idx > 0 {
                        let carry = multiply_by_unit(&mut mag, UNIT_BASE);
                        if carry != 0 {
                            return Err(ParseNumberError::Overflow);
                        }
                    }
                    let carry = add_unit(&mut mag, u);
                    if carry != 0 {
                        return Err(ParseNumberError::Overflow);
                    }
                    idx = next_idx;
                }
                if is_magnitude_zero(&mag) {
                    return Ok($name::ZERO);
                }

                // Rescale
                let sm = match si.point() {
                    None => $name::SCALE,
                    Some(p) => $name::SCALE - (si.str().len() - p - 1),
                };
                let mut d = $name::new(si.neg(), mag);
                let overflow = d.move_point_right(sm);
                if overflow {
                    return Err(ParseNumberError::Overflow);
                }
                Ok(d)
            }
        }

        fn get_unit(si: &StrInfo, start: usize) -> Option<(Unit, usize)> {
            let len = si.str().len() - if si.point().is_some() { 1 } else { 0 };
            let mut n = if start == 0 { len % UNIT_BASE_POWER } else { UNIT_BASE_POWER };
            if n == 0 {
                n = UNIT_BASE_POWER;
            }
            let mut u: Unit = 0;
            for (i, b) in si.str()[start..].bytes().enumerate() {
                if b == b'.' {
                    continue;
                }
                u = u * 10 + (b - b'0') as Unit;
                n -= 1;
                if n == 0 {
                    return Some((u, start + i + 1))
                }
            }
            None
        }

        fn preparse(s: &str, max_scale: usize) -> Result<StrInfo, ParseNumberError> {
            let mut start = 0;
            let mut neg = false;
            let mut point = None;

            if s.starts_with('-') {
                neg = true;
                start = 1;
            } else if s.starts_with('+') {
                start = 1;
            };

            if s[start..].is_empty() || (s.len() - start == 1 && s[start..].starts_with('.')) {
                return Err(ParseNumberError::InvalidFormat);
            }

            // Skip leading zeros
            for c in s[start..].chars() {
                match c {
                    '1'...'9' | '.' => break,
                    '0' => start += 1,
                    _ => return Err(ParseNumberError::InvalidFormat)
                }
            }

            // Find the end
            let mut scale = 0;
            let mut maybe_end = start;
            let mut end = start;
            for (i, c) in s[start..].chars().enumerate() {
                match c {
                    '0'...'9' => {
                        if point.is_none() || scale < max_scale {
                            maybe_end += 1;
                        }
                        if point.is_some() && scale < max_scale {
                            scale += 1;
                        }
                        if point.is_none() || c != '0' {
                            end = maybe_end;
                        }
                    },
                    '.' => if point.is_none() {
                        point = Some(i);
                        maybe_end += 1;
                        end += 1;
                    } else {
                        return Err(ParseNumberError::InvalidFormat)
                    },
                    _ => return Err(ParseNumberError::InvalidFormat)
                }
            }

            // Remove point if there's nothing after it
            if let Some(p) = point {
                if p + start + 1 == end {
                    point = None;
                    end -= 1;
                }
            }

            Ok(StrInfo::new(&s[start..end], neg, point))
        }

        // Implementation of traits from std::ops

        /// Represents a division result as an integral part and remainder.
        #[derive(Debug)]
        enum DivProduct {
            Infinity,
            Normal {
                int: [Unit; BIG_M_LENGTH],
                rem: [Unit; M_LENGTH],
            },
        }

        impl Neg for $name {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self::Output {
                if !self.is_nan() && !self.is_zero() {
                    let mut d = self;
                    d.flags ^= 1;
                    d
                } else {
                    self
                }
            }
        }

        impl Add<$name> for $name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self {
                if self.is_special() || rhs.is_special() {
                    if self.is_nan() || rhs.is_nan() {
                        return $name::NAN;
                    }
                    let s_inf = self.is_infinite();
                    let r_inf = rhs.is_infinite();
                    return if s_inf && !r_inf {
                        self
                    } else if !s_inf && r_inf {
                        rhs
                    } else if self.flags == rhs.flags {     // same infinities
                        self
                    } else {                                // opposite infinities
                        $name::NAN
                    };
                }

                // Normal numbers
                if self.flags == rhs.flags {
                    // Add magnitudes
                    let mut magnitude = self.magnitude.clone();
                    let carry = add_magnitude(&mut magnitude, &rhs.magnitude);
                    let flags = if carry != 0 {
                        self.flags | FLAG_INFINITY
                    } else {
                        self.flags
                    };
                    $name::with_flags(flags, magnitude)
                } else {
                    // Subtract from bigger magnitude
                    let cmp = cmp_magnitudes(&self.magnitude, &rhs.magnitude);
                    let (flags, magnitude) = match cmp {
                        Ordering::Equal => return $name::ZERO,
                        Ordering::Greater => {
                            let mut mg = self.magnitude.clone();
                            sub_from_greater(&mut mg, &rhs.magnitude);
                            (self.flags, mg)
                        }
                        Ordering::Less => {
                            let mut mg = rhs.magnitude.clone();
                            sub_from_greater(&mut mg, &self.magnitude);
                            (rhs.flags, mg)
                        }
                    };
                    $name::with_flags(flags, magnitude)
                }
            }
        }

        impl Sub<$name> for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                if self.is_special() || rhs.is_special() {
                    if self.is_nan() || rhs.is_nan() {
                        return $name::NAN;
                    }
                    let s_inf = self.is_infinite();
                    let r_inf = rhs.is_infinite();
                    return if s_inf && !r_inf {
                        self
                    } else if !s_inf && r_inf {
                        -rhs
                    } else if self.flags == rhs.flags {     // same infinities
                        $name::NAN
                    } else {                                // opposite infinities
                        self
                    };
                }

                // Normal numbers
                if self.flags == rhs.flags {
                    // Subtract from bigger magnitude
                    let cmp = cmp_magnitudes(&self.magnitude, &rhs.magnitude);
                    let (flags, magnitude) = match cmp {
                        Ordering::Equal => return $name::ZERO,
                        Ordering::Greater => {
                            let mut mg = self.magnitude.clone();
                            sub_from_greater(&mut mg, &rhs.magnitude);
                            (self.flags, mg)
                        }
                        Ordering::Less => {
                            let mut mg = rhs.magnitude.clone();
                            sub_from_greater(&mut mg, &self.magnitude);
                            (rhs.flags ^ 1, mg)
                        }
                    };
                    $name::with_flags(flags, magnitude)
                } else {
                    // Add magnitudes
                    let mut magnitude = self.magnitude.clone();
                    let carry = add_magnitude(&mut magnitude, &rhs.magnitude);
                    let flags = if carry != 0 {
                        self.flags | FLAG_INFINITY
                    } else {
                        self.flags
                    };
                    $name::with_flags(flags, magnitude)
                }
            }
        }

        impl Mul<$name> for $name {
            type Output = $name;

            fn mul(self, rhs: $name) -> Self::Output {
                // Handle special cases
                if self.is_special() || rhs.is_special() {
                    return if self.is_nan() || rhs.is_nan() || self.is_zero() || rhs.is_zero() {
                        $name::NAN
                    } else if self.is_sign_positive() ^ rhs.is_sign_positive() {
                        $name::NEG_INFINITY
                    } else {
                        $name::INFINITY
                    };
                }

                // Multiply magnitudes
                let mut mw = [0; BIG_M_LENGTH];
                multiply(&self.magnitude, &rhs.magnitude, &mut mw);

                // Round the result
                let flags = if self.is_sign_positive() ^ rhs.is_sign_positive() { FLAG_NEGATIVE } else { FLAGS_NO };
                let mut magnitude = [0; M_LENGTH];
                let div = divide(&mw, &ONE.magnitude);
                match div {
                    DivProduct::Infinity => $name::with_flags(flags | FLAG_INFINITY, magnitude),
                    DivProduct::Normal { int: ref rm, rem: _ } => {
                        let w = weight(rm);
                        if w == 0 {
                            $name::ZERO
                        } else if w > M_LENGTH {
                            $name::with_flags(flags | FLAG_INFINITY, magnitude)
                        } else {
                            copy(rm, &mut magnitude);
                            $name::with_flags(flags, magnitude)
                        }
                    },
                }
            }
        }

        impl Div<$name> for $name {
            type Output = $name;

            fn div(self, rhs: $name) -> Self::Output {
                // Handle special cases
                if self.is_special() {
                    return if self.is_nan() || rhs.is_special() { $name::NAN } else { self };
                } else if rhs.is_special() {
                    return if rhs.is_nan() { $name::NAN } else { $name::ZERO };
                } else if rhs.is_zero() {
                    return if self.is_zero() {
                        $name::NAN
                    } else {
                        $name::with_flags(self.flags | FLAG_INFINITY, [0; M_LENGTH])
                    };
                }

                // Move dividend's point right by SCALE positions
                let mut dividend = [0; BIG_M_LENGTH];
                multiply(&self.magnitude, &ONE.magnitude, &mut dividend);

                // Actually divide
                let flags = if self.is_sign_positive() ^ rhs.is_sign_positive() { FLAG_NEGATIVE } else { FLAGS_NO };
                let mut magnitude = [0; M_LENGTH];
                let div = divide(&dividend, &rhs.magnitude);
                match div {
                    DivProduct::Infinity => $name::with_flags(flags | FLAG_INFINITY, magnitude),
                    DivProduct::Normal { int: ref rm, rem: _ } => {
                        if weight(rm) > M_LENGTH {
                            $name::with_flags(flags | FLAG_INFINITY, magnitude)
                        } else {
                            copy(rm, &mut magnitude);
                            let mut d = $name::with_flags(flags, magnitude);
                            d
                        }
                    },
                }
            }
        }

        impl Rem<$name> for $name {
            type Output = $name;

            fn rem(self, rhs: $name) -> Self::Output {
                // Handle special cases
                if self.is_special() {
                    return $name::NAN;
                } else if rhs.is_special() {
                    return if rhs.is_nan() { $name::NAN } else { self };
                } else if rhs.is_zero() {
                    return $name::NAN;
                }

                // Move dividend's point right by SCALE positions
                let mut dividend = [0; BIG_M_LENGTH];
                multiply(&self.magnitude, &ONE.magnitude, &mut dividend);

                // Actually divide
                let flags = if self.is_sign_positive() ^ rhs.is_sign_positive() { FLAG_NEGATIVE } else { FLAGS_NO };
                let mut magnitude = [0; M_LENGTH];
                let mut div = divide(&dividend, &rhs.magnitude);
                match div {
                    DivProduct::Infinity => $name::with_flags(flags | FLAG_INFINITY, magnitude),
                    DivProduct::Normal { int: ref mut rm, rem: _ } => {
                        round_magnitude(rm);
                        copy(rm, &mut magnitude);
                        let v = $name::with_flags(FLAGS_NO, magnitude);
                        let z = v * rhs;
                        if self.is_sign_positive() ^ rhs.is_sign_positive() { self + z } else { self - z }
                    },
                }
            }
        }

        impl Shr<usize> for $name {
            type Output = Self;

            fn shr(self, rhs: usize) -> Self::Output {
                if self.is_special() || rhs == 0 {
                    return self;
                }

                let full_units = rhs / UNIT_BITS;
                if full_units >= M_LENGTH {
                    return Self::zero();
                }

                let s = rhs % UNIT_BITS;
                let src = self.magnitude;
                let mut mag = [0; M_LENGTH];
                if s > 0 {
                    for i in 0..(M_LENGTH - full_units - 1) {
                      mag[i] = (src[i + full_units] >> s) | (src[i + full_units + 1] << (UNIT_BITS - s))
                    }
                    mag[M_LENGTH - full_units - 1] = src[M_LENGTH - 1] >> s;
                } else {
                    for i in 0..(M_LENGTH - full_units) {
                        mag[i] = src[i + full_units]
                    }
                }
                $name::new(self.is_sign_negative() && !is_magnitude_zero(&mag), mag)
            }
        }

        impl Shl<usize> for $name {
            type Output = Self;

            fn shl(self, rhs: usize) -> Self::Output {
                if self.is_special() || rhs == 0 {
                    return self;
                }

                let full_units = rhs / UNIT_BITS;
                let src = self.magnitude;
                for u in src[M_LENGTH-full_units..].iter() {
                    if *u != 0 {    // Overflow
                        return self.to_signed_infinity();
                    }
                }

                let s = rhs % UNIT_BITS;
                if s > 0 {
                    let of = src[M_LENGTH - full_units - 1] >> (UNIT_BITS - s as usize);
                    if of > 0 {         // Overflow
                        return self.to_signed_infinity();
                    }
                }

                let mut mag = [0; M_LENGTH];
                if s > 0 {
                    for i in 1..M_LENGTH-full_units {
                        mag[full_units + i] = (src[i] << s) | (src[i - 1] >> (UNIT_BITS - s as usize));
                    }
                    mag[full_units] = src[0] << s;
                } else {
                    for (m, s) in mag[full_units..].iter_mut().zip(src.iter()) {
                        *m = *s;
                    }
                }

                $name::new(self.is_sign_negative() && !is_magnitude_zero(&mag), mag)
            }
        }

        impl_op_assign!($name, AddAssign, $name, add_assign, add);
        impl_op_assign!($name, SubAssign, $name, sub_assign, sub);
        impl_op_assign!($name, MulAssign, $name, mul_assign, mul);
        impl_op_assign!($name, DivAssign, $name, div_assign, div);
        impl_op_assign!($name, RemAssign, $name, rem_assign, rem);
        impl_op_assign!($name, ShlAssign, usize, shl_assign, shl);
        impl_op_assign!($name, ShrAssign, usize, shr_assign, shr);

        //
        // Various auxiliary functions
        //

        #[inline(always)]
        fn cmp_magnitudes(a: &[Unit; M_LENGTH], b: &[Unit; M_LENGTH]) -> Ordering {
            for (s, r) in a.iter().rev().zip(b.iter().rev()) {
                if s > r {
                    return Ordering::Greater;
                } else if s < r {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }

        /// Adds a unit value to the given `dest` magnitude and returns the `carry` unit that doesn't
        /// fit into `dest`.
        #[inline]
        fn add_unit(mag: &mut [Unit], u: Unit) -> Unit {
            if u == 0 {
                return 0;
            }
            let mut carry = u as BigUnit;
            for d in mag.iter_mut() {
                let m = (carry as BigUnit) + (*d as BigUnit);
                *d = lo(m);
                carry = m >> UNIT_BITS;
            }
            lo(carry)
        }

        /// Adds `rhs` to `dest` and returns the carry unit if there was overflow.
        #[inline]
        fn add_magnitude(dest: &mut [Unit; M_LENGTH], rhs: &[Unit; M_LENGTH]) -> Unit {
            let mut carry = 0;
            for (d, r) in dest.iter_mut().zip(rhs.iter()) {
                let m = (carry as BigUnit) + (*d as BigUnit) + (*r as BigUnit);
                *d = lo(m);
                carry = hi(m);
            }
            carry
        }

        /// Subtracts `rhs` from `src`.
        /// The caller must check that `src >= rhs`.
        #[inline]
        fn sub_from_greater(src: &mut [Unit], rhs: &[Unit]) {
            debug_assert!(src.len() >= rhs.len());

            let mut borrow = false;
            for (s, r) in src.iter_mut().zip(rhs.iter()) {
                let mut bs = *s as BigUnit;
                let br = *r as BigUnit;

                if borrow {
                    if bs > 0 {
                        bs -= 1;
                        borrow = false;
                    } else {
                        bs = MAX_UNIT;
                    }
                }

                if bs < br {
                    bs += BIG_ONE;
                    borrow = true;
                }

                bs -= br;
                *s = lo(bs);
            }

            debug_assert!(!borrow);
        }

        /// Multiplies the given magnitude `m` by a single unit `v`. Returns the carry
        /// unit if there was overflow.
        fn multiply_by_unit(mag: &mut [Unit], v: Unit) -> Unit {
            let mut carry: BigUnit = 0;
            let mul = v as BigUnit;
            for mut u in mag.iter_mut() {
                let m = carry + *u as BigUnit * mul;
                carry = hi(m) as BigUnit;
                *u = lo(m);
            }
            lo(carry)
        }

        /// Calculates `a` * `b`.
        /// Caller is responsible for providing `dest` of a descent size and zeroed initial value.
        #[inline(always)]
        fn multiply(a: &[Unit], b: &[Unit], dest: &mut [Unit]) {
            let mut i = 0;
            let mut k = 0;

            for ad in a.iter() {
                for bd in b.iter() {
                    let t = *ad as BigUnit * *bd as BigUnit;
                    let (t_hi, t_lo) = (hi(t), lo(t));
                    let c = add_unit(&mut dest[k..], t_lo);
                    debug_assert!(c == 0);
                    let c = add_unit(&mut dest[k+1..], t_hi);
                    debug_assert!(c == 0);
                    k += 1;
                }
                i += 1;
                k = i;
            }
        }

        /// Divides the `dividend` magnitude to the `divisor` magnitude.
        /// The caller must check that the divisor is not zero.
        /// D. Knuth (3rd edition), 4.3.1, Algorithm D with minor tweaks.
        fn divide(dividend: &[Unit; BIG_M_LENGTH], divisor: &[Unit; M_LENGTH]) -> DivProduct {
            let n = weight(divisor);
            let mut m = weight(dividend);
            if m == 0 {   // Dividend is zero
                return DivProduct::Normal { int: [0; BIG_M_LENGTH], rem: [0; M_LENGTH] };
            };
            debug_assert!(n > 0);

            // Arrays for the result
            let mut qq = [0; BIG_M_LENGTH];     // Integral part
            let mut rr = [0; M_LENGTH];         // Remainder

            let u_hi = dividend[m - 1];
            let mut v_hi = divisor[n - 1];
            let mut bv_hi = v_hi as BigUnit;
            if m < n || (m == n && u_hi < v_hi) {
                // Dividend is less than divisor
                return DivProduct::Normal {
                    int: [0; BIG_M_LENGTH],
                    rem: magnitude_from_slice(&dividend[..M_LENGTH])
                };
            } else if m - n > M_LENGTH {
                return DivProduct::Infinity;
            } else if n == 1 {
                // Special case for a one-unit divisor
                let mut c = 0;
                for j in (0..m).rev() {
                    let p = big_unit(c, dividend[j]);
                    qq[j] = lo(p / bv_hi);
                    c = lo(p % bv_hi);
                }
                rr[0] = c;
                return DivProduct::Normal { int: qq, rem: rr };
            }

            let mut u = [0; BIG_M_LENGTH + 1];
            let mut v = [0; M_LENGTH];

            // Normalization
            let s = v_hi.leading_zeros();
            const B: BigUnit = (Unit::max_value() as BigUnit) + 1;
            if s > 0 {
                let u_len = u.len();
                copy_with_shl(dividend, &mut u, u_len, s);
                copy_with_shl(divisor, &mut v, divisor.len(), s);
                if u[m] != 0 {
                    m += 1;
                }
                v_hi = v[n - 1];
                bv_hi = v_hi as BigUnit;
            } else {
                for (to, from) in u.iter_mut().zip(dividend.iter()) {
                    *to = *from;
                }
                v.copy_from_slice(divisor);
            };

            let bv_hi2 = v[n - 2] as BigUnit;
            for j in (0..m-n+1).rev() {
                // Estimate q
                let uu = big_unit(u[j + n], u[j + n - 1]);
                let mut q: BigUnit = uu / bv_hi;
                let mut r: BigUnit = uu % bv_hi;
                let bu_hi2 = u[j + n - 2] as BigUnit;
                while q >= B || q * bv_hi2 > B * r + bu_hi2 {
                    q -= 1;
                    r += bv_hi;
                    if r >= B {
                        break;
                    }
                }

                // Multiply and subtract
                let mut k: IBigUnit = 0;
                let mut t: IBigUnit;
                for (i, vd) in v.iter().enumerate() {
                    let p: BigUnit = (*vd as BigUnit) * q;
                    t = u[i+j] as IBigUnit - k - (p & UNIT_MASK) as IBigUnit;
                    u[i+j] = t as Unit;
                    k = (p >> UNIT_BITS) as IBigUnit - (t >> UNIT_BITS) as IBigUnit;
                }
                t = u[j+n] as IBigUnit - k;
                u[j+n] = t as Unit;
                qq[j] = lo(q);

                // If subtracted too much, add one divisor back
                if t < 0 {
                    k = 0;
                    for (i, vd) in v.iter().enumerate() {
                        t = u[i+j] as IBigUnit + *vd as IBigUnit + k;
                        u[i+j] = t as Unit;
                        k = t >> UNIT_BITS;
                    }
                    u[j+n] += k as Unit;
                    qq[j] -= 1;
                }
            }

            // Copy and denormalize remainder
            let rr_len = rr.len();
            copy_with_shr(&u, &mut rr, rr_len, s);
            DivProduct::Normal { int: qq, rem: rr }
        }

        #[inline(always)]
        fn round_magnitude(m: &mut [Unit; BIG_M_LENGTH]) {
            let one_mag = &ONE.magnitude;
            match divide(m, one_mag) {
                DivProduct::Infinity => unreachable!(),
                DivProduct::Normal { int: ref q, rem: _ } => {
                    let mut sq = [0; M_LENGTH];
                    copy(q, &mut sq);
                    for d in m.iter_mut() {
                        *d = 0;
                    }
                    multiply(&sq, one_mag, m);
                }
            };
        }

        /// Returns the number of the significant units in the slice.
        #[inline(always)]
        fn weight(mag: &[Unit]) -> usize {
            for (i, u) in mag.iter().enumerate().rev() {
                if *u != 0 {
                    return i + 1;
                }
            }
            0
        }

        #[inline(always)]
        fn magnitude_from_slice(src: &[Unit]) -> [Unit; M_LENGTH] {
            let mut mag = [0; M_LENGTH];
            mag.copy_from_slice(src);
            mag
        }

        /// Copies values from `src` to `dest`.
        #[inline(always)]
        fn copy(src: &[Unit], dest: &mut[Unit]) {
            for (d, s) in dest.iter_mut().zip(src.iter()) {
                *d = *s;
            }
        }

        #[inline(always)]
        fn copy_with_shl(src: &[Unit], dest: &mut[Unit], len: usize, s: u32) {
            for i in (1..len).rev() {
                let l = if i < src.len() { (src[i].wrapping_shl(s)) } else { 0 };      // TODO: Looks ugly. Refactor
                dest[i] = l | (src[i-1] >> (UNIT_BITS - s as usize));
            }
            dest[0] = src[0] << s;
        }

        #[inline(always)]
        fn copy_with_shr(src: &[Unit], dest: &mut[Unit], len: usize, s: u32) {
            for i in 0..len {
                dest[i] = (src[i] >> s) | (src[i+1].wrapping_shl(UNIT_BITS as u32 - s));
            }
        }

        /// Checks if all the units in the given magnitude are zeros.
        #[inline(always)]
        fn is_magnitude_zero(m: &[Unit; M_LENGTH]) -> bool {
            for d in m.iter() {
                if *d != 0 {
                    return false;
                }
            }
            true
        }

        /// Creates a big unit from two parts.
        #[inline(always)]
        fn big_unit(hi: Unit, lo: Unit) -> BigUnit {
            (lo as BigUnit) | ((hi as BigUnit) << UNIT_BITS)
        }

        #[inline(always)]
        fn hi(u: BigUnit) -> Unit {
            (u >> UNIT_BITS) as Unit
        }

        #[inline(always)]
        fn lo(u: BigUnit) -> Unit {
            u as Unit
        }

        fn to_digits(mut digits: Vec<Unit>) -> Vec<u8> {
            normalize_digits(&mut digits);
            if digits.is_empty() {
                return vec![0];
            }

            const RADIX: Unit = 10;
            let mut res: Vec<u8> = Vec::with_capacity(20);    // TODO: Be more intelligent here

            while digits.len() > 1 {
                let (q, mut r) = div_rem_digit(digits, UNIT_BASE);
                for _ in 0..UNIT_BASE_POWER {
                    res.push((r % RADIX) as u8);
                    r /= RADIX;
                }
                digits = q;
            }

            let mut r = digits[0];
            while r != 0 {
                res.push((r % RADIX) as u8);
                r /= RADIX;
            }

            res
        }

        #[inline]
        fn div_wide(hi: Unit, lo: Unit, divisor: Unit) -> (Unit, Unit) {
            debug_assert!(hi < divisor);
            let val = big_unit(hi, lo);
            let bdiv = divisor as BigUnit;
            ((val / bdiv) as Unit, (val % bdiv) as Unit)
        }

        /// Removes trailing zeros
        #[inline]
        fn normalize_digits(m: &mut Vec<Unit>) {
            while let Some(&0) = m.last() {
                m.pop();
            }
        }

        fn div_rem_digit(mut digits: Vec<Unit>, b: Unit) -> (Vec<Unit>, Unit) {
            let mut rem = 0;
            for d in digits.iter_mut().rev() {
                let (w, r) = div_wide(rem, *d, b);
                *d = w;
                rem = r;
            }
            normalize_digits(&mut digits);
            (digits, rem)
        }

        //
        // Interoperability with primitive types
        //

        impl_float_primitive_interop!($name, f32, f32);
        impl_float_primitive_interop!($name, f64, f64);
        impl_unit_primitive_interop!($name, u8, i8, i8);

        /// Macro for creating number values from other types
        #[macro_export]
        macro_rules! $modname {
            ($e:expr) => { $name::from($e) };
            ($e:expr, $s:expr) => { $name::with_scale($e, $s) };
        }
    }
}
