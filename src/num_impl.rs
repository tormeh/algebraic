use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use core::num::FpCategory;
use core::ops::Neg;
use num_traits::float::FloatCore;
use num_traits::{
    ConstOne, ConstZero, Float, FloatConst, FromPrimitive, Inv, Num, NumCast, One, Signed,
    ToPrimitive, Zero,
};

impl<T> Zero for Algebraic<T>
where
    T: AlgebraicFloatTrait + Zero,
{
    #[inline]
    fn zero() -> Self {
        Self::new(<T as Zero>::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<T> ConstZero for Algebraic<T>
where
    T: AlgebraicFloatTrait + ConstZero,
{
    const ZERO: Self = Self::new(T::ZERO);
}

impl<T> One for Algebraic<T>
where
    T: AlgebraicFloatTrait + One,
{
    #[inline]
    fn one() -> Self {
        Self::new(<T as One>::one())
    }
}

impl<T> ConstOne for Algebraic<T>
where
    T: AlgebraicFloatTrait + ConstOne,
{
    const ONE: Self = Self::new(T::ONE);
}

impl<T> Num for Algebraic<T>
where
    T: AlgebraicFloatTrait + Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    #[inline]
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(src, radix).map(Self::new)
    }
}

impl<T> Signed for Algebraic<T>
where
    T: AlgebraicFloatTrait + Signed + Neg<Output = T>,
{
    #[inline]
    fn abs(&self) -> Self {
        Self::new(self.value.abs())
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        Self::new(self.value.abs_sub(&other.value))
    }

    #[inline]
    fn signum(&self) -> Self {
        Self::new(self.value.signum())
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.value.is_negative()
    }
}

macro_rules! forward_to_primitive {
    ($($method:ident -> $primitive:ty),+ $(,)?) => {
        $(
            #[inline]
            fn $method(&self) -> Option<$primitive> {
                <T as ToPrimitive>::$method(&self.value)
            }
        )+
    };
}

impl<T> ToPrimitive for Algebraic<T>
where
    T: AlgebraicFloatTrait + ToPrimitive,
{
    forward_to_primitive! {
        to_isize -> isize,
        to_i8 -> i8,
        to_i16 -> i16,
        to_i32 -> i32,
        to_i64 -> i64,
        to_i128 -> i128,
        to_usize -> usize,
        to_u8 -> u8,
        to_u16 -> u16,
        to_u32 -> u32,
        to_u64 -> u64,
        to_u128 -> u128,
        to_f32 -> f32,
        to_f64 -> f64,
    }
}

macro_rules! forward_from_primitive {
    ($($method:ident($primitive:ty)),+ $(,)?) => {
        $(
            #[inline]
            fn $method(n: $primitive) -> Option<Self> {
                <T as FromPrimitive>::$method(n).map(Self::new)
            }
        )+
    };
}

impl<T> FromPrimitive for Algebraic<T>
where
    T: AlgebraicFloatTrait + FromPrimitive,
{
    forward_from_primitive! {
        from_isize(isize),
        from_i8(i8),
        from_i16(i16),
        from_i32(i32),
        from_i64(i64),
        from_i128(i128),
        from_usize(usize),
        from_u8(u8),
        from_u16(u16),
        from_u32(u32),
        from_u64(u64),
        from_u128(u128),
        from_f32(f32),
        from_f64(f64),
    }
}

impl<T> NumCast for Algebraic<T>
where
    T: AlgebraicFloatTrait + NumCast,
{
    #[inline]
    fn from<N: ToPrimitive>(n: N) -> Option<Self> {
        T::from(n).map(Self::new)
    }
}

macro_rules! forward_float_static {
    ($($method:ident),+ $(,)?) => {
        $(
            #[inline]
            fn $method() -> Self {
                Self::new(T::$method())
            }
        )+
    };
}

macro_rules! forward_float_unary {
    ($($method:ident),+ $(,)?) => {
        $(
            #[inline]
            fn $method(self) -> Self {
                Self::new(self.value.$method())
            }
        )+
    };
}

macro_rules! forward_float_predicates {
    ($($method:ident),+ $(,)?) => {
        $(
            #[inline]
            fn $method(self) -> bool {
                self.value.$method()
            }
        )+
    };
}

macro_rules! forward_float_binary {
    ($($method:ident),+ $(,)?) => {
        $(
            #[inline]
            fn $method(self, other: Self) -> Self {
                Self::new(self.value.$method(other.value))
            }
        )+
    };
}

impl<T> FloatConst for Algebraic<T>
where
    T: AlgebraicFloatTrait + FloatConst,
{
    forward_float_static! {
        E,
        FRAC_1_PI,
        FRAC_1_SQRT_2,
        FRAC_2_PI,
        FRAC_2_SQRT_PI,
        FRAC_PI_2,
        FRAC_PI_3,
        FRAC_PI_4,
        FRAC_PI_6,
        FRAC_PI_8,
        LN_10,
        LN_2,
        LOG10_E,
        LOG2_E,
        PI,
        SQRT_2,
    }
}

impl<T> FloatCore for Algebraic<T>
where
    T: AlgebraicFloatTrait + FloatCore,
{
    forward_float_static! {
        nan,
        infinity,
        neg_infinity,
        neg_zero,
        min_value,
        min_positive_value,
        epsilon,
        max_value,
    }

    forward_float_predicates! {
        is_nan,
        is_infinite,
        is_finite,
        is_normal,
        is_subnormal,
        is_sign_positive,
        is_sign_negative,
    }

    #[inline]
    fn classify(self) -> FpCategory {
        self.value.classify()
    }

    forward_float_unary! {
        floor,
        ceil,
        round,
        trunc,
        fract,
        abs,
        signum,
        to_degrees,
        to_radians,
    }

    #[inline]
    fn recip(self) -> Self {
        Self::one() / self
    }

    #[inline]
    fn powi(self, exp: i32) -> Self {
        Self::new(self.value.powi(exp))
    }

    forward_float_binary! {
        max,
        min,
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.value.clamp(min.value, max.value))
    }

    #[inline]
    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }
}

