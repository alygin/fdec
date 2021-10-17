//! Conversion between decimals and primitive types.

/// Generates implementations of set of traits for interoperability between a numero type and
/// primitive integer type that fits into a single unit.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_unit_primitive_interop {
    ($name:ident, $uprim:ty, $iprim:ty, $itid:ident) => {
        impl From<$uprim> for $name {
            #[inline(always)]
            fn from(v: $uprim) -> Self {
                $name::from_unit(false, v as Unit, 0)
            }
        }
        impl From<$iprim> for $name {
            #[inline(always)]
            fn from(v: $iprim) -> Self {
                $name::with_scale(v, 0)
            }
        }
        impl WithScale<$uprim> for $name {
            #[inline(always)]
            fn with_scale(v: $uprim, scale: usize) -> Self {
                $name::from_unit(false, v as Unit, scale)
            }
        }
        impl WithScale<$iprim> for $name {
            fn with_scale(v: $iprim, scale: usize) -> Self {
                use std::$itid::MIN;
                let val = if v > 0 {
                    v as Unit
                } else if v == MIN {
                    let uv = -(v + 1) as Unit;
                    uv + 1
                } else {
                    -v as Unit
                };
                $name::from_unit(v < 0, val, scale)
            }
        }
        impl_primitive_arithmetic!($name, $uprim);
        impl_primitive_arithmetic!($name, $iprim);
    };
}

/// Generates implementations of set of traits for interoperability between an fdec type and
/// primitive integer type that don't fits into a single unit.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_big_primitive_interop {
    ($name:ident, $uprim:ty, $iprim:ty, $itid:ident) => {
        impl From<$uprim> for $name {
            #[inline(always)]
            fn from(v: $uprim) -> Self {
                $name::with_scale(v, 0)
            }
        }
        impl From<$iprim> for $name {
            #[inline(always)]
            fn from(v: $iprim) -> Self {
                $name::with_scale(v, 0)
            }
        }
        impl WithScale<$uprim> for $name {
            fn with_scale(v: $uprim, scale: usize) -> Self {
                const U_MASK: $uprim = UNIT_MASK as $uprim; // TODO: Move to the impl level?

                let mut vv = v;
                let mut i = 0;
                let mut mag = [0; M_LENGTH];
                while vv != 0 && i < M_LENGTH {
                    mag[i] = (vv & U_MASK) as Unit;
                    i += 1;
                    vv >>= UNIT_BITS;
                }
                if vv != 0 {
                    return $name::infinity();
                }
                let mut num = $name::from_le_units(false, mag);
                let overflow = num.move_point_right($name::SCALE - scale);
                if overflow {
                    $name::infinity()
                } else {
                    num
                }
            }
        }
        impl WithScale<$iprim> for $name {
            fn with_scale(v: $iprim, scale: usize) -> Self {
                use std::$itid::MIN;
                let uv = if v > 0 {
                    v as $uprim
                } else if v == MIN {
                    (-(v + 1) as $uprim) + 1 // Prevent overflow
                } else {
                    -v as $uprim
                };
                let num = $name::with_scale(uv, scale);
                if v >= 0 {
                    num
                } else {
                    -num
                }
            }
        }
        impl_primitive_arithmetic!($name, $uprim);
        impl_primitive_arithmetic!($name, $iprim);
    };
}

/// Generates implementations of set of traits for interoperability between an fdec type and
/// primitive float type.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_float_primitive_interop {
    ($name:ident, $prim:ty, $itid:ident) => {
        impl From<$prim> for $name {
            fn from(v: $prim) -> Self {
                if !v.is_normal() {
                    if v.is_nan() {
                        return $name::NAN;
                    } else if v.is_infinite() {
                        return if v.is_sign_negative() {
                            $name::NEG_INFINITY
                        } else {
                            $name::INFINITY
                        };
                    } else if v == 0 as $prim {
                        return $name::ZERO;
                    }
                    // Subnormal values are handled as normal
                }
                let s = &format!("{:.PRECISION$}", v, PRECISION = $name::SCALE);
                match $name::from_str(s) {
                    Ok(n) => n,
                    Err(ParseNumberError::Overflow) => {
                        if v.is_sign_negative() {
                            $name::NEG_INFINITY
                        } else {
                            $name::INFINITY
                        }
                    }
                    Err(ParseNumberError::InvalidFormat) => unreachable!(),
                }
            }
        }
        impl_primitive_arithmetic!($name, $prim);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_primitive_trait {
    ($name:ident, $prim:ty, $trait:ident, $method:ident) => {
        impl $trait<$prim> for $name {
            type Output = $name;
            #[inline(always)]
            fn $method(self, v: $prim) -> Self::Output {
                self.$method($name::from(v))
            }
        }
        impl $trait<$name> for $prim {
            type Output = $name;
            #[inline(always)]
            fn $method(self, v: $name) -> Self::Output {
                $name::from(self).$method(v)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_primitive_arithmetic {
    ($name:ident, $prim:ty) => {
        impl_primitive_trait!($name, $prim, Add, add);
        impl_primitive_trait!($name, $prim, Sub, sub);
        impl_primitive_trait!($name, $prim, Mul, mul);
        impl_primitive_trait!($name, $prim, Div, div);
        impl_primitive_trait!($name, $prim, Rem, rem);
        impl_op_assign!($name, AddAssign, $prim, add_assign, add);
        impl_op_assign!($name, SubAssign, $prim, sub_assign, sub);
        impl_op_assign!($name, MulAssign, $prim, mul_assign, mul);
        impl_op_assign!($name, DivAssign, $prim, div_assign, div);
        impl_op_assign!($name, RemAssign, $prim, rem_assign, rem);
    };
}
