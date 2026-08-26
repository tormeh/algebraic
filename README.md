# algebraic

Tired of using `.algebraic_add()` and its siblings?

`algebraic` provides ergonomic algebraic (auto-vectorizable) floating-point wrapper types (`Algebraic<T>`) and convenience type aliases (`af32`, `af64`, `af16`, `af128`) for Rust floating-point types.

## Why Use `algebraic`?

Standard floating-point types (like `f32` and `f64`) follow strict IEEE 754 precision rules. Under these rules, floating-point addition and multiplication are not associative due to precision limits and rounding errors. Because of this, the Rust compiler (via LLVM) must evaluate mathematical operations in the exact order specified by the source code. This restriction has a major performance cost: it prevents auto-vectorization.

The recently stabilized algebraic methods for floating-point types in Rust solves this, but they can be tedious to write, and they are never invoked when using iterator methods. This library addresses this by wrapping floating-point primitives and always invoking the algebraic intrinsics under the hood. That's it. It's just convenience.

## Features

- Full support for standard arithmetic (`+`, `-`, `*`, `/`, `%`) and assign (`+=`, `-=`, `*=`, `/=`, `%=`) operators across:
  - Owned types (`af64 + af64`)
  - Borrowed types (`&af64 + &af64`)
  - Mixed references (`af64 + &af64`)
  - Primitive-to-algebraic operations (`af64 + f64`)
- Full support for `Sum` and `Product` traits, allowing you to sum or multiply standard floating-point iterators directly into an `Algebraic<T>`.

## Example

```rust
use algebraic::af32;

// Construct algebraic floats using `new` or `From::from`
let a = af32::new(1.5);
let b = af32::from(2.0);

// Perform standard arithmetic operations which compile down to algebraic intrinsics,
// allowing the compiler to aggressively reorder and vectorize.
let mut c = a * b + af32::new(0.5);
assert_eq!(c.value(), 3.5);

c += af32::new(1.0);
assert_eq!(f32::from(c), 4.5);

// Works seamlessly with iterators for operations like sum and product
let values = [1.0, 2.0, 3.0, 4.0].map(af32::new);
let sum: af32 = values.into_iter().sum();
assert_eq!(sum.value(), 10.0);
```

## Feature Flags and Requirements

| Feature | Supported Types | Rust Toolchain |
|---|---|---|
| *(None)* | `f32` (`af32`), `f64` (`af64`) | **Stable (1.98+)** |
| `f16` | `f16` (`af16`) | **Nightly** |
| `f128` | `f128` (`af128`) | **Nightly** |

## Disclaimer

I'm just a guy. Should you use this for serious stuff? Maybe, maybe not.
