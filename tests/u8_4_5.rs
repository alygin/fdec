#[macro_use]
extern crate fdec;

fdec8! {
    module dec,
    name Decimal,
    length 4,
    scale 5
}

use dec::*;
use std::str::FromStr;

#[test]
fn test_const() {
    assert_eq!(Decimal::zero(), Decimal::from(0));
    assert_eq!(Decimal::ulp(), Decimal::with_scale(1, 5));
    assert_eq!(Decimal::one(), Decimal::from(1));
    assert_eq!(Decimal::max(), Decimal::from_str("42949.67295").unwrap());
    assert_eq!(Decimal::min(), Decimal::from_str("-42949.67295").unwrap());
}

#[test]
fn test_macro() {
    assert_eq!(dec!(75), Decimal::from(75));
    assert_eq!(dec!(75, 1), Decimal::with_scale(75, 1));
}

#[test]
fn test_from_str() {
    assert_eq!(Decimal::from_str("0").unwrap(), Decimal::zero());
    assert_eq!(Decimal::from_str("1").unwrap(), Decimal::one());
    assert_eq!(Decimal::from_str("0.00001").unwrap(), Decimal::ulp());
    assert_eq!(Decimal::from_str("42949.67295").unwrap(), Decimal::max());
    assert_eq!(
        Decimal::from_str("42949.67296").unwrap_err(),
        ParseNumberError::Overflow
    );
    assert_eq!(
        Decimal::from_str("999999999999999999999").unwrap_err(),
        ParseNumberError::Overflow
    );
}

#[test]
fn test_from_prim() {
    assert_eq!(Decimal::from(0_u8), Decimal::zero());
    assert_eq!(Decimal::from(17_u8), Decimal::from_str("17").unwrap());

    assert_eq!(Decimal::from(0_u16), Decimal::zero());
    assert_eq!(Decimal::from(17_u16), Decimal::from_str("17").unwrap());
    assert_eq!(
        Decimal::from(42949_u16),
        Decimal::from_str("42949").unwrap()
    );
    assert_eq!(Decimal::from(42950_u16), Decimal::infinity());
    assert_eq!(Decimal::from(u16::max_value()), Decimal::infinity());

    assert_eq!(Decimal::from(0_u32), Decimal::zero());
    assert_eq!(Decimal::from(17_u32), Decimal::from_str("17").unwrap());
    assert_eq!(
        Decimal::from(42949_u32),
        Decimal::from_str("42949").unwrap()
    );
    assert_eq!(Decimal::from(42950_u32), Decimal::infinity());
    assert_eq!(Decimal::from(u32::max_value()), Decimal::infinity());

    assert_eq!(Decimal::from(0_u64), Decimal::zero());
    assert_eq!(Decimal::from(17_u64), Decimal::from_str("17").unwrap());
    assert_eq!(
        Decimal::from(42949_u64),
        Decimal::from_str("42949").unwrap()
    );
    assert_eq!(Decimal::from(42950_u64), Decimal::infinity());
    assert_eq!(Decimal::from(u64::max_value()), Decimal::infinity());
}

#[test]
fn test_to_string() {
    assert_eq!(Decimal::zero().to_string(), "0");
    assert_eq!(Decimal::ulp().to_string(), "0.00001");
    assert_eq!(Decimal::one().to_string(), "1");
    assert_eq!(Decimal::max().to_string(), "42949.67295");
    assert_eq!(Decimal::min().to_string(), "-42949.67295");
    assert_eq!(Decimal::from(42949_u64).to_string(), "42949");
}

#[test]
fn test_constants() {
    assert_eq!(dec::consts::E.to_string(), "2.71828");
    assert_eq!(dec::consts::PI.to_string(), "3.14159");
}
