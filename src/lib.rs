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
//! // Construct algebraic floats using `new` or `From::from`
//! let a = af32::new(1.5);
//! let b = af32::from(2.0);
//!
//! // Perform standard arithmetic operations which compile down to algebraic intrinsics,
//! // allowing the compiler to aggressively reorder and vectorize.
//! let mut c = a * b + af32::new(0.5);
//! assert_eq!(c.value(), 3.5);
//!
//! c += af32::new(1.0);
//! assert_eq!(f32::from(c), 4.5);
//!
//! // Works seamlessly with iterators for operations like sum and product
//! let values = [1.0, 2.0, 3.0, 4.0].map(af32::new);
//! let sum: af32 = values.into_iter().sum();
//! assert_eq!(sum.value(), 10.0);
//! ```

// Core modules
mod algebraic;
mod conversions;
mod impls;
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
