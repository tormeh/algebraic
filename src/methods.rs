//! Inherent constants and methods for the concrete [`Algebraic<T>`] types.
//!
//! These mirror the stable constants and methods of the primitive float types that are available
//! in `core`. See the crate-level documentation for how to use the `std`-only methods.

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
            /// The radix or base of the internal representation.
            pub const RADIX: u32 = $t::RADIX;
            /// Number of significant digits in base 2.
            pub const MANTISSA_DIGITS: u32 = $t::MANTISSA_DIGITS;
            /// Approximate number of significant digits in base 10.
            pub const DIGITS: u32 = $t::DIGITS;
            /// One greater than the minimum possible normal power of 2 exponent.
            pub const MIN_EXP: i32 = $t::MIN_EXP;
            /// Maximum possible power of 2 exponent.
            pub const MAX_EXP: i32 = $t::MAX_EXP;
            /// Minimum `x` for which `10^x` is a normal number.
            pub const MIN_10_EXP: i32 = $t::MIN_10_EXP;
            /// Maximum `x` for which `10^x` is a normal number.
            pub const MAX_10_EXP: i32 = $t::MAX_10_EXP;

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

            /// Returns the memory representation of this number as a byte array in big-endian
            /// byte order.
            #[must_use]
            #[inline]
            pub const fn to_be_bytes(self) -> [u8; size_of::<$t>()] {
                self.value.to_be_bytes()
            }

            /// Returns the memory representation of this number as a byte array in little-endian
            /// byte order.
            #[must_use]
            #[inline]
            pub const fn to_le_bytes(self) -> [u8; size_of::<$t>()] {
                self.value.to_le_bytes()
            }

            /// Returns the memory representation of this number as a byte array in native byte
            /// order.
            #[must_use]
            #[inline]
            pub const fn to_ne_bytes(self) -> [u8; size_of::<$t>()] {
                self.value.to_ne_bytes()
            }

            /// Creates a value from its representation as a byte array in big-endian byte order.
            #[must_use]
            #[inline]
            pub const fn from_be_bytes(bytes: [u8; size_of::<$t>()]) -> Self {
                Self::new($t::from_be_bytes(bytes))
            }

            /// Creates a value from its representation as a byte array in little-endian byte
            /// order.
            #[must_use]
            #[inline]
            pub const fn from_le_bytes(bytes: [u8; size_of::<$t>()]) -> Self {
                Self::new($t::from_le_bytes(bytes))
            }

            /// Creates a value from its representation as a byte array in native byte order.
            #[must_use]
            #[inline]
            pub const fn from_ne_bytes(bytes: [u8; size_of::<$t>()]) -> Self {
                Self::new($t::from_ne_bytes(bytes))
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
        assert_eq!(af32::new(4.0).recip(), af32::new(0.25));
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

    /// Checks that every constant and method gives the same result as on the primitive type,
    /// comparing bit patterns so that NaN and signed zeros are compared exactly.
    macro_rules! test_forwarding {
        ($name:ident, $t:ident) => {
            #[test]
            fn $name() {
                type A = Algebraic<$t>;

                assert_eq!(A::NAN.to_bits(), $t::NAN.to_bits());
                assert_eq!(A::INFINITY.to_bits(), $t::INFINITY.to_bits());
                assert_eq!(A::NEG_INFINITY.to_bits(), $t::NEG_INFINITY.to_bits());
                assert_eq!(A::MIN.to_bits(), $t::MIN.to_bits());
                assert_eq!(A::MAX.to_bits(), $t::MAX.to_bits());
                assert_eq!(A::MIN_POSITIVE.to_bits(), $t::MIN_POSITIVE.to_bits());
                assert_eq!(A::EPSILON.to_bits(), $t::EPSILON.to_bits());
                assert_eq!(A::RADIX, $t::RADIX);
                assert_eq!(A::MANTISSA_DIGITS, $t::MANTISSA_DIGITS);
                assert_eq!(A::DIGITS, $t::DIGITS);
                assert_eq!(A::MIN_EXP, $t::MIN_EXP);
                assert_eq!(A::MAX_EXP, $t::MAX_EXP);
                assert_eq!(A::MIN_10_EXP, $t::MIN_10_EXP);
                assert_eq!(A::MAX_10_EXP, $t::MAX_10_EXP);

                let values: [$t; 8] = [
                    -1.5,
                    2.0,
                    0.0,
                    -0.0,
                    $t::NAN,
                    $t::INFINITY,
                    $t::NEG_INFINITY,
                    $t::MIN_POSITIVE / 2.0,
                ];
                for x in values {
                    let a = A::new(x);
                    assert_eq!(a.is_nan(), x.is_nan());
                    assert_eq!(a.is_infinite(), x.is_infinite());
                    assert_eq!(a.is_finite(), x.is_finite());
                    assert_eq!(a.is_subnormal(), x.is_subnormal());
                    assert_eq!(a.is_normal(), x.is_normal());
                    assert_eq!(a.classify(), x.classify());
                    assert_eq!(a.is_sign_positive(), x.is_sign_positive());
                    assert_eq!(a.is_sign_negative(), x.is_sign_negative());
                    assert_eq!(a.abs().to_bits(), x.abs().to_bits());
                    assert_eq!(a.signum().to_bits(), x.signum().to_bits());
                    assert_eq!(a.to_degrees().to_bits(), x.to_degrees().to_bits());
                    assert_eq!(a.to_radians().to_bits(), x.to_radians().to_bits());
                    assert_eq!(
                        a.clamp(A::new(-1.0), A::new(1.0)).to_bits(),
                        x.clamp(-1.0, 1.0).to_bits()
                    );
                    assert_eq!(a.next_up().to_bits(), x.next_up().to_bits());
                    assert_eq!(a.next_down().to_bits(), x.next_down().to_bits());
                    assert_eq!(a.to_bits(), x.to_bits());
                    assert_eq!(A::from_bits(x.to_bits()).to_bits(), x.to_bits());
                    assert_eq!(a.to_be_bytes(), x.to_be_bytes());
                    assert_eq!(a.to_le_bytes(), x.to_le_bytes());
                    assert_eq!(a.to_ne_bytes(), x.to_ne_bytes());
                    assert_eq!(A::from_be_bytes(x.to_be_bytes()).to_bits(), x.to_bits());
                    assert_eq!(A::from_le_bytes(x.to_le_bytes()).to_bits(), x.to_bits());
                    assert_eq!(A::from_ne_bytes(x.to_ne_bytes()).to_bits(), x.to_bits());

                    for y in values {
                        let b = A::new(y);
                        assert_eq!(a.copysign(b).to_bits(), x.copysign(y).to_bits());
                        assert_eq!(a.max(b).to_bits(), x.max(y).to_bits());
                        assert_eq!(a.min(b).to_bits(), x.min(y).to_bits());
                        assert_eq!(a.midpoint(b).to_bits(), x.midpoint(y).to_bits());
                        assert_eq!(a.total_cmp(&b), x.total_cmp(&y));
                    }
                }

                // `recip` uses algebraic division, so only check a case that is exact.
                let quarter: $t = 0.25;
                assert_eq!(A::new(4.0).recip().to_bits(), quarter.to_bits());
            }
        };
    }

    #[cfg(feature = "f16")]
    test_forwarding!(test_forwarding_f16, f16);
    test_forwarding!(test_forwarding_f32, f32);
    test_forwarding!(test_forwarding_f64, f64);
    #[cfg(feature = "f128")]
    test_forwarding!(test_forwarding_f128, f128);
}
