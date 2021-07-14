#[macro_use]
extern crate fdec;

fdec32! {
    module decimal,
    name Decimal,
    length 5,
    scale 25
}

use decimal::Decimal;

fn test_str(d: Decimal, expected: &str) {
    assert_eq!(d.to_string(), expected);
}

#[cfg(test)]
mod basic {
    use super::decimal::*;

    #[test]
    fn test_nan() {
        let nan = Decimal::nan();
        assert!(nan.is_nan());
        assert!(nan.is_special());
        assert!(nan.is_sign_positive());
        assert!(!nan.is_infinite());
        assert!(!nan.is_zero());

        assert!(!Decimal::infinity().is_nan());
        assert!(!Decimal::neg_infinity().is_nan());
        assert!(!Decimal::zero().is_nan());
        assert!(!Decimal::max().is_nan());
        assert!(!Decimal::min().is_nan());
    }

    #[test]
    fn test_infinity() {
        let inf = Decimal::infinity();
        assert!(inf.is_infinite());
        assert!(inf.is_special());
        assert!(inf.is_sign_positive());
        assert!(!inf.is_nan());
        assert!(!inf.is_zero());

        let neg_inf = Decimal::neg_infinity();
        assert!(neg_inf.is_infinite());
        assert!(neg_inf.is_special());
        assert!(neg_inf.is_sign_negative());
        assert!(!neg_inf.is_nan());
        assert!(!neg_inf.is_zero());

        assert!(!Decimal::nan().is_infinite());
        assert!(!Decimal::zero().is_infinite());
        assert!(!Decimal::max().is_infinite());
        assert!(!Decimal::min().is_infinite());
    }

    #[test]
    fn test_special() {
        assert!(Decimal::nan().is_special());
        assert!(Decimal::infinity().is_special());
        assert!(Decimal::neg_infinity().is_special());
        assert!(!Decimal::zero().is_special());
        assert!(!Decimal::max().is_special());
        assert!(!Decimal::min().is_special());
    }

    #[test]
    fn test_constants() {
        assert_eq!(Decimal::e().to_string(), "2.7182818284590452353602874");
    }

    #[test]
    fn test_macro() {
        assert_eq!(decimal!(75), Decimal::from(75));
        assert_eq!(decimal!(75, 1), Decimal::with_scale(75, 1));
    }
}

#[cfg(test)]
mod cmp {
    use super::decimal::*;

    use std::cmp::Ordering;

    #[test]
    fn test_eq_normal() {
        assert_eq(Decimal::zero(), -Decimal::zero(), true);
        assert_eq(Decimal::zero(), Decimal::one(), false);
        assert_eq(Decimal::from(17), Decimal::from(17), true);
        assert_eq(Decimal::from(19), Decimal::from(-19), false);
        assert_eq(Decimal::min(), Decimal::max(), false);
        assert_eq(Decimal::zero(), Decimal::ulp(), false);
    }

    #[test]
    fn test_eq_nan() {
        assert_ne!(Decimal::nan(), Decimal::nan());
        assert_ne!(Decimal::nan(), Decimal::zero());
        assert_ne!(Decimal::nan(), Decimal::one());
        assert_ne!(Decimal::nan(), Decimal::infinity());
        assert_ne!(Decimal::nan(), Decimal::neg_infinity());
    }

    #[test]
    fn test_eq_infinity() {
        assert_eq!(Decimal::infinity(), Decimal::infinity());
        assert_eq!(Decimal::neg_infinity(), Decimal::neg_infinity());
        assert_ne!(Decimal::neg_infinity(), Decimal::infinity());
        assert_ne!(Decimal::infinity(), Decimal::one());
        assert_ne!(Decimal::infinity(), Decimal::zero());
        assert_ne!(Decimal::neg_infinity(), Decimal::one());
    }

    #[test]
    fn test_partial_cmp_nan() {
        let vals = vec![
            Decimal::nan(),
            Decimal::infinity(),
            Decimal::neg_infinity(),
            Decimal::zero(),
            -Decimal::zero(),
            Decimal::one(),
            Decimal::max(),
            Decimal::min(),
            Decimal::ulp(),
        ];
        for v in vals.iter() {
            assert_eq!(Decimal::nan().partial_cmp(v), None);
            assert_eq!(v.partial_cmp(&Decimal::nan()), None);
        }
    }

    #[test]
    fn test_partial_cmp_infinity() {
        let inf = Decimal::infinity();
        let neg_inf = Decimal::neg_infinity();
        let vals = vec![
            Decimal::zero(),
            -Decimal::zero(),
            Decimal::one(),
            Decimal::max(),
            Decimal::min(),
            Decimal::ulp(),
        ];

        for v in vals.iter() {
            assert_cmp(inf, *v, Ordering::Greater);
            assert_cmp(neg_inf, *v, Ordering::Less);
        }

        assert_eq!(inf.partial_cmp(&inf), Some(Ordering::Equal));
        assert_eq!(neg_inf.partial_cmp(&neg_inf), Some(Ordering::Equal));
        assert_cmp(inf, neg_inf, Ordering::Greater);
    }

    #[test]
    fn test_partial_cmp_normal() {
        assert_cmp(Decimal::zero(), -Decimal::zero(), Ordering::Equal);
        assert_cmp(Decimal::zero(), Decimal::one(), Ordering::Less);
        assert_cmp(Decimal::zero(), Decimal::ulp(), Ordering::Less);
        assert_cmp(Decimal::zero(), Decimal::max(), Ordering::Less);
        assert_cmp(Decimal::zero(), Decimal::min(), Ordering::Greater);

        assert_cmp(Decimal::one(), Decimal::max(), Ordering::Less);
        assert_cmp(Decimal::from(-2), -Decimal::zero(), Ordering::Less);
        assert_cmp(Decimal::from(2), -Decimal::zero(), Ordering::Greater);

        assert_cmp(Decimal::from(-10), Decimal::from(10), Ordering::Less);
        assert_cmp(
            Decimal::from(-1_000_000_000_000_000i64),
            Decimal::from(2),
            Ordering::Less,
        );
    }

    fn assert_eq(a: Decimal, b: Decimal, expected: bool) {
        if expected {
            assert_eq!(a, b);
            assert_eq!(b, a);
        } else {
            assert_ne!(a, b);
            assert_ne!(b, a);
        }
    }

    fn assert_cmp(a: Decimal, b: Decimal, expected: Ordering) {
        assert_eq!(a.partial_cmp(&b), Some(expected));
        assert_eq!(b.partial_cmp(&a), Some(expected.reverse()));
    }
}

#[cfg(test)]
mod fmt {
    use super::decimal::*;
    use super::test_str;

    #[test]
    fn test_whole_numbers() {
        test_str(Decimal::zero(), "0");
        test_str(-Decimal::zero(), "0");
        test_str(Decimal::one(), "1");
        test_str(Decimal::from(123456), "123456");
        test_str(Decimal::from(-123456), "-123456");
    }

