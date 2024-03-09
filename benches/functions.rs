#[macro_use]
extern crate fdec;
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

fdec32! {
    module dec,
    name Decimal,
    length 5,
    scale 25
}

use dec::*;

fn sqrt(c: &mut Criterion) {
    const A: u32 = 500_000_000;
    const N: usize = 20;

    let mut x = Decimal::zero();
    c.bench_function("sqrt", |b| {
        b.iter(|| {
            let two = Decimal::from(2);
            let a = Decimal::from(A);
            x = a / two;
            for _ in 0..N {
                x = (x + a / x) / two;
            }
        })
    });

    assert_eq!(x.to_string(), "22360.6797749978969640917366873");
}

fn powi(c: &mut Criterion) {
    const N: i32 = 20;
    const V: u32 = 10;

    let v = Decimal::from(V);
    let mut x = Decimal::default();
    c.bench_function("powi", |b| {
        b.iter(|| {
            x = v.powi(N);
        })
    });

    assert_eq!(x.to_string(), "100000000000000000000");
}

criterion_group!(benches, sqrt, powi);
criterion_main!(benches);
