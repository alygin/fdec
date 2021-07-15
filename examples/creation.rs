#[macro_use]
extern crate fdec;

fdec32! {
    module dec,
    name Decimal,
    length 5,
    scale 25
}

use std::str::FromStr;

use dec::*;

/// Shows various methods to create numbers.
fn main() {
    println!("Predefined constants");
    // These constants always present in such types.
    print(Decimal::zero(), "Zero");
    print(Decimal::ulp(), "Ulp");
    print(Decimal::one(), "One");
    print(Decimal::max(), "Maximum");
    print(Decimal::min(), "Minimum");

    println!("\nBasic math constants");
    // Provides the same basic math constants as Rust's primitive types.
    print(*dec::consts::E, "Euler's number");
    print(*dec::consts::PI, "Archimedes’ constant (π)");
    print(*dec::consts::SQRT_2, "Sqrt(2)");
    print(*dec::consts::FRAC_1_SQRT_2, "1 / Sqrt(2)");
    // ...and many more.

    println!("\nSpecial values");
    // Special values are also supported.
    print(Decimal::nan(), "Not a number");
    print(Decimal::infinity(), "+Infinity");
    print(Decimal::neg_infinity(), "-Infinity");

    println!("\nFrom primitives");
    // Numbers can be created from primitive numeric types.
    print(Decimal::from(1), "One");
    print(Decimal::from(-10), "Minus ten");
    print(Decimal::from(0.25), "Quarter");
    print(Decimal::from(1.0e+25_f64), "10^25 is too big");
    print(7.into(), "Coerced 7");
    // There's also a special macro generated to simplify creating numbers from primitives.
    print(dec!(2), "Two from macro");

    println!("\nFrom primitives with scaling");
    // You can provide an integer with a scale to create a decimal number.
    print(Decimal::with_scale(5, 1), "Half");
    print(Decimal::with_scale(1, 2), "One percent");
    print(Decimal::with_scale(72, 20), "72 * 10^(-20)");
    print(dec!(12, 1), "1.2 from macro");

    println!("\nFrom strings");
    // Strings in decimal notation can also be used to produce numbers.
    // Don't forget to `use std::str::FromStr`.
    print_result(Decimal::from_str("1000000000"), "Billion");
    print_result(Decimal::from_str("0.75"), "Three quaters");
    print_result(Decimal::from_str("-0.03"), "-3%");
    print_result(
        Decimal::from_str("10000000000000000000000000"),
        "10^25 is too big",
    );
    print_result(Decimal::from_str("foo"), "Foo");

    println!("\nNegation");
    // One can negate a number to create a new number.
    print(-Decimal::ulp(), "-Ulp");
    print(-Decimal::from(10), "Minus ten");
    print(-Decimal::zero(), "-0 is 0");
}

fn print(n: Decimal, name: &str) {
    println!("  {:25}: {}", name, n);
}

fn print_result(res: Result<Decimal, ParseNumberError>, name: &str) {
    match res {
        Ok(n) => println!("  {:25}: {}", name, n),
        Err(e) => println!("  {:25}: {:?}", name, e),
    }
}
