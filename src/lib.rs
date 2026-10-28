#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]
#![no_std]
#![warn(missing_docs)]

//! # Algebraic Floating-Point Operations Library
//!
//! `algebraic` provides ergonomic [`Algebraic<T>`] wrappers for Rust's primitive floating-point types.
//! It enables algebraic reordering optimizations (analogous to the compiler's `-ffast-math` or
//! `-funsafe-math-optimizations` flags) on standard arithmetic operations by leveraging Rust's
//! experimental algebraic float intrinsics.
//!
//! Ideally these should work like normal float types, but faster and less deterministic.
//!
//! ## Features & Toolchain Requirements
//!
//! - **f32 and f64**: Supported on stable Rust (version 1.98+).
//! - **f16 and f128** (Feature-gated): Require a nightly toolchain and the unstable features `f16` / `f128`.
//! - **serde** (Feature-gated): Adds `Serialize`/`Deserialize` implementations for [`Algebraic<T>`],
//!   serializing/deserializing exactly as the wrapped primitive would. Works without `std`.
//! - **num** (Feature-gated): Adds `num-traits` implementations, including `Float` and `FloatCore`, for supported
//!   algebraic floating-point types.
//!
//! To use the unstable types, add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! algebraic = { version = "*", features = ["f16", "f128"] }
//! ```
//!
//! To enable `serde` support, add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! algebraic = { version = "*", features = ["serde"] }
//! ```
//!
//! ## Example
//!
//! ```rust
//! use algebraic::af32;
//!
//! // Perform standard arithmetic operations which compile down to algebraic intrinsics,
//! // allowing the compiler to aggressively reorder and vectorize.
//! let a: af32 = 2.0_f32.into();
//! let mut b: af32 = 1.5 * a + 0.5;
//! assert_eq!(b.value(), 3.5);
//!
//! b += 1.0;
//! assert_eq!(b.value(), 4.5);
//!
//! // Works seamlessly with iterators for operations like sum and product
//! let values = [1.0, 2.0, 3.0, 4.0].map(af32::new);
//! let sum: af32 = values.into_iter().sum();
//! assert_eq!(sum.value(), 10.0);
//! ```
//!
//! ## Swapping between primitive and algebraic floats
//!
//! The concrete types (`af32`, `af64`, ...) provide the constants and methods that the primitive
//! float types have in `core`, such as `EPSILON`, `abs`, `max`, `clamp` and `to_bits`. These take
//! and return the algebraic type, so code using them compiles unchanged with either type:
//!
//! ```rust
//! // type Real = f64;
//! type Real = algebraic::af64;
//!
//! fn largest_magnitude(values: &[Real]) -> Real {
//!     values
//!         .iter()
//!         .fold(Real::from(0.0), |acc, &x| acc.max(x.abs()))
//! }
//!
//! let values = [3.0, -4.0].map(Real::from);
//! assert_eq!(largest_magnitude(&values), Real::from(4.0));
//! ```
//!
//! Since this crate is `no_std`, methods that need `std`, such as `sqrt`, `floor`, `mul_add` and
//! `sin`, are not available as inherent methods. Enable the **`num`** feature and import
//! [`num_traits::Float`](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html)
//! to call them on both the primitive and the algebraic types:
//!
//! ```rust
//! # #[cfg(feature = "num")]
//! # {
//! use num_traits::Float;
//!
//! // type Real = f64;
//! type Real = algebraic::af64;
//!
//! fn norm(values: &[Real]) -> Real {
//!     values.iter().map(|&x| x * x).sum::<Real>().sqrt()
//! }
//!
//! let values = [3.0, -4.0].map(Real::from);
//! assert_eq!(norm(&values), Real::from(5.0));
//! # }
//! ```
//!
//! The inherent methods forward to the primitive methods and are therefore not algebraic, except
//! `recip`, which uses algebraic division.

// Core modules
mod algebraic;
mod conversions;
mod impls;
mod methods;
#[cfg(feature = "num")]
mod num_impl;
mod ops;
#[cfg(feature = "serde")]
mod serde_impl;
mod traits;
mod types;

// Test module
#[cfg(test)]
mod tests;

// Public exports
pub use algebraic::Algebraic;
pub use traits::AlgebraicFloatTrait;
#[cfg(feature = "f16")]
pub use types::af16;
#[cfg(feature = "f128")]
pub use types::af128;
pub use types::{af32, af64};
