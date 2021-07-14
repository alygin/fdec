#![feature(test)]

extern crate test;
#[macro_use]
extern crate fdec;

use std::f64::MAX as F64_MAX;
use std::f64::MIN_POSITIVE as F64_MIN_POSITIVE;
use std::str::FromStr;
use std::u64::MAX as U64_MAX;
use test::Bencher;

fdec32! {
    module decimal,
    name Decimal,
    length 5,
    scale 25
}

use decimal::*;

#[bench]
fn bench_create_from_u64(b: &mut Bencher) {
    b.iter(|| {
        Decimal::from(0u64);
        Decimal::from(1u64);
        Decimal::from(U64_MAX);
    });
}

#[bench]
fn bench_create_from_f64(b: &mut Bencher) {
    b.iter(|| {
        Decimal::from(0f64);
        Decimal::from(F64_MIN_POSITIVE);
        Decimal::from(F64_MAX);
    });
}

#[bench]
fn bench_create_from_str(b: &mut Bencher) {
    b.iter(|| {
        Decimal::from_str("0").unwrap();
        Decimal::from_str(".0000000000000000000000001").unwrap();
        Decimal::from_str("146150163733090291820368.4832716283019655932542975").unwrap();
    });
}
