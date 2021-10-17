//! Set of tests for methods that export fdec numbers to byte arrays or create fdec numbers from byte/unit arrays.
#[macro_use]
extern crate fdec;

fdec32! {
    module dec,
    name Dec,
    length 3,
    scale 5
}

use dec::*;
use std::ops::Neg;

#[test]
fn test_to_be_bytes() {
    assert_eq!(Dec::zero().to_be_bytes(), [0; 13]);
    assert_eq!(
        Dec::ulp().to_be_bytes(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
    );
    assert_eq!(
        Dec::ulp().neg().to_be_bytes(),
        [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
    );
    assert_eq!(
        Dec::nan().to_be_bytes(),
        [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
    assert_eq!(
        Dec::infinity().to_be_bytes(),
        [0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
    assert_eq!(
        Dec::neg_infinity().to_be_bytes(),
        [0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
    assert_eq!(
        Dec::from_le_units(true, [0xC1_C2_C3_C4, 0xB1_B2_B3_B4, 0xA1_A2_A3_A4]).to_be_bytes(),
        [0x01, 0xA1, 0xA2, 0xA3, 0xA4, 0xB1, 0xB2, 0xB3, 0xB4, 0xC1, 0xC2, 0xC3, 0xC4]
    );
}

#[test]
fn test_to_le_bytes() {
    assert_eq!(Dec::zero().to_le_bytes(), [0; 13]);
    assert_eq!(
        Dec::ulp().to_be_bytes(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
    );
    assert_eq!(
        Dec::ulp().neg().to_le_bytes(),
        [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
    );
    assert_eq!(
        Dec::nan().to_le_bytes(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]
    );
    assert_eq!(
        Dec::infinity().to_le_bytes(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04]
    );
    assert_eq!(
        Dec::neg_infinity().to_le_bytes(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05]
    );
    assert_eq!(
        Dec::from_le_units(true, [0xC1_C2_C3_C4, 0xB1_B2_B3_B4, 0xA1_A2_A3_A4]).to_le_bytes(),
        [0xC4, 0xC3, 0xC2, 0xC1, 0xB4, 0xB3, 0xB2, 0xB1, 0xA4, 0xA3, 0xA2, 0xA1, 0x01]
    );
}

#[test]
fn test_from_le_units() {
    assert_eq!(Dec::from_le_units(false, [0; 3]), Dec::zero());
    assert_eq!(Dec::from_le_units(false, [1, 0, 0]), Dec::ulp());
    assert_eq!(Dec::from_le_units(false, [u32::MAX; 3]), Dec::max());
    assert_eq!(Dec::from_le_units(true, [u32::MAX; 3]), Dec::min());
    assert_eq!(
        Dec::from_le_units(false, [9, 3, 0]).to_string(),
        "128849.01897"
    );
}

#[test]
fn test_from_be_units() {
    assert_eq!(Dec::from_be_units(false, [0; 3]), Dec::zero());
    assert_eq!(Dec::from_be_units(false, [0, 0, 1]), Dec::ulp());
    assert_eq!(Dec::from_be_units(false, [u32::MAX; 3]), Dec::max());
    assert_eq!(Dec::from_be_units(true, [u32::MAX; 3]), Dec::min());
    assert_eq!(
        Dec::from_be_units(false, [0, 3, 9]).to_string(),
        "128849.01897"
    );
}

#[test]
fn test_from_be_bytes() {
    assert_eq!(Dec::from_be_bytes(&[0; 13]).unwrap(), Dec::zero());

    let ulp = Dec::from_be_bytes(&[0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x01]).unwrap();
    assert_eq!(ulp, Dec::ulp());

    let val = Dec::from_be_bytes(&[
        0x01, 0xA1, 0xA2, 0xA3, 0xA4, 0xB1, 0xB2, 0xB3, 0xB4, 0xC1, 0xC2, 0xC3, 0xC4,
    ])
    .unwrap();
    assert_eq!(
        val,
        Dec::from_le_units(true, [0xC1_C2_C3_C4, 0xB1_B2_B3_B4, 0xA1_A2_A3_A4])
    );
}

#[test]
fn test_from_be_bytes_special_numbers() {
    let nan = Dec::from_be_bytes(&[0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert!(nan.is_nan());

    let infinity = Dec::from_be_bytes(&[0x04, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert!(infinity.is_infinite());
    assert!(!infinity.is_sign_negative());

    let neg_infinity = Dec::from_be_bytes(&[0x05, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert!(neg_infinity.is_infinite());
    assert!(neg_infinity.is_sign_negative());
}

#[test]
fn test_to_from_be_bytes() {
    assert_to_from_be_bytes(Dec::zero());
    assert_to_from_be_bytes(Dec::ulp());
    assert_to_from_be_bytes(Dec::one());
    assert_to_from_be_bytes(Dec::infinity());
    assert_to_from_be_bytes(Dec::neg_infinity());
    assert_to_from_be_bytes(*dec::consts::PI);
    assert_to_from_be_bytes(*dec::consts::E);
}

#[test]
fn test_from_be_bytes_invalid() {
    assert_eq!(
        Dec::from_be_bytes(&[0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // NaN & Infinite at the same time
        Err(FromBytesError::InvalidFlags)
    );
    assert_eq!(
        Dec::from_be_bytes(&[0x08, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), // Unsupported flag is filled
        Err(FromBytesError::InvalidFlags)
    );
}

#[test]
fn test_from_le_bytes() {
    assert_eq!(Dec::from_le_bytes(&[0; 13]).unwrap(), Dec::zero());

    let ulp = Dec::from_le_bytes(&[0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x00]).unwrap();
    assert_eq!(ulp, Dec::ulp());

    let val = Dec::from_le_bytes(&[
        0xC4, 0xC3, 0xC2, 0xC1, 0xB4, 0xB3, 0xB2, 0xB1, 0xA4, 0xA3, 0xA2, 0xA1, 0x01,
    ])
    .unwrap();
    assert_eq!(
        val,
        Dec::from_le_units(true, [0xC1_C2_C3_C4, 0xB1_B2_B3_B4, 0xA1_A2_A3_A4])
    );
}

#[test]
fn test_from_le_bytes_special_numbers() {
    let nan = Dec::from_le_bytes(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x02]).unwrap();
    assert!(nan.is_nan());

    let infinity = Dec::from_le_bytes(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x04]).unwrap();
    assert!(infinity.is_infinite());
    assert!(!infinity.is_sign_negative());

    let neg_infinity = Dec::from_le_bytes(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x05]).unwrap();
    assert!(neg_infinity.is_infinite());
    assert!(neg_infinity.is_sign_negative());
}

#[test]
fn test_to_from_le_bytes() {
    assert_to_from_le_bytes(Dec::zero());
    assert_to_from_le_bytes(Dec::ulp());
    assert_to_from_le_bytes(Dec::one());
    assert_to_from_le_bytes(Dec::infinity());
    assert_to_from_le_bytes(Dec::neg_infinity());
    assert_to_from_le_bytes(*dec::consts::PI);
    assert_to_from_le_bytes(*dec::consts::E);
}

fn assert_to_from_be_bytes(value: Dec) {
    let bytes = value.to_be_bytes();
    let value_from = Dec::from_be_bytes(&bytes).unwrap();
    assert_eq!(value_from, value);
}

fn assert_to_from_le_bytes(value: Dec) {
    let bytes = value.to_le_bytes();
    let value_from = Dec::from_le_bytes(&bytes).unwrap();
    assert_eq!(value_from, value);
}