impl<T> Float for Algebraic<T>
where
    T: AlgebraicFloatTrait + Float,
{
    forward_float_static! {
        nan,
        infinity,
        neg_infinity,
        neg_zero,
        min_value,
        min_positive_value,
        max_value,
    }

    forward_float_predicates! {
        is_nan,
        is_infinite,
        is_finite,
        is_normal,
        is_sign_positive,
        is_sign_negative,
    }

    #[inline]
    fn classify(self) -> FpCategory {
        self.value.classify()
    }

    forward_float_unary! {
        floor,
        ceil,
        round,
        trunc,
        fract,
        abs,
        signum,
        sqrt,
        exp,
        exp2,
        ln,
        log2,
        log10,
        cbrt,
        sin,
        cos,
        tan,
        asin,
        acos,
        atan,
        exp_m1,
        ln_1p,
        sinh,
        cosh,
        tanh,
        asinh,
        acosh,
        atanh,
    }

    #[inline]
    fn recip(self) -> Self {
        Self::one() / self
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::new(self.value.mul_add(a.value, b.value))
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        Self::new(self.value.powi(n))
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        Self::new(self.value.log(base.value))
    }

    forward_float_binary! {
        powf,
        max,
        min,
        abs_sub,
        hypot,
        atan2,
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        (Self::new(sin), Self::new(cos))
    }

    #[inline]
    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }
}

impl<T> Inv for Algebraic<T>
where
    T: AlgebraicFloatTrait,
{
    type Output = Self;

    #[inline]
    fn inv(self) -> Self::Output {
        Self::one() / self
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::types::{af32, af64};

    #[test]
    fn test_float_core_f32() {
        let x = af32::new(4.0);
        let y = af32::new(2.0);
        assert_eq!(FloatCore::floor(af32::new(4.7)), af32::new(4.0));
        assert_eq!(FloatCore::ceil(af32::new(4.2)), af32::new(5.0));
        assert_eq!(FloatCore::min(x, y), af32::new(2.0));
        assert_eq!(FloatCore::max(x, y), af32::new(4.0));
        assert_eq!(
            FloatCore::clamp(x, af32::new(1.0), af32::new(3.0)),
            af32::new(3.0)
        );
        assert_eq!(FloatCore::recip(y), af32::new(0.5));
        assert_eq!(FloatCore::powi(y, 3), af32::new(8.0));
        assert!(FloatCore::is_finite(x));
        assert!(!FloatCore::is_infinite(x));
        assert!(!FloatCore::is_nan(x));
        assert!(FloatCore::is_sign_positive(x));
        assert!(!FloatCore::is_sign_negative(x));
    }

    #[test]
    fn test_float_core_f64() {
        let x = af64::new(-4.0);
        assert_eq!(FloatCore::abs(x), af64::new(4.0));
        assert_eq!(FloatCore::signum(x), af64::new(-1.0));
        assert!(FloatCore::is_sign_negative(x));
        assert!(!FloatCore::is_sign_positive(x));
        assert!(FloatCore::is_nan(<af64 as FloatCore>::nan()));
        assert!(FloatCore::is_infinite(<af64 as FloatCore>::infinity()));
    }

    #[test]
    fn test_float_traits() {
        let b = af32::new(2.0);
        assert_eq!(Float::sqrt(af32::new(9.0)), af32::new(3.0));
        assert_eq!(Float::recip(b), af32::new(0.5));
        assert_eq!(Inv::inv(b), af32::new(0.5));
        let zero: af32 = Zero::zero();
        assert_eq!(zero, af32::new(0.0));
        let one: af32 = One::one();
        assert_eq!(one, af32::new(1.0));
        let pi: af32 = FloatConst::PI();
        assert_eq!(pi, af32::new(core::f32::consts::PI));
    }
}
