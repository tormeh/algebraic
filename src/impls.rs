//! Per-type implementations for the primitive floating-point types: [`AlgebraicFloatTrait`],
//! `From` conversions in both directions, and the reversed mixed-type operators
//! (`T op Algebraic<T>`).
//!
//! The reversed operators cannot be a blanket impl: Rust's orphan rules forbid
//! `impl<T: AlgebraicFloatTrait> Add<Algebraic<T>> for T`, since `T` would be an uncovered type
//! parameter used as `Self` for a foreign trait. Each concrete type therefore gets its own impls.

use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

macro_rules! impl_reverse_op {
    ($t:ident, $Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident, $alg:ident) => {
        impl $Op<Algebraic<Self>> for $t {
            type Output = Algebraic<Self>;

            #[inline]
            fn $op(self, rhs: Algebraic<Self>) -> Self::Output {
                Algebraic::new(self.$alg(rhs.value))
            }
        }

        impl $OpAssign<Algebraic<Self>> for $t {
            #[inline]
            fn $op_assign(&mut self, rhs: Algebraic<Self>) {
                *self = self.$alg(rhs.value);
            }
        }
    };
}

macro_rules! impl_float {
    ($t:ident) => {
        impl AlgebraicFloatTrait for $t {
            #[inline]
            fn algebraic_add(self, rhs: Self) -> Self {
                Self::algebraic_add(self, rhs)
            }

            #[inline]
            fn algebraic_sub(self, rhs: Self) -> Self {
                Self::algebraic_sub(self, rhs)
            }

            #[inline]
            fn algebraic_mul(self, rhs: Self) -> Self {
                Self::algebraic_mul(self, rhs)
            }

            #[inline]
            fn algebraic_div(self, rhs: Self) -> Self {
                Self::algebraic_div(self, rhs)
            }

            #[inline]
            fn algebraic_rem(self, rhs: Self) -> Self {
                Self::algebraic_rem(self, rhs)
            }

            #[inline]
            fn zero() -> Self {
                0.0
            }

            #[inline]
            fn one() -> Self {
                1.0
            }
        }

        impl From<$t> for Algebraic<$t> {
            #[inline]
            fn from(value: $t) -> Self {
                Self::new(value)
            }
        }

        impl From<Algebraic<Self>> for $t {
            #[inline]
            fn from(val: Algebraic<Self>) -> Self {
                val.value
            }
        }

        impl_reverse_op!($t, Add, add, AddAssign, add_assign, algebraic_add);
        impl_reverse_op!($t, Sub, sub, SubAssign, sub_assign, algebraic_sub);
        impl_reverse_op!($t, Mul, mul, MulAssign, mul_assign, algebraic_mul);
        impl_reverse_op!($t, Div, div, DivAssign, div_assign, algebraic_div);
        impl_reverse_op!($t, Rem, rem, RemAssign, rem_assign, algebraic_rem);
    };
}

#[cfg(feature = "f16")]
impl_float!(f16);
impl_float!(f32);
impl_float!(f64);
#[cfg(feature = "f128")]
impl_float!(f128);
