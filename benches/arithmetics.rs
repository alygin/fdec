#![feature(test)]

extern crate test;
#[macro_use]
extern crate fdec;

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
fn bench_add(b: &mut Bencher) {
    let a = Decimal::from(U64_MAX);
    b.iter(|| a + a);
}

#[bench]
fn bench_multiply(b: &mut Bencher) {
    let one = Decimal::from(1);
    b.iter(|| Decimal::max() * one);
}
