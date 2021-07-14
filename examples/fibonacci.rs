#[macro_use]
extern crate fdec;

// 160-bit integer number.
fdec32! {
    module int,
    name BigInt,
    length 5
}

use int::*;

/// Prints Fibonacci numbers until hits the maximum possible value.
fn main() {
    println!("Fibonacci numbers");
    let mut a = BigInt::zero();
    let mut b = BigInt::one();
    while !b.is_infinite() {
        println!("{}", b);
        let tmp = b;
        b += a;
        a = tmp;
    }
}
