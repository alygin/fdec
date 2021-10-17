#[macro_use]
extern crate fdec;

// 160-bit numbers with 25 decimal places.
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
    print(*dec::consts::E, "Euler's number (e)");
    print(*dec::consts::PI, "Archimedes’ constant (π)");
    print(*dec::consts::SQRT_2, "Sqrt(2)");
    print(*dec::consts::FRAC_1_SQRT_2, "1 / Sqrt(2)");
    println!("...and many more.");

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

    println!("\nFrom arrays");
    print(
        Decimal::from_be_units(false, [0, 0, 17, 3, 2]),
        "From a unit array (BE)",
    );
    print(
        Decimal::from_le_units(false, [2, 3, 17, 0, 0]),
        "From a unit array (LE)",
    );
    print(
        Decimal::from_be_bytes(&[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0A, 0x12, 0x9B, 0x54,
        ])
        .unwrap(),
        "From a byte array (BE)",
    );
    print(
        Decimal::from_le_bytes(&[
            0x54, 0x9B, 0x12, 0x0A, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
        .unwrap(),
        "From a byte array (LE)",
    );
}

fn print(n: Decimal, name: &str) {
    println!("  {:25}: {}", name, n);
}

fn print_result(res: Result<Decimal, ParseNumberError>, name: &str) {
    match res {
        Ok(n) => println!("  {:25}: {}", name, n),
        Err(e) => println!("  {:25}: Error: {:?}", name, e),
    }
}

/* =================== Output ===================

Predefined constants
  Zero                     : 0
  Ulp                      : 0.0000000000000000000000001
  One                      : 1
  Maximum                  : 146150163733090291820368.4832716283019655932542975
  Minimum                  : -146150163733090291820368.4832716283019655932542975

Basic math constants
  Euler's number (e)       : 2.7182818284590452353602875
  Archimedes’ constant (π) : 3.1415926535897932384626434
  Sqrt(2)                  : 1.4142135623730950488016887
  1 / Sqrt(2)              : 0.7071067811865475244008444
...and many more.

Special values
  Not a number             : NaN
  +Infinity                : Infinity
  -Infinity                : -Infinity

From primitives
  One                      : 1
  Minus ten                : -10
  Quarter                  : 0.25
  10^25 is too big         : Infinity
  Coerced 7                : 7
  Two from macro           : 2

From primitives with scaling
  Half                     : 0.5
  One percent              : 0.01
  72 * 10^(-20)            : 0.00000000000000000072
  1.2 from macro           : 1.2

From strings
  Billion                  : 1000000000
  Three quaters            : 0.75
  -3%                      : -0.03
  10^25 is too big         : Error: Overflow
  Foo                      : Error: InvalidFormat

Negation
  -Ulp                     : -0.0000000000000000000000001
  Minus ten                : -10
  -0 is 0                  : 0

From arrays
  From a unit array (BE)   : 0.0000313594649265947279362
  From a unit array (LE)   : 0.0000313594649265947279362
  From a byte array (BE)   : 0.0000000000000000168991572
  From a byte array (LE)   : 0.0000000000000000168991572

==============================================*/
