//! Convenient type aliases for commonly used Algebraic floating-point types.

use crate::algebraic::Algebraic;

/// This type represents a half-precision float wrapping `f16` with algebraic optimization enabled.
/// This type alias requires the **`f16`** crate feature to be enabled and requires a nightly
/// toolchain supporting the unstable `#![feature(f16)]` language feature.
#[cfg(feature = "f16")]
#[allow(non_camel_case_types)]
pub type af16 = Algebraic<f16>;

/// This type represents a single-precision float wrapping `f32` with algebraic optimization enabled.
#[allow(non_camel_case_types)]
pub type af32 = Algebraic<f32>;

/// This type represents a double-precision float wrapping `f64` with algebraic optimization enabled.
#[allow(non_camel_case_types)]
pub type af64 = Algebraic<f64>;

/// This type represents an quadruple-precision float wrapping `f128` with algebraic optimization enabled.
/// This type alias requires the **`f128`** crate feature to be enabled and requires a nightly
/// toolchain supporting the unstable `#![feature(f128)]` language feature.
#[cfg(feature = "f128")]
#[allow(non_camel_case_types)]
pub type af128 = Algebraic<f128>;
