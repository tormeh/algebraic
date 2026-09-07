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

## Benchmarks

Measured with [Criterion](https://github.com/bheisler/criterion.rs) on an AMD
Ryzen 75800X3D with `rustc 1.98.0`, operating on slices of 65,536 elements with
`RUSTFLAGS="-C target-cpu=x86-64-v3"` (see[`benches/arithmetic.rs`](benches/arithmetic.rs)):

| Operation       | `f32`   | `af32`  | Speedup    | `f64`    | `af64`   | Speedup    |
|-----------------|---------|---------|------------|----------|----------|------------|
| `sum`           | 44.4 µs | 1.87 µs | **~23.8x** | 44.6 µs  | 4.64 µs  | **~9.6x**  |
| `product`       | 44.4 µs | 1.86 µs | **~23.9x** | 44.5 µs  | 4.33 µs  | **~10.3x** |
| dot product     | 44.7 µs | 4.16 µs | **~10.7x** | 44.7 µs  | 8.75 µs  | **~5.1x**  |
| elementwise add | 7.07 µs | 6.79 µs | ~1.04x     | 15.38 µs | 15.30 µs | ~1.01x     |

These numbers are hardware- and compiler-version-dependent; run `just bench-v3` (adjust the
`target` compiler flag to your CPU) to measure on your own machine.

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
