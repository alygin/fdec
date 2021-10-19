#[macro_use]
extern crate fdec;

fdec16! {
    module dec,
    name Dec,
    length 1,
    scale 3
}

use dec::*;

#[test]
fn test_from_unit_boundaries() {
    assert_eq!(Dec::from(65_u16), Dec::from_le_units(false, [65000]));
    assert_eq!(Dec::from(-65_i16), Dec::from_le_units(true, [65000]));
}

#[test]
fn test_from_unit_overflow() {
    assert_eq!(Dec::from(66_u16), Dec::infinity());
    assert_eq!(Dec::with_scale(6601_u16, 2), Dec::infinity());
    assert_eq!(Dec::from(-66_i16), Dec::neg_infinity());
    assert_eq!(Dec::with_scale(-6601_i16, 2), Dec::neg_infinity());
}
