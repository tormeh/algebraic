//! Type aliases for commonly used Algebraic floating-point types

use crate::algebraic::Algebraic;

/// 16-bit algebraic floating-point type
#[allow(non_camel_case_types)]
pub type af16 = Algebraic<f16>;

/// 32-bit algebraic floating-point type
#[allow(non_camel_case_types)]
pub type af32 = Algebraic<f32>;

/// 64-bit algebraic floating-point type
#[allow(non_camel_case_types)]
pub type af64 = Algebraic<f64>;

/// 128-bit algebraic floating-point type
#[allow(non_camel_case_types)]
pub type af128 = Algebraic<f128>;
