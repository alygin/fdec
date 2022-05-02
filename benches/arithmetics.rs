#[macro_use]
extern crate fdec;
extern crate criterion;

use std::u64::MAX as U64_MAX;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fdec32! {
    module decimal,
    name Decimal,
    length 5,
    scale 25
}

use decimal::*;

fn bench_add(c: &mut Criterion) {
    let a = black_box(Decimal::from(U64_MAX));
    c.bench_function("add", |b| b.iter(|| a + a));
}

fn bench_multiply(c: &mut Criterion) {
    let one = black_box(Decimal::from(1));
    c.bench_function("multiply", |b| b.iter(|| Decimal::max() * one));
}

criterion_group!(benches, bench_add, bench_multiply);
criterion_main!(benches);
