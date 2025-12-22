#![feature(f16)]
#![feature(f128)]
#![feature(float_algebraic)]

//! Algebraic floating-point operations library
//!
//! This library provides a wrapper type `Algebraic<T>` for floating-point types
//! that supports algebraic operations using Rust's experimental algebraic float methods.
//!
//! # Features
//!
//! - Support for f16, f32, f64, and f128 floating-point types
//! - Algebraic operations (addition, subtraction, multiplication, division, remainder)
//! - Iterator support (Sum, Product)
//! - Convenient type aliases (af16, af32, af64, af128)
//!
//! # Example
//!
//! ```
//! use algebraic::{Algebraic, af64};
//!
//! let a: af64 = Algebraic::from(2.0);
//! let b: af64 = Algebraic::from(3.0);
//! let result = a + b;
//! assert_eq!(5.0, result.into());
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
pub use types::{af16, af32, af64, af128};
