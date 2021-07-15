#[macro_use]
extern crate fdec;

fdec32! {
    module int,
    name Int,
    length 4,
    scale 0
}

use int::*;
use std::str::FromStr;

#[test]
fn test_const() {
    assert_eq!(Int::zero(), Int::from(0));
    assert_eq!(Int::ulp(), Int::from(1));
    assert_eq!(Int::one(), Int::from(1));
    assert_eq!(
        Int::max(),
        Int::from_str("340282366920938463463374607431768211455").unwrap()
    );
    assert_eq!(
        Int::min(),
        Int::from_str("-340282366920938463463374607431768211455").unwrap()
    );
}

#[test]
fn test_macro() {
    assert_eq!(int!(77), Int::from(77));
}

#[test]
fn test_constants() {
    assert_eq!(*int::consts::E, Int::from(3));
    assert_eq!(*int::consts::PI, Int::from(3));
}
