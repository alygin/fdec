#[macro_use]
extern crate fdec;

// 160-bit numbers with 25 decimal places.
fdec32! {
    module dec,
    name Decimal,
    length 5,
    scale 25
}

use dec::*;

const N: u64 = 7_637_890_381;
const MAX_ITER: usize = 100;

/// Calculates square root of the given number `N` using the Newton's method with no more
/// than `MAX_ITER` iterations, and compares it to what f64::sqrt() produces.
fn main() {
    let n = Decimal::from(N);
    let mut x = n >> 1;
    let mut prev_x = Decimal::zero();
    let mut i = 0;
    while i < MAX_ITER && x != prev_x {
        prev_x = x;
        x = (x + n / x) >> 1;
        i += 1;
    }

    let fdec_msg = format!("sqrt({}) is {}", N, x);
    let f64_msg = format!("sqrt({}) is {:.25}", N, (N as f64).sqrt());
    println!("fdec: {}", fdec_msg);
    println!("f64:  {}", f64_msg);
    print!("      ");
    print_diff(&fdec_msg, &f64_msg);
}

fn print_diff(s1: &str, s2: &str) {
    for (d1, d2) in s1.chars().zip(s2.chars()) {
        if d1 == ',' {
            break;
        }
        if d1 != d2 {
            println!("^ f64 breaks here");
            break;
        }
        print!(" ");
    }
}

/* =================== Output ===================

fdec: sqrt(7637890381) is 87395.0249213306191846225143944
f64:  sqrt(7637890381) is 87395.0249213306233286857604980
                                          ^ f64 breaks here

==============================================*/
