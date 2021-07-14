#![feature(test)]

extern crate test;
#[macro_use]
extern crate fdec;

use test::Bencher;

fdec32! {
    module dec,
    name Decimal,
    length 5,
    scale 25
}

use dec::*;

#[bench]
fn sqrt(b: &mut Bencher) {
    const A: u32 = 500_000_000;
    const N: usize = 20;

    let mut x = Decimal::zero();
    b.iter(|| {
        let two = Decimal::from(2);
        let a = Decimal::from(A);
        x = a / two;
        for _ in 0..N {
            x = (x + a / x) / two;
        }
    });

    assert_eq!(x.to_string(), "22360.6797749978969640917366873");
}

#[bench]
fn powi(b: &mut Bencher) {
    const N: i32 = 20;
    const V: u32 = 10;

    let v = Decimal::from(V);
    let mut x = Decimal::default();
    b.iter(|| {
        x = v.powi(N);
    });

    assert_eq!(x.to_string(), "100000000000000000000");
}
