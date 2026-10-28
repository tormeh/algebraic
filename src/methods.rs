//! Inherent constants and methods for the concrete [`Algebraic<T>`] types.
//!
//! These mirror the constants and methods of the primitive float types that are available in
//! `core`, so that e.g. `af32::abs` works like `f32::abs` without importing any traits.
//! Methods that require `std` (such as `sqrt` or `floor`) are not provided here; with the `num`
//! feature they are available through `num_traits::Float`.

use crate::algebraic::Algebraic;
use core::cmp::Ordering;
use core::num::FpCategory;

macro_rules! impl_float_methods {
    ($t:ident, $bits:ty) => {
        impl Algebraic<$t> {
            /// Not a Number (NaN).
            pub const NAN: Self = Self::new($t::NAN);
            /// Infinity (∞).
            pub const INFINITY: Self = Self::new($t::INFINITY);
            /// Negative infinity (−∞).
            pub const NEG_INFINITY: Self = Self::new($t::NEG_INFINITY);
            /// Smallest finite value.
            pub const MIN: Self = Self::new($t::MIN);
            /// Largest finite value.
            pub const MAX: Self = Self::new($t::MAX);
            /// Smallest positive normal value.
            pub const MIN_POSITIVE: Self = Self::new($t::MIN_POSITIVE);
            /// Machine epsilon: the difference between `1.0` and the next larger representable number.
            pub const EPSILON: Self = Self::new($t::EPSILON);

            /// Returns `true` if this value is NaN.
            #[must_use]
            #[inline]
            pub const fn is_nan(self) -> bool {
                self.value.is_nan()
            }

            /// Returns `true` if this value is positive or negative infinity.
            #[must_use]
            #[inline]
            pub const fn is_infinite(self) -> bool {
                self.value.is_infinite()
            }

            /// Returns `true` if this value is neither infinite nor NaN.
            #[must_use]
            #[inline]
            pub const fn is_finite(self) -> bool {
                self.value.is_finite()
            }

            /// Returns `true` if this value is subnormal.
            #[must_use]
            #[inline]
            pub const fn is_subnormal(self) -> bool {
                self.value.is_subnormal()
            }

            /// Returns `true` if this value is neither zero, infinite, subnormal nor NaN.
            #[must_use]
            #[inline]
            pub const fn is_normal(self) -> bool {
                self.value.is_normal()
            }

            /// Returns the floating-point category of this value.
            #[must_use]
            #[inline]
            pub const fn classify(self) -> FpCategory {
                self.value.classify()
            }

            /// Returns `true` if `self` has a positive sign, including `+0.0`, positive NaNs
            /// and positive infinity.
            #[must_use]
            #[inline]
            pub const fn is_sign_positive(self) -> bool {
                self.value.is_sign_positive()
            }

            /// Returns `true` if `self` has a negative sign, including `-0.0`, negative NaNs
            /// and negative infinity.
            #[must_use]
            #[inline]
            pub const fn is_sign_negative(self) -> bool {
                self.value.is_sign_negative()
            }

            /// Computes the absolute value of `self`.
            #[must_use]
            #[inline]
            pub const fn abs(self) -> Self {
                Self::new(self.value.abs())
            }

            /// Returns a number representing the sign of `self`: `1.0` if positive (including
            /// `+0.0` and infinity), `-1.0` if negative (including `-0.0` and negative infinity),
            /// and NaN if `self` is NaN.
            #[must_use]
            #[inline]
            pub const fn signum(self) -> Self {
                Self::new(self.value.signum())
            }

            /// Returns a value with the magnitude of `self` and the sign of `sign`.
            #[must_use]
            #[inline]
            pub const fn copysign(self, sign: Self) -> Self {
                Self::new(self.value.copysign(sign.value))
            }

            /// Takes the reciprocal (inverse) of a number, `1/x`, using algebraic division.
            #[must_use]
            #[inline]
            pub fn recip(self) -> Self {
                Self::new(1.0) / self
            }

            /// Converts radians to degrees.
            #[must_use]
            #[inline]
            pub const fn to_degrees(self) -> Self {
                Self::new(self.value.to_degrees())
            }

            /// Converts degrees to radians.
            #[must_use]
            #[inline]
            pub const fn to_radians(self) -> Self {
                Self::new(self.value.to_radians())
            }

            /// Returns the maximum of the two numbers, ignoring NaN.
            #[must_use]
            #[inline]
            pub const fn max(self, other: Self) -> Self {
                Self::new(self.value.max(other.value))
            }

            /// Returns the minimum of the two numbers, ignoring NaN.
            #[must_use]
            #[inline]
            pub const fn min(self, other: Self) -> Self {
                Self::new(self.value.min(other.value))
            }

            /// Restricts a value to a certain interval unless it is NaN.
            ///
            /// # Panics
            ///
            /// Panics if `min > max`, `min` is NaN, or `max` is NaN.
            #[must_use]
            #[inline]
            pub const fn clamp(self, min: Self, max: Self) -> Self {
                Self::new(self.value.clamp(min.value, max.value))
            }

            /// Calculates the midpoint (average) between `self` and `other`.
            #[must_use]
            #[inline]
            pub const fn midpoint(self, other: Self) -> Self {
                Self::new(self.value.midpoint(other.value))
            }

            /// Returns the least number greater than `self`.
            #[must_use]
            #[inline]
            pub const fn next_up(self) -> Self {
                Self::new(self.value.next_up())
            }

            /// Returns the greatest number less than `self`.
            #[must_use]
            #[inline]
            pub const fn next_down(self) -> Self {
                Self::new(self.value.next_down())
            }

            /// Returns the ordering between `self` and `other` according to the IEEE 754
            /// `totalOrder` predicate.
            #[must_use]
            #[inline]
            pub fn total_cmp(&self, other: &Self) -> Ordering {
                self.value.total_cmp(&other.value)
            }

            /// Raw transmutation to the underlying bits.
            #[must_use]
            #[inline]
            pub const fn to_bits(self) -> $bits {
                self.value.to_bits()
            }

            /// Raw transmutation from bits.
            #[must_use]
            #[inline]
            pub const fn from_bits(v: $bits) -> Self {
                Self::new($t::from_bits(v))
            }
        }
    };
}