    #[test]
    fn test_rational_numbers() {
        test_str(Decimal::with_scale(123456, 1), "12345.6");
        test_str(Decimal::with_scale(123456, 4), "12.3456");
        test_str(Decimal::with_scale(123456, 6), "0.123456");
        test_str(Decimal::with_scale(123456, 7), "0.0123456");
        test_str(Decimal::with_scale(123456, 15), "0.000000000123456");
        test_str(Decimal::with_scale(-123456, 3), "-123.456");
        test_str(Decimal::with_scale(-123456, 8), "-0.00123456");
    }

    #[test]
    fn test_special_numbers() {
        test_str(Decimal::nan(), "NaN");
        test_str(Decimal::infinity(), "Infinity");
        test_str(Decimal::neg_infinity(), "-Infinity");
    }
}

#[cfg(test)]
mod ops {
    use super::decimal::{Decimal, WithScale};
    use fdec::Number;
    use std::str::FromStr;

    #[test]
    fn test_add_zero() {
        let vals = [
            Decimal::zero(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(*v + Decimal::zero(), *v);
        }
    }

    #[test]
    fn test_add_nan() {
        let vals = [
            Decimal::nan(),
            Decimal::zero(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
            Decimal::infinity(),
            Decimal::neg_infinity(),
        ];
        for v in vals.iter() {
            assert!((*v + Decimal::nan()).is_nan());
        }
    }

    #[test]
    fn test_add_infinity() {
        let vals = [
            Decimal::zero(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_add(*v, Decimal::infinity(), Decimal::infinity());
            assert_add(*v, Decimal::neg_infinity(), Decimal::neg_infinity());
        }

        assert_eq!(
            Decimal::infinity() + Decimal::infinity(),
            Decimal::infinity()
        );
        assert_eq!(
            Decimal::neg_infinity() + Decimal::neg_infinity(),
            Decimal::neg_infinity()
        );
        assert!((Decimal::infinity() + Decimal::neg_infinity()).is_nan());
        assert!((Decimal::neg_infinity() + Decimal::infinity()).is_nan());
    }

    #[test]
    fn test_add_normal() {
        assert_add(Decimal::zero(), Decimal::zero(), Decimal::zero());
        assert_add(Decimal::zero(), -Decimal::zero(), Decimal::zero());
        assert_add(Decimal::zero(), Decimal::one(), Decimal::one());
        assert_add(
            Decimal::zero(),
            Decimal::from(314152),
            Decimal::from(314152),
        );

        assert_add(Decimal::one(), Decimal::one(), Decimal::from(2));
        assert_add(Decimal::from(100), Decimal::from(23), Decimal::from(123));
        assert_add(
            Decimal::from(5_000_000_000_001u64),
            Decimal::from(7_000_000_000_002u64),
            Decimal::from(12_000_000_000_003u64),
        );

        assert_add(Decimal::max(), Decimal::min(), Decimal::zero());
        assert_add(Decimal::from(87654), Decimal::from(-87654), Decimal::zero());
        assert_add(
            Decimal::from(100),
            Decimal::from(9_000_000_000_000_000u64),
            Decimal::from(9_000_000_000_000_100u64),
        );
        assert_add(
            Decimal::from(-100),
            Decimal::from(9_000_000_000_000_000u64),
            Decimal::from(8_999_999_999_999_900u64),
        );
        assert_add(
            Decimal::from(100),
            Decimal::from(-9_000_000_000_000_000i64),
            Decimal::from(-8_999_999_999_999_900i64),
        );

        assert_add(Decimal::max(), Decimal::ulp(), Decimal::infinity());
    }

    #[test]
    fn test_add_with_overflow() {
        assert_add(Decimal::max(), Decimal::from(1), Decimal::infinity());
        assert_add(Decimal::min(), Decimal::from(-1), Decimal::neg_infinity());
    }

    #[test]
    fn test_add_assign() {
        let mut n = Decimal::one();
        n += Decimal::from(10);
        assert_eq!(n, Decimal::from(11));
        n += 0.5;
        assert_eq!(n, Decimal::with_scale(115, 1));
    }

    #[test]
    fn test_add_with_primitives() {
        assert_eq!(Decimal::one() + 5, Decimal::from(6));
        assert_eq!(9 + Decimal::one(), Decimal::from(10));
        assert_eq!(8.25 + Decimal::one(), Decimal::with_scale(925, 2));
        assert_eq!(Decimal::from(10) + 4.5, Decimal::with_scale(145, 1));
    }

    #[test]
    fn test_add_with_primitives_with_assign() {
        let mut n = Decimal::one();
        n += 5;
        assert_eq!(n, Decimal::from(6));
        n += 0.25;
        assert_eq!(n, Decimal::with_scale(625, 2));
    }

    #[test]
    fn test_sub_zero() {
        let vals = [
            Decimal::zero(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(*v - Decimal::zero(), *v);
        }
    }

    #[test]
    fn test_sub_nan() {
        let vals = [
            Decimal::nan(),
            Decimal::zero(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
            Decimal::infinity(),
            Decimal::neg_infinity(),
        ];
        for v in vals.iter() {
            assert!((*v - Decimal::nan()).is_nan());
        }
    }

    #[test]
    fn test_sub_infinity() {
        let vals = [
            Decimal::zero(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_sub(*v, Decimal::infinity(), Decimal::neg_infinity());
            assert_sub(*v, Decimal::neg_infinity(), Decimal::infinity());
        }

        assert_eq!(
            Decimal::infinity() - Decimal::neg_infinity(),
            Decimal::infinity()
        );
        assert_eq!(
            Decimal::neg_infinity() - Decimal::infinity(),
            Decimal::neg_infinity()
        );
        assert!((Decimal::infinity() - Decimal::infinity()).is_nan());
        assert!((Decimal::neg_infinity() - Decimal::neg_infinity()).is_nan());
    }

    #[test]
    fn test_sub_normal() {
        assert_sub(Decimal::zero(), Decimal::zero(), Decimal::zero());
        assert_sub(Decimal::zero(), -Decimal::zero(), Decimal::zero());
        assert_sub(Decimal::zero(), Decimal::one(), Decimal::from(-1));
        assert_sub(
            Decimal::zero(),
            Decimal::from(314152),
            Decimal::from(-314152),
        );

        assert_sub(Decimal::one(), Decimal::one(), Decimal::zero());
        assert_sub(Decimal::from(-1), Decimal::from(-1), Decimal::zero());
        assert_sub(Decimal::from(100), Decimal::from(23), Decimal::from(77));
        assert_sub(
            Decimal::from(5_000_000_000_005u64),
            Decimal::from(2_000_000_000_002u64),
            Decimal::from(3_000_000_000_003u64),
        );

        assert_sub(Decimal::max(), Decimal::max(), Decimal::zero());
        assert_sub(Decimal::min(), Decimal::min(), Decimal::zero());
        assert_sub(Decimal::ulp(), Decimal::ulp(), Decimal::zero());

        assert_sub(Decimal::from(30), Decimal::from(20), Decimal::from(10));
        assert_sub(Decimal::from(30), Decimal::from(-20), Decimal::from(50));
        assert_sub(Decimal::from(-30), Decimal::from(20), Decimal::from(-50));

        assert_sub(
            Decimal::from(9_000_000_000_000_000u64),
            Decimal::from(100),
            Decimal::from(8_999_999_999_999_900u64),
        );

        assert_sub(Decimal::min(), Decimal::ulp(), Decimal::neg_infinity());
    }

    #[test]
    fn test_sub_with_overflow() {
        assert_sub(Decimal::min(), Decimal::from(1), Decimal::neg_infinity());
        assert_sub(Decimal::max(), Decimal::from(-1), Decimal::infinity());
    }

    #[test]
    fn test_sub_assign() {
        let mut n = Decimal::from(10);
        n -= Decimal::one();
        assert_eq!(n, Decimal::from(9));
    }

    #[test]
    fn test_sub_with_primitives() {
        assert_eq!(Decimal::one() - 5, Decimal::from(-4));
        assert_eq!(9 - Decimal::one(), Decimal::from(8));
        assert_eq!(Decimal::one() - 5.125, Decimal::with_scale(-4125, 3));
        assert_eq!(12.5 - Decimal::one(), Decimal::with_scale(115, 1));
    }

    #[test]
    fn test_sub_with_primitives_with_assign() {
        let mut n = Decimal::from(10);
        n -= 7;
        assert_eq!(n, Decimal::from(3));
        n -= 0.5;
        assert_eq!(n, Decimal::with_scale(25, 1));
    }

    #[test]
    fn test_mul_zero() {
        let vals = [
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(*v * Decimal::zero(), Decimal::zero());
            assert_eq!(Decimal::zero() * *v, Decimal::zero());
        }
    }

    #[test]
    fn test_mul_one() {
        let one = Decimal::from(1);
        let vals = [
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(2804),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(*v * one, *v);
        }
    }

    #[test]
    fn test_mul_nan() {
        let vals = [
            Decimal::nan(),
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(42),
            Decimal::min(),
            Decimal::max(),
            Decimal::infinity(),
            Decimal::neg_infinity(),
        ];
        for v in vals.iter() {
            assert!((*v * Decimal::nan()).is_nan());
        }
    }

    #[test]
    fn test_mul_infinity() {
        assert_mul(Decimal::from(42), Decimal::infinity(), Decimal::infinity());
        assert_mul(
            Decimal::from(42),
            Decimal::neg_infinity(),
            Decimal::neg_infinity(),
        );
        assert_mul(
            Decimal::from(-42),
            Decimal::infinity(),
            Decimal::neg_infinity(),
        );

        assert_mul(
            Decimal::infinity(),
            Decimal::infinity(),
            Decimal::infinity(),
        );
        assert_mul(
            Decimal::infinity(),
            Decimal::neg_infinity(),
            Decimal::neg_infinity(),
        );
        assert_mul(
            Decimal::neg_infinity(),
            Decimal::neg_infinity(),
            Decimal::infinity(),
        );

        assert!((Decimal::zero() * Decimal::infinity()).is_nan());
        assert!((Decimal::infinity() * Decimal::zero()).is_nan());
        assert!((Decimal::zero() * Decimal::neg_infinity()).is_nan());
        assert!((Decimal::neg_infinity() * Decimal::zero()).is_nan());
    }

    #[test]
    fn test_mul_integral() {
        let ones = Decimal::from(111111111u64);
        assert_mul(ones, ones, Decimal::from(12345678987654321u64));
        assert_mul(ones, Decimal::from(5), Decimal::from(555555555));
        assert_mul(
            Decimal::from(20_000_000u64),
            Decimal::from(30_000_000u64),
            Decimal::from(600_000_000_000_000u64),
        );
        assert_mul(Decimal::from(-10), Decimal::from(5), Decimal::from(-50));
        assert_mul(Decimal::from(0), Decimal::from(-1), Decimal::from(0));
    }

    #[test]
    fn test_mul_rational_ones() {
        // Multiply ones with various scales
        for (i, j) in (0..Decimal::SCALE).zip(0..Decimal::SCALE) {
            let di = Decimal::with_scale(1, i);
            let dj = Decimal::with_scale(1, j);
            let exp = if i + j > Decimal::SCALE {
                Decimal::zero()
            } else {
                Decimal::with_scale(1, i + j)
            };
            assert_mul(di, dj, exp);
        }
    }

    #[test]
    fn test_mul_rescale() {
        for s in 0..17 {
            let scaled_ten = Decimal::from(10u64.pow(s));
            for n in -100..100 {
                assert_mul(
                    Decimal::with_scale(n, s as usize),
                    scaled_ten,
                    Decimal::from(n),
                );
            }
        }
    }

    #[test]
    fn test_mul_rational() {
        assert_mul(Decimal::with_scale(25, 2), Decimal::from(4), Decimal::one());
        assert_mul(
            Decimal::with_scale(11, 16),
            Decimal::with_scale(6, 10),
            Decimal::with_scale(6, 25),
        );
        assert_mul(
            Decimal::with_scale(1428571428571429u64, 16),
            Decimal::from(7),
            Decimal::with_scale(10000000000000003u64, 16),
        );
    }

    #[test]
    fn test_mul_with_overflow() {
        assert_eq!(Decimal::max() * Decimal::from(2), Decimal::infinity());
        assert_eq!(Decimal::max() * Decimal::max(), Decimal::infinity());
        assert_eq!(Decimal::min() * Decimal::from(2), Decimal::neg_infinity());
        assert_eq!(Decimal::min() * Decimal::max(), Decimal::neg_infinity());
    }

    #[test]
    fn test_mul_assign() {
        let mut n = Decimal::from(10);
        n *= Decimal::from(2);
        assert_eq!(n, Decimal::from(20));
    }

    #[test]
    fn test_mul_with_primitives() {
        assert_eq!(Decimal::from(10) * 5, Decimal::from(50));
        assert_eq!(9 * Decimal::from(10), Decimal::from(90));
        assert_eq!(Decimal::from(10) * 2.5, Decimal::from(25));
        assert_eq!(1.25 * Decimal::from(10), Decimal::with_scale(125, 1));
    }

    #[test]
    fn test_mul_with_primitives_with_assign() {
        let mut n = Decimal::from(10);
        n *= 4;
        assert_eq!(n, Decimal::from(40));
        n *= 0.5;
        assert_eq!(n, Decimal::from(20));
    }

    #[test]
    fn test_div_one() {
        let one = Decimal::from(1);
        let vals = [
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(2804),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(*v / one, *v);
        }
    }

    #[test]
    fn test_div_nan() {
        let vals = [
            Decimal::nan(),
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(80000000),
            Decimal::min(),
            Decimal::max(),
            Decimal::infinity(),
            Decimal::neg_infinity(),
        ];
        for v in vals.iter() {
            assert!((*v / Decimal::nan()).is_nan());
            assert!((Decimal::nan() / *v).is_nan());
        }
    }

    #[test]
    fn test_div_infinity() {
        assert!((Decimal::infinity() / Decimal::infinity()).is_nan());
        assert!((Decimal::infinity() / Decimal::neg_infinity()).is_nan());
        assert!((Decimal::neg_infinity() / Decimal::infinity()).is_nan());

        assert_eq!(Decimal::infinity() / Decimal::zero(), Decimal::infinity());
        assert_eq!(
            Decimal::neg_infinity() / Decimal::zero(),
            Decimal::neg_infinity()
        );
        assert_eq!(Decimal::infinity() / Decimal::from(1), Decimal::infinity());

        assert_eq!(Decimal::zero() / Decimal::infinity(), Decimal::zero());
        assert_eq!(Decimal::from(572) / Decimal::infinity(), Decimal::zero());
        assert_eq!(Decimal::max() / Decimal::infinity(), Decimal::zero());
    }

    #[test]
    fn test_div_by_infinity() {
        assert_eq!(Decimal::zero() / Decimal::infinity(), Decimal::zero());
        assert_eq!(Decimal::from(572) / Decimal::infinity(), Decimal::zero());
        assert_eq!(Decimal::max() / Decimal::infinity(), Decimal::zero());
    }

    #[test]
    fn test_div_integral() {
        assert_eq!(Decimal::from(6) / Decimal::from(2), Decimal::from(3));
        assert_eq!(
            Decimal::from(4_000_000_000_000u64) / Decimal::from(2_000_000_000),
            Decimal::from(2_000)
        );
        assert_eq!(Decimal::from(-6) / Decimal::from(2), Decimal::from(-3));
        assert_eq!(Decimal::from(6) / Decimal::from(-2), Decimal::from(-3));
        assert_eq!(
            Decimal::from(2_000_000_000_000_000_000u64)
                / Decimal::from(2_000_000_000_000_000_000u64),
            Decimal::from(1)
        );
        assert_eq!(Decimal::max() / Decimal::min(), Decimal::from(-1));
        assert_eq!(Decimal::min() / Decimal::max(), Decimal::from(-1));
    }

    #[test]
    fn test_div_rational() {
        assert_eq!(
            Decimal::with_scale(66, 0) / Decimal::from(11),
            Decimal::with_scale(6, 0)
        );
        assert_eq!(Decimal::max() / Decimal::min(), Decimal::from(-1));
        assert_eq!(Decimal::min() / Decimal::max(), Decimal::from(-1));
        assert_eq!(Decimal::min() / Decimal::min(), Decimal::from(1));
        assert_eq!(Decimal::ulp() / Decimal::max(), Decimal::zero());
        assert_eq!(Decimal::max() / Decimal::ulp(), Decimal::infinity());
        assert_eq!(Decimal::min() / Decimal::ulp(), Decimal::neg_infinity());

        assert_eq!(
            Decimal::from(1) / Decimal::from(2_000_000_000_000_000_000u64),
            Decimal::with_scale(5, 19)
        );
        assert_eq!(Decimal::from(1) / Decimal::ulp(), Decimal::infinity());
        assert_eq!(
            Decimal::from(70) / Decimal::with_scale(7, 24),
            Decimal::infinity()
        );
        // assert_eq!(Decimal::with_scale(1, 2) / Decimal::ulp(), 100000000000000000000000); // TODO: Uncomment after adding 'from negative scale'

        assert_eq!(
            Decimal::with_scale(75, 3) / Decimal::with_scale(25, 4),
            Decimal::from(30)
        );
        assert_eq!(
            Decimal::with_scale(75, 3) / Decimal::with_scale(20, 4),
            Decimal::with_scale(375, 1)
        );
    }

    #[test]
    fn test_div_by_zero() {
        assert_eq!(Decimal::from(10) / Decimal::zero(), Decimal::infinity());
        assert_eq!(Decimal::infinity() / Decimal::zero(), Decimal::infinity());
        assert!((Decimal::zero() / Decimal::zero()).is_nan());
    }

    #[test]
    fn test_div_assign() {
        let mut n = Decimal::from(10);
        n /= Decimal::from(4);
        assert_eq!(n, Decimal::with_scale(25, 1));
    }

    #[test]
    fn test_div_with_primitives() {
        assert_eq!(Decimal::from(10) / 4, Decimal::with_scale(25, 1));
        assert_eq!(9 / Decimal::from(6), Decimal::with_scale(15, 1));
        assert_eq!(Decimal::from(17) / 4.25, Decimal::from(4));
        assert_eq!(9.5 / Decimal::from(25), Decimal::with_scale(38, 2));
    }

    #[test]
    fn test_div_with_primitives_with_assign() {
        let mut n = Decimal::from(10);
        n /= 4;
        assert_eq!(n, Decimal::with_scale(25, 1));
        n /= 0.5;
        assert_eq!(n, Decimal::from(5));
    }

    #[test]
    fn test_rem_nan() {
        let vals = [
            Decimal::nan(),
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(40000000),
            Decimal::min(),
            Decimal::max(),
            Decimal::infinity(),
            Decimal::neg_infinity(),
        ];
        for v in vals.iter() {
            assert!((*v % Decimal::nan()).is_nan());
            assert!((Decimal::nan() % *v).is_nan());
        }
    }

    #[test]
    fn test_rem_infinity() {
        let vals = [
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(80000000),
            Decimal::min(),
            Decimal::max(),
            Decimal::infinity(),
            Decimal::neg_infinity(),
        ];
        for v in vals.iter() {
            assert!((Decimal::infinity() % *v).is_nan());
            assert!((Decimal::neg_infinity() % *v).is_nan());
        }
    }

    #[test]
    fn test_rem_by_infinity() {
        let vals = [
            Decimal::zero(),
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(80000000),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(*v % Decimal::infinity(), *v);
            assert_eq!(*v % Decimal::neg_infinity(), *v);
        }
    }

    #[test]
    fn test_rem() {
        assert_eq!(Decimal::from(4) % Decimal::from(4), Decimal::zero());
        assert_eq!(Decimal::from(5) % Decimal::from(4), Decimal::from(1));
        assert_eq!(Decimal::from(6) % Decimal::from(4), Decimal::from(2));
        assert_eq!(Decimal::from(7) % Decimal::from(4), Decimal::from(3));
        assert_eq!(Decimal::from(8) % Decimal::from(4), Decimal::zero());

        assert_eq!(Decimal::from(13) % Decimal::from(5), Decimal::from(3));
        assert_eq!(Decimal::from(13) % Decimal::from(-5), Decimal::from(3));
        assert_eq!(Decimal::from(-13) % Decimal::from(5), Decimal::from(-3));
        assert_eq!(Decimal::from(-13) % Decimal::from(-5), Decimal::from(-3));

        assert_eq!(
            Decimal::with_scale(372, 2) % Decimal::with_scale(112, 2),
            Decimal::with_scale(36, 2)
        );

        assert_eq!(Decimal::max() % Decimal::max(), Decimal::zero());
        assert_eq!(Decimal::max() % Decimal::min(), Decimal::zero());
        assert_eq!(Decimal::min() % Decimal::max(), Decimal::zero());
    }

    #[test]
    fn test_rem_by_zero() {
        assert!((Decimal::from(10) % Decimal::zero()).is_nan());
        assert!((Decimal::infinity() % Decimal::zero()).is_nan());
        assert!((Decimal::zero() % Decimal::zero()).is_nan());
    }

    #[test]
    fn test_rem_assign() {
        let mut n = Decimal::from(10);
        n %= Decimal::from(7);
        assert_eq!(n, Decimal::from(3));
    }

    #[test]
    fn test_rem_with_primitives() {
        assert_eq!(Decimal::from(10) % 4, Decimal::from(2));
        assert_eq!(19 % Decimal::from(6), Decimal::one());
        assert_eq!(Decimal::from(10) % 4.5, Decimal::one());
        assert_eq!(13.5 % Decimal::from(4), Decimal::with_scale(15, 1));
    }

    #[test]
    fn test_rem_with_primitives_with_assign() {
        let mut n = Decimal::from(10);
        n %= 4;
        assert_eq!(n, Decimal::from(2));
        n %= 0.75;
        assert_eq!(n, Decimal::with_scale(5, 1));
    }

    #[test]
    fn test_neg() {
        assert!((-Decimal::nan()).is_nan());
        assert_eq!(-Decimal::infinity(), Decimal::neg_infinity());
        assert_eq!(-Decimal::one(), Decimal::from(-1));
        assert_eq!(-Decimal::zero(), Decimal::zero());
        assert_eq!(-Decimal::from(426), Decimal::from(-426));
        assert_eq!(Decimal::from(427), -Decimal::from(-427));
        assert_eq!(-Decimal::max(), Decimal::min());
        assert_eq!(-Decimal::min(), Decimal::max());
    }

    #[test]
    fn test_shr_special() {
        assert!((Decimal::nan() >> 1).is_nan());
        assert_eq!(Decimal::infinity() >> 1, Decimal::infinity());
        assert_eq!(Decimal::neg_infinity() >> 1, Decimal::neg_infinity());
    }

    #[test]
    fn test_shr() {
        assert_eq!(Decimal::with_scale(2, Decimal::SCALE) >> 1, Decimal::ulp());
        assert_eq!(Decimal::from(2) >> 1, Decimal::one());
        assert_eq!(Decimal::from(10) >> 1, Decimal::from(5));
        assert_eq!(Decimal::from(-10) >> 1, Decimal::from(-5));
        assert_eq!(Decimal::ulp() >> 1, Decimal::zero());
        assert_eq!(-Decimal::ulp() >> 1, Decimal::zero());

        assert_eq!(Decimal::from(4294967296u64) >> 32, Decimal::one());
        assert_eq!(Decimal::from(8589934592u64) >> 32, Decimal::from(2));
        assert_eq!(
            Decimal::max() >> 1,
            Decimal::from_str("73075081866545145910184.2416358141509827966271487").unwrap()
        );
        assert_eq!(Decimal::max() >> 159, Decimal::ulp());
    }

    #[test]
    fn test_shr_with_assign() {
        let mut n = Decimal::from(40);
        n >>= 2;
        assert_eq!(n, Decimal::from(10));
    }

    #[test]
    fn test_shl_special() {
        assert!((Decimal::nan() << 1).is_nan());
        assert_eq!(Decimal::infinity() << 1, Decimal::infinity());
        assert_eq!(Decimal::neg_infinity() << 1, Decimal::neg_infinity());
    }

    #[test]
    fn test_shl() {
        assert_eq!(Decimal::ulp() << 1, Decimal::with_scale(2, Decimal::SCALE));
        assert_eq!(Decimal::one() << 1, Decimal::from(2));
        assert_eq!(Decimal::from(5) << 1, Decimal::from(10));
        assert_eq!(Decimal::from(-5) << 1, Decimal::from(-10));
        assert_eq!(Decimal::zero() << 1, Decimal::zero());

        assert_eq!(Decimal::one() << 32, Decimal::from(4294967296u64));
        assert_eq!(Decimal::from(2) << 32, Decimal::from(8589934592u64));

        assert_eq!(
            Decimal::new(false, [0xaabbccdd, 0, 0, 0, 0]) << 32,
            Decimal::new(false, [0, 0xaabbccdd, 0, 0, 0])
        );
        assert_eq!(
            Decimal::new(false, [0xaabbccdd, 0, 0, 0, 0]) << 64,
            Decimal::new(false, [0, 0, 0xaabbccdd, 0, 0])
        );
        assert_eq!(
            Decimal::new(false, [0xaabbccdd, 0, 0, 0, 0]) << 72,
            Decimal::new(false, [0, 0, 0xbbccdd00, 0x000000aa, 0])
        );
        assert_eq!(
            Decimal::new(false, [0, 0, 0, 0, 0x40000000]) << 1,
            Decimal::new(false, [0, 0, 0, 0, 0x80000000])
        );
    }

    #[test]
    fn test_shl_overflow() {
        assert_eq!(Decimal::max() << 1, Decimal::infinity());
        assert_eq!(Decimal::min() << 1, Decimal::neg_infinity());

        assert_eq!(
            Decimal::new(false, [0, 0, 0, 0, 0x80000000]) << 1,
            Decimal::infinity()
        );
        assert_eq!(
            Decimal::new(false, [0, 0, 0, 0, 0x40000000]) << 2,
            Decimal::infinity()
        );
        assert_eq!(Decimal::ulp() << 160, Decimal::infinity());
    }

    #[test]
    fn test_shl_with_assign() {
        let mut n = Decimal::from(10);
        n <<= 2;
        assert_eq!(n, Decimal::from(40));
    }

    fn assert_add(a: Decimal, b: Decimal, expected: Decimal) {
        assert_eq!(a + b, expected);
        assert_eq!(b + a, expected);
    }

    fn assert_sub(a: Decimal, b: Decimal, expected: Decimal) {
        assert_eq!(a - b, expected);
        assert_eq!(b - a, -expected);
    }

    fn assert_mul(a: Decimal, b: Decimal, expected: Decimal) {
        assert_eq!(a * b, expected);
        assert_eq!(b * a, expected);
    }
}

#[cfg(test)]
mod prim {
    use super::decimal::*;
    use super::test_str;

    #[test]
    fn test_from_u8() {
        test_str(Decimal::from(0u8), "0");
        test_str(Decimal::from(1u8), "1");
        test_str(Decimal::from(42u8), "42");
        test_str(Decimal::from(108u8), "108");
        test_str(Decimal::from(u8::max_value()), "255");
    }

    #[test]
    fn test_from_u8_with_scale() {
        test_str(Decimal::with_scale(0u8, 0), "0");
        test_str(Decimal::with_scale(0u8, 3), "0");
        test_str(Decimal::with_scale(1u8, 7), "0.0000001");
        test_str(Decimal::with_scale(42u8, 1), "4.2");
        test_str(Decimal::with_scale(42u8, 2), "0.42");
        test_str(Decimal::with_scale(42u8, 3), "0.042");
    }

    #[test]
    fn test_from_i8() {
        test_str(Decimal::from(0i8), "0");
        test_str(Decimal::from(1i8), "1");
        test_str(Decimal::from(-1i8), "-1");
        test_str(Decimal::from(42i8), "42");
        test_str(Decimal::from(-42i8), "-42");
        test_str(Decimal::from(108i8), "108");
        test_str(Decimal::from(-108i8), "-108");
        test_str(Decimal::from(i8::min_value()), "-128");
        test_str(Decimal::from(i8::max_value()), "127");
    }

    #[test]
    fn test_from_i8_with_scale() {
        test_str(Decimal::with_scale(0i8, 0), "0");
        test_str(Decimal::with_scale(0i8, 4), "0");
        test_str(Decimal::with_scale(1i8, 0), "1");
        test_str(Decimal::with_scale(1i8, 1), "0.1");
        test_str(Decimal::with_scale(-1i8, 0), "-1");
        test_str(Decimal::with_scale(-1i8, 1), "-0.1");
        test_str(Decimal::with_scale(-1i8, 4), "-0.0001");
        test_str(Decimal::with_scale(42i8, 1), "4.2");
        test_str(Decimal::with_scale(-42i8, 2), "-0.42");
        test_str(Decimal::with_scale(-100i8, 1), "-10");
        test_str(Decimal::with_scale(-100i8, 2), "-1");
        test_str(Decimal::with_scale(i8::min_value(), 3), "-0.128");
        test_str(Decimal::with_scale(i8::max_value(), 3), "0.127");
    }

    #[test]
    fn test_from_u16() {
        test_str(Decimal::from(0u16), "0");
        test_str(Decimal::from(1u16), "1");
        test_str(Decimal::from(42u16), "42");
        test_str(Decimal::from(1008u16), "1008");
        test_str(Decimal::from(u16::max_value()), "65535");
    }

    #[test]
    fn test_from_i16() {
        test_str(Decimal::from(0i16), "0");
        test_str(Decimal::from(1i16), "1");
        test_str(Decimal::from(-1i16), "-1");
        test_str(Decimal::from(42i16), "42");
        test_str(Decimal::from(-42i16), "-42");
        test_str(Decimal::from(1008i16), "1008");
        test_str(Decimal::from(-1008i16), "-1008");
        test_str(Decimal::from(i16::min_value()), "-32768");
        test_str(Decimal::from(i16::max_value()), "32767");
    }

    #[test]
    fn test_from_u32() {
        test_str(Decimal::from(0u32), "0");
        test_str(Decimal::from(1u32), "1");
        test_str(Decimal::from(42u32), "42");
        test_str(Decimal::from(100008u32), "100008");
        test_str(Decimal::from(u32::max_value()), "4294967295");
    }

    #[test]
    fn test_from_u32_with_scale() {
        test_str(Decimal::with_scale(0u32, 0), "0");
        test_str(Decimal::with_scale(0u32, 4), "0");
        test_str(Decimal::with_scale(1u32, 0), "1");
        test_str(Decimal::with_scale(1u32, 1), "0.1");
        test_str(Decimal::with_scale(10u32, 1), "1");
        test_str(Decimal::with_scale(10000u32, 4), "1");
        test_str(Decimal::with_scale(10001u32, 4), "1.0001");
        test_str(Decimal::with_scale(42u32, 0), "42");
        test_str(Decimal::with_scale(42u32, 1), "4.2");
        test_str(Decimal::with_scale(42u32, 10), "0.0000000042");
        test_str(Decimal::with_scale(u32::max_value(), 0), "4294967295");
    }

    #[test]
    fn test_from_i32() {
        test_str(Decimal::from(0i32), "0");
        test_str(Decimal::from(1i32), "1");
        test_str(Decimal::from(-1i32), "-1");
        test_str(Decimal::from(42i32), "42");
        test_str(Decimal::from(-42i32), "-42");
        test_str(Decimal::from(100008i32), "100008");
        test_str(Decimal::from(-100008i32), "-100008");
        test_str(Decimal::from(i32::min_value()), "-2147483648");
        test_str(Decimal::from(i32::max_value()), "2147483647");
    }

    #[test]
    fn test_from_u64() {
        test_str(Decimal::from(0u64), "0");
        test_str(Decimal::from(1u64), "1");
        test_str(Decimal::from(42u64), "42");
        test_str(Decimal::from(10000060008u64), "10000060008");
        test_str(Decimal::from(u64::max_value()), "18446744073709551615");
    }

    #[test]
    fn test_from_u64_with_scale() {
        test_str(Decimal::with_scale(0u64, 4), "0");
        test_str(Decimal::with_scale(1u64, 2), "0.01");
        test_str(Decimal::with_scale(42u64, 1), "4.2");
        test_str(Decimal::with_scale(10000060008u64, 6), "10000.060008");
        test_str(
            Decimal::with_scale(99999999999999999u64, 12),
            "99999.999999999999",
        );
        test_str(
            Decimal::with_scale(u64::max_value(), 8),
            "184467440737.09551615",
        );
    }

    #[test]
    fn test_from_i64() {
        test_str(Decimal::from(0i64), "0");
        test_str(Decimal::from(1i64), "1");
        test_str(Decimal::from(-1i64), "-1");
        test_str(Decimal::from(42i64), "42");
        test_str(Decimal::from(-42i64), "-42");
        test_str(Decimal::from(10000060008i64), "10000060008");
        test_str(Decimal::from(-10000060008i64), "-10000060008");
        test_str(Decimal::from(i64::max_value()), "9223372036854775807");
        test_str(Decimal::from(i64::min_value()), "-9223372036854775808");
    }

    #[test]
    fn test_from_i64_with_scale() {
        test_str(Decimal::with_scale(0i64, 0), "0");
        test_str(Decimal::with_scale(0i64, 4), "0");
        test_str(Decimal::with_scale(1i64, 0), "1");
        test_str(Decimal::with_scale(1i64, 6), "0.000001");
        test_str(Decimal::with_scale(-1i64, 0), "-1");
        test_str(Decimal::with_scale(-1i64, 3), "-0.001");
        test_str(Decimal::with_scale(42i64, 1), "4.2");
        test_str(Decimal::with_scale(-42i64, 1), "-4.2");
        test_str(Decimal::with_scale(10000060008i64, 4), "1000006.0008");
        test_str(Decimal::with_scale(-10000060008i64, 7), "-1000.0060008");
        test_str(Decimal::with_scale(10000000000000000i64, 10), "1000000");
        test_str(Decimal::with_scale(-10000000000000000i64, 10), "-1000000");
        test_str(
            Decimal::with_scale(i64::max_value(), 2),
            "92233720368547758.07",
        );
        test_str(
            Decimal::with_scale(i64::min_value(), 4),
            "-922337203685477.5808",
        );
    }

    #[test]
    fn test_from_f32() {
        use std::f32::{INFINITY, NAN, NEG_INFINITY};

        // Special values
        assert!(Decimal::from(NAN).is_nan());
        assert_eq!(Decimal::from(INFINITY), Decimal::infinity());
        assert_eq!(Decimal::from(NEG_INFINITY), Decimal::neg_infinity());

        // Zero and normal values
        assert_eq!(Decimal::from(0_f32), Decimal::zero());
        assert_eq!(Decimal::from(-0_f32), Decimal::zero());
        test_str(Decimal::from(54321_f32), "54321");
        test_str(Decimal::from(-54321_f32), "-54321");
        test_str(Decimal::from(0.5_f32), "0.5");
        test_str(Decimal::from(-0.5_f32), "-0.5");
        test_str(Decimal::from(0.015625_f32), "0.015625");
        test_str(Decimal::from(0.02_f32), "0.0199999995529651641845703");
        test_str(Decimal::from(2_000_000.2_f32), "2000000.25");

        // Subnormal values
        assert_eq!(Decimal::from(1.0e-40_f32), Decimal::zero());
        assert_eq!(Decimal::from(-1.0e-40_f32), Decimal::zero());

        // With overflow
        assert_eq!(Decimal::from(1.0e+25_f32), Decimal::infinity());
        assert_eq!(Decimal::from(-1.0e+25_f32), Decimal::neg_infinity());
    }

    #[test]
    fn test_from_f64() {
        use std::f64::{INFINITY, NAN, NEG_INFINITY};

        // Special values
        assert!(Decimal::from(NAN).is_nan());
        assert_eq!(Decimal::from(INFINITY), Decimal::infinity());
        assert_eq!(Decimal::from(NEG_INFINITY), Decimal::neg_infinity());

        // Zero and normal values
        assert_eq!(Decimal::from(0_f64), Decimal::zero());
        assert_eq!(Decimal::from(-0_f64), Decimal::zero());
        test_str(Decimal::from(54321_f64), "54321");
        test_str(Decimal::from(-54321_f64), "-54321");
        test_str(Decimal::from(0.5_f64), "0.5");
        test_str(Decimal::from(-0.5_f64), "-0.5");
        test_str(Decimal::from(0.015625_f64), "0.015625");
        test_str(Decimal::from(0.02_f64), "0.0200000000000000004163336");
        test_str(
            Decimal::from(2_000_000.02_f64),
            "2000000.0200000000186264514923096",
        );

        // Subnormal values
        assert_eq!(Decimal::from(1.0e-308_f64), Decimal::zero());
        assert_eq!(Decimal::from(-1.0e-308_f64), Decimal::zero());

        // With overflow
        assert_eq!(Decimal::from(1.0e+25_f64), Decimal::infinity());
        assert_eq!(Decimal::from(-1.0e+25_f64), Decimal::neg_infinity());
    }
}

#[cfg(test)]
mod str {
    use std::str::FromStr;

    use super::decimal::*;

    #[test]
    fn test_from_str_zero() {
        assert_from_str("0", Decimal::zero());
        assert_from_str("+0", Decimal::zero());
        assert_from_str("-0", Decimal::zero());

        assert_from_str("000", Decimal::zero());
        assert_from_str("+000", Decimal::zero());
        assert_from_str("-000", Decimal::zero());

        assert_from_str("0.0", Decimal::zero());
        assert_from_str("+0.0", Decimal::zero());
        assert_from_str("-0.0", Decimal::zero());

        assert_from_str(".0", Decimal::zero());
        assert_from_str("+.0", Decimal::zero());
        assert_from_str("-.0", Decimal::zero());

        assert_from_str("000.00000", Decimal::zero());
        assert_from_str("+000.00000", Decimal::zero());
        assert_from_str("-000.00000", Decimal::zero());
    }

    #[test]
    fn test_from_str() {
        assert_from_str("1", Decimal::from(1));
        assert_from_str("+1", Decimal::from(1));
        assert_from_str("-1", Decimal::from(-1));

        assert_from_str("123", Decimal::from(123));
        assert_from_str("123", Decimal::from(123));
        assert_from_str("123000000000", Decimal::from(123_000_000_000u64));
        assert_from_str("+123000000000", Decimal::from(123_000_000_000u64));
        assert_from_str("-123000000000", Decimal::from(-123_000_000_000i64));
        assert_from_str("-98765432100", Decimal::from(-98_765_432_100i64));

        assert_from_str("123.45678", Decimal::with_scale(12345678, 5));
        assert_from_str("-123.45678", Decimal::with_scale(-12345678, 5));
        assert_from_str("-.09837263340", Decimal::with_scale(-983726334, 10));
        assert_from_str("0.01000234", Decimal::with_scale(1000234, 8));
        assert_from_str(
            ".000000000012345678901234567890123456789",
            Decimal::with_scale(123456789012345u64, 25),
        );
        assert_from_str("2000000.25", Decimal::with_scale(200_000_025, 2));

        assert_from_str(".0000000000000000000000001", Decimal::ulp());
        assert_from_str(".00000000000000000000000009", Decimal::zero());
        assert_from_str("-.0000000000000000000000001", -Decimal::ulp());
        assert_from_str("-.00000000000000000000000009", Decimal::zero());

        assert_from_str(
            "146150163733090291820368.4832716283019655932542975",
            Decimal::max(),
        );
        assert_from_str(
            "-146150163733090291820368.4832716283019655932542975",
            Decimal::min(),
        );
    }

    #[test]
    fn test_from_str_invalid_format() {
        assert_from_str_error("", ParseNumberError::InvalidFormat);
        assert_from_str_error(" ", ParseNumberError::InvalidFormat);

        assert_from_str_error("+", ParseNumberError::InvalidFormat);
        assert_from_str_error("-", ParseNumberError::InvalidFormat);

        assert_from_str_error(".", ParseNumberError::InvalidFormat);
        assert_from_str_error("+.", ParseNumberError::InvalidFormat);
        assert_from_str_error("-.", ParseNumberError::InvalidFormat);

        assert_from_str_error("A", ParseNumberError::InvalidFormat);
        assert_from_str_error("1230.239.02", ParseNumberError::InvalidFormat);
        assert_from_str_error("2..329+02", ParseNumberError::InvalidFormat);
        assert_from_str_error("23.329+02", ParseNumberError::InvalidFormat);

        assert_from_str_error("1o", ParseNumberError::InvalidFormat);
    }

    #[test]
    fn test_from_str_overflow() {
        // max + ulp
        assert_from_str_error(
            "146150163733090291820368.4832716283019655932542976",
            ParseNumberError::Overflow,
        );
        assert_from_str_error(
            "-146150163733090291820368.4832716283019655932542976",
            ParseNumberError::Overflow,
        );
        // max + 1
        assert_from_str_error("146150163733090291820369", ParseNumberError::Overflow);
        assert_from_str_error("-146150163733090291820369", ParseNumberError::Overflow);

        assert_from_str_error(
            "999999999999999999999999999999999999999999999999999",
            ParseNumberError::Overflow,
        );
    }

    fn assert_from_str(s: &str, exp: Decimal) {
        match Decimal::from_str(s) {
            Err(e) => panic!("Expected number, got {:?}", e),
            Ok(n) => assert_eq!(n, exp),
        }
    }

    fn assert_from_str_error(s: &str, err: ParseNumberError) {
        match Decimal::from_str(s) {
            Ok(d) => panic!("Error expected, {} found", d),
            Err(k) => assert_eq!(k, err),
        };
    }
}

#[cfg(test)]
mod math {
    use super::decimal::*;
    use std::str::FromStr;

    #[test]
    fn test_abs() {
        assert!(Decimal::nan().abs().is_nan());
        assert_eq!(Decimal::infinity().abs(), Decimal::infinity());
        assert_eq!(Decimal::neg_infinity().abs(), Decimal::infinity());
        assert_eq!(Decimal::zero().abs(), Decimal::zero());
        assert_eq!(Decimal::from(10).abs(), Decimal::from(10));
        assert_eq!(Decimal::from(-5).abs(), Decimal::from(5));
        assert_eq!(Decimal::min().abs(), Decimal::max());
        assert_eq!(Decimal::max().abs(), Decimal::max());
    }

    #[test]
    fn test_trunc() {
        assert!(Decimal::nan().trunc().is_nan());
        assert_eq!(Decimal::infinity().trunc(), Decimal::infinity());
        assert_eq!(Decimal::neg_infinity().trunc(), Decimal::neg_infinity());
        assert_eq!(Decimal::zero().trunc(), Decimal::zero());
        assert_eq!(Decimal::ulp().trunc(), Decimal::zero());
        assert_eq!(Decimal::one().trunc(), Decimal::one());
        assert_eq!(Decimal::with_scale(43, 1).trunc(), Decimal::from(4));
        assert_eq!(Decimal::with_scale(-423, 2).trunc(), Decimal::from(-4));
        assert_eq!(
            Decimal::max().trunc(),
            Decimal::from_str("146150163733090291820368").unwrap()
        );
        assert_eq!(
            Decimal::min().trunc(),
            Decimal::from_str("-146150163733090291820368").unwrap()
        );
    }

    #[test]
    fn test_fract() {
        assert!(Decimal::nan().fract().is_nan());
        assert_eq!(Decimal::infinity().fract(), Decimal::infinity());
        assert_eq!(Decimal::neg_infinity().fract(), Decimal::neg_infinity());
        assert_eq!(Decimal::zero().fract(), Decimal::zero());
        assert_eq!(Decimal::ulp().fract(), Decimal::ulp());
        assert_eq!(Decimal::one().fract(), Decimal::zero());
        assert_eq!(
            Decimal::with_scale(43, 1).fract(),
            Decimal::with_scale(3, 1)
        );
        assert_eq!(
            Decimal::with_scale(-423, 2).fract(),
            Decimal::with_scale(-23, 2)
        );
        assert_eq!(
            Decimal::max().fract(),
            Decimal::from_str("0.4832716283019655932542975").unwrap()
        );
        assert_eq!(
            Decimal::min().fract(),
            Decimal::from_str("-0.4832716283019655932542975").unwrap()
        );
    }

    #[test]
    fn test_sqrt() {
        assert!(Decimal::nan().sqrt().is_nan());
        assert!(Decimal::neg_infinity().sqrt().is_nan());
        assert!(Decimal::from(-3).sqrt().is_nan());
        assert_eq!(Decimal::infinity().sqrt(), Decimal::infinity());

        assert_eq!(Decimal::zero().sqrt(), Decimal::zero());
        assert_eq!(Decimal::one().sqrt(), Decimal::one());
        assert_eq!(Decimal::from(12345654321u64).sqrt(), Decimal::from(111111));
        assert_eq!(Decimal::with_scale(1, 2).sqrt(), Decimal::with_scale(1, 1));
        assert_eq!(
            Decimal::with_scale(15625, 8).sqrt(),
            Decimal::with_scale(125, 4)
        );
    }

    #[test]
    fn test_powi_special() {
        assert!(Decimal::nan().powi(0).is_nan());
        assert!(Decimal::nan().powi(1).is_nan());
        assert!(Decimal::nan().powi(-1).is_nan());

        assert_eq!(Decimal::infinity().powi(0), Decimal::one());
        assert_eq!(Decimal::infinity().powi(1), Decimal::infinity());
        assert_eq!(Decimal::infinity().powi(-1), Decimal::zero());

        assert_eq!(Decimal::neg_infinity().powi(0), Decimal::one());
        assert_eq!(Decimal::neg_infinity().powi(1), Decimal::neg_infinity());
        assert_eq!(Decimal::neg_infinity().powi(2), Decimal::infinity());
        assert_eq!(Decimal::neg_infinity().powi(-1), Decimal::zero());
    }

    #[test]
    fn test_powi_to_zero() {
        let vals = [
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(-1),
            Decimal::from(423),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(v.powi(0), Decimal::one());
        }
    }

    #[test]
    fn test_powi_to_one() {
        let vals = [
            Decimal::ulp(),
            Decimal::one(),
            Decimal::from(-1),
            Decimal::from(1_000_000),
            Decimal::min(),
            Decimal::max(),
        ];
        for v in vals.iter() {
            assert_eq!(v.powi(1), *v);
        }
    }

    #[test]
    fn test_powi_one() {
        let vals = vec![-5, -1, 0, 1, 5, 100, 1000];
        for v in vals.iter() {
            assert_eq!(Decimal::one().powi(*v), Decimal::one());
        }
    }

    #[test]
    fn test_powi() {
        let ten = Decimal::from(10);
        assert_eq!(ten.powi(0), Decimal::from(1));
        assert_eq!(ten.powi(1), Decimal::from(10));
        assert_eq!(ten.powi(2), Decimal::from(100));
        assert_eq!(ten.powi(3), Decimal::from(1000));
        assert_eq!(ten.powi(4), Decimal::from(10000));
        assert_eq!(
            ten.powi(23),
            Decimal::from_str("100000000000000000000000").unwrap()
        );

        assert_eq!(
            Decimal::with_scale(5, 1).powi(2),
            Decimal::from_str("0.25").unwrap()
        );
        assert_eq!(
            Decimal::with_scale(5, 1).powi(10),
            Decimal::from_str("0.0009765625").unwrap()
        );

        assert_eq!(
            Decimal::with_scale(1001, 2).powi(5),
            Decimal::from_str("100501.0010005001").unwrap()
        );
    }

    #[test]
    fn test_powi_to_negative() {
        let ten = Decimal::from(10);
        assert_eq!(ten.powi(-1), Decimal::from_str("0.1").unwrap());
        assert_eq!(ten.powi(-2), Decimal::from_str("0.01").unwrap());
        assert_eq!(ten.powi(-3), Decimal::from_str("0.001").unwrap());
        assert_eq!(ten.powi(-4), Decimal::from_str("0.0001").unwrap());
        assert_eq!(ten.powi(-25), Decimal::ulp());

        let tenth = Decimal::with_scale(1, 1); // 0.1
        assert_eq!(tenth.powi(-1), Decimal::from(10));
        assert_eq!(tenth.powi(-2), Decimal::from(100));
        assert_eq!(tenth.powi(-3), Decimal::from(1000));
        assert_eq!(tenth.powi(-4), Decimal::from(10000));
        assert_eq!(
            tenth.powi(-23),
            Decimal::from_str("100000000000000000000000").unwrap()
        );

        assert_eq!(Decimal::zero().powi(-2), Decimal::infinity());

        assert_eq!(
            Decimal::with_scale(107, 4).powi(-5),
            Decimal::from_str("7129861794.8366843792737120134353235").unwrap()
        );
    }

    #[test]
    fn test_powi_overflow() {
        assert_eq!(Decimal::from(10).powi(24), Decimal::infinity());
        assert_eq!(Decimal::with_scale(1, 1).powi(-24), Decimal::infinity());
    }
}
