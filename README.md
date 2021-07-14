# fdec

A collection of macros for generating fixed-size fixed-point numeric types
that exactly fit your domain. The types are fully equipped for performing mathematical
computations and are easy to use.

With a simple macro call you get a type that:

- has no representation errors in the range, defined by the type parameters,
- supports arithmetic operations: `+`, `-`, `*`, `/`, `%`, `<<`, `>>`,
- comes with mathematical functions: `abs()`, `powi()`, `sqrt()`,
- has special values NaN and ±Infinity, and uses them instead of panicing,
- seamlessly interacts with Rust's primitive types,
- doesn't use heap allocations for math operations.

## Usage

First, add the dependency on `fdec` to your `Cargo.toml`:

```toml
[dependencies]
fdec = "0.1"
```

Second, import it at your crate's root with the `macro_use` attribute:

```rust
#[macro_use]
extern crate fdec;
```

Now, everything is ready for adding custom numeric types to your project.

## Example

Here, we define the `Decimal` structure that represents 160-bit numbers
with 30 decimal places.

```rust
#[macro_use]
extern crate fdec;

fdec32! {            // Use 32-bit units
    module dec,      // Put all the generated code into the `dec` module
    name Decimal,    // Name the main struct `Decimal`
    length 5,        // 5 * 32-bit units = 160 bits to store numbers
    scale 30         // Use 30 decimal places
}

use dec::*;          // Bring the generated stuff to the scope

fn main() {
    // Use it
    let a = Decimal::one();
    let b = Decimal::from(14);
    let c = dec!(9);
    let result = a + 30 * (b / c).powi(3);
    println!("{} + 30 * ({} / {})^3 = {}", a, b, c, result);
    // 1 + 30 * (14 / 9)^3 = 113.92181069958847736625514403278
}
```

More examples come with the crate's source code:
- Many ways to create values: [creation.rs](https://github.com/alygin/fdec/tree/master/examples/creation.rs)
- Compute Fibonacci numbers: [fibonacci.rs](https://github.com/alygin/fdec/tree/master/examples/fibonacci.rs)
- Calculate square root with high precision: [sqrt.rs](https://github.com/alygin/fdec/tree/master/examples/sqrt.rs)