#[cfg(feature = "f16")]
impl_float_methods!(f16, u16);
impl_float_methods!(f32, u32);
impl_float_methods!(f64, u64);
#[cfg(feature = "f128")]
impl_float_methods!(f128, u128);

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::types::{af32, af64};

    #[test]
    fn test_constants() {
        assert!(af32::NAN.is_nan());
        assert_eq!(af32::INFINITY.value(), f32::INFINITY);
        assert_eq!(af32::NEG_INFINITY.value(), f32::NEG_INFINITY);
        assert_eq!(af64::MIN.value(), f64::MIN);
        assert_eq!(af64::MAX.value(), f64::MAX);
        assert_eq!(af64::MIN_POSITIVE.value(), f64::MIN_POSITIVE);
        assert_eq!(af64::EPSILON.value(), f64::EPSILON);
    }

    #[test]
    fn test_predicates() {
        let x = af32::new(-1.5);
        assert!(!x.is_nan());
        assert!(!x.is_infinite());
        assert!(x.is_finite());
        assert!(x.is_normal());
        assert!(!x.is_subnormal());
        assert!(x.is_sign_negative());
        assert!(!x.is_sign_positive());
        assert_eq!(x.classify(), FpCategory::Normal);
        assert!(af64::INFINITY.is_infinite());
    }

    #[test]
    fn test_sign_methods() {
        assert_eq!(af32::new(-2.0).abs(), af32::new(2.0));
        assert_eq!(af64::new(3.0).abs(), af64::new(3.0));
        assert_eq!(af32::new(-2.0).signum(), af32::new(-1.0));
        assert_eq!(af64::new(2.0).copysign(af64::new(-0.0)), af64::new(-2.0));
    }

    #[test]
    fn test_arithmetic_methods() {
        assert_eq!(af32::new(4.0).recip(), af32::new(0.5));
        assert_eq!(af64::new(90.0).to_radians().value(), 90.0_f64.to_radians());
        assert_eq!(af64::new(1.0).to_degrees().value(), 1.0_f64.to_degrees());
        assert_eq!(af32::new(1.0).midpoint(af32::new(3.0)), af32::new(2.0));
        assert!(af32::new(1.0).next_up() > af32::new(1.0));
        assert!(af32::new(1.0).next_down() < af32::new(1.0));
    }

    #[test]
    fn test_comparison_methods() {
        let a = af32::new(1.0);
        let b = af32::new(2.0);
        assert_eq!(a.max(b), b);
        assert_eq!(a.min(b), a);
        assert_eq!(a.max(af32::NAN), a);
        assert_eq!(af32::new(5.0).clamp(a, b), b);
        assert_eq!(a.total_cmp(&b), Ordering::Less);
        assert_eq!(af64::new(-0.0).total_cmp(&af64::new(0.0)), Ordering::Less);
    }

    #[test]
    fn test_bits() {
        let x = af32::new(1.5);
        assert_eq!(x.to_bits(), 1.5_f32.to_bits());
        assert_eq!(af32::from_bits(x.to_bits()), x);
        assert_eq!(af64::from_bits(2.0_f64.to_bits()), af64::new(2.0));
    }
}
