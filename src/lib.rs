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
//!
//! To use the unstable types, add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! algebraic = { version = "*", features = ["f16", "f128"] }
//! ```

// Core modules
mod algebraic;
mod conversions;
mod impls;
mod ops;
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
