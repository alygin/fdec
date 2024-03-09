#[macro_use]
extern crate fdec;
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::f64::MAX as F64_MAX;
use std::f64::MIN_POSITIVE as F64_MIN_POSITIVE;
use std::str::FromStr;
use std::u64::MAX as U64_MAX;

fdec32! {
    module decimal,
    name Decimal,
    length 5,
    scale 25
}

use decimal::*;

fn bench_create_from_u64(c: &mut Criterion) {
    c.bench_function("create_from_u64", |b| {
        b.iter(|| {
            black_box(Decimal::from(0u64));
            black_box(Decimal::from(1u64));
            black_box(Decimal::from(U64_MAX));
        })
    });
}

fn bench_create_from_f64(c: &mut Criterion) {
    c.bench_function("create_from_f64", |b| {
        b.iter(|| {
            black_box(Decimal::from(0f64));
            black_box(Decimal::from(F64_MIN_POSITIVE));
            black_box(Decimal::from(F64_MAX));
        })
    });
}

fn bench_create_from_str(c: &mut Criterion) {
    c.bench_function("create_from_str", |b| {
        b.iter(|| {
            Decimal::from_str("0").unwrap();
            Decimal::from_str(".0000000000000000000000001").unwrap();
            Decimal::from_str("146150163733090291820368.4832716283019655932542975").unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_create_from_u64,
    bench_create_from_f64,
    bench_create_from_str
);
criterion_main!(benches);
