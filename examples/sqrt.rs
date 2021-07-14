#[macro_use]
extern crate fdec;

// 160-bit decimal numbers with 25 decimal places.
fdec32! {
    module decimal,
    name Decimal,
    length 5,
    scale 25
}

use decimal::*;

const N: u64 = 5_000_000_000_000;
const MAX_ITER: usize = 100;

/// Calculates square root of the given number `N` using the Newton's method with no more
/// than `MAX_ITER` iterations.
fn main() {
    let (result, iterations) = if N == 0 {
        (Decimal::zero(), 0)
    } else {
        let n = Decimal::from(N);
        let mut x = n >> 1;
        let mut prev_x = Decimal::zero();
        let mut i = 0;
        while i < MAX_ITER && x != prev_x {
            prev_x = x;
            x = (x + n / x) >> 1;
            i += 1;
        }
        (x, i)
    };

    println!(
        "sqrt({}) is {}, calculated in {} iteration(s)",
        N, result, iterations
    );
}
