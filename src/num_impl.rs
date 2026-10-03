use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use core::num::FpCategory;
use core::ops::Neg;
use num_traits::{
    ConstOne, ConstZero, Float, FloatConst, FromPrimitive, Inv, Num, NumCast, One, Signed,
    ToPrimitive, Zero,
};

impl<T: AlgebraicFloatTrait + Neg<Output = T>> Neg for Algebraic<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

impl<T> Zero for Algebraic<T>
where
    T: AlgebraicFloatTrait + Zero,
{
    fn zero() -> Self {
        Self::new(<T as Zero>::zero())
    }

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

    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(src, radix).map(Self::new)
    }
}

impl<T> Signed for Algebraic<T>
where
    T: AlgebraicFloatTrait + Signed + Neg<Output = T>,
{
    fn abs(&self) -> Self {
        Self::new(self.value.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self::new(self.value.abs_sub(&other.value))
    }

    fn signum(&self) -> Self {
        Self::new(self.value.signum())
    }

    fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.value.is_negative()
    }
}

macro_rules! forward_to_primitive {
    ($($method:ident -> $primitive:ty),+ $(,)?) => {
        $(
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
    fn from<N: ToPrimitive>(n: N) -> Option<Self> {
        T::from(n).map(Self::new)
    }
}

impl<T> FloatConst for Algebraic<T>
where
    T: AlgebraicFloatTrait + FloatConst,
{
    fn E() -> Self {
        Self::new(T::E())
    }

    fn FRAC_1_PI() -> Self {
        Self::new(T::FRAC_1_PI())
    }

    fn FRAC_1_SQRT_2() -> Self {
        Self::new(T::FRAC_1_SQRT_2())
    }

    fn FRAC_2_PI() -> Self {
        Self::new(T::FRAC_2_PI())
    }

    fn FRAC_2_SQRT_PI() -> Self {
        Self::new(T::FRAC_2_SQRT_PI())
    }

    fn FRAC_PI_2() -> Self {
        Self::new(T::FRAC_PI_2())
    }

    fn FRAC_PI_3() -> Self {
        Self::new(T::FRAC_PI_3())
    }

    fn FRAC_PI_4() -> Self {
        Self::new(T::FRAC_PI_4())
    }

    fn FRAC_PI_6() -> Self {
        Self::new(T::FRAC_PI_6())
    }

    fn FRAC_PI_8() -> Self {
        Self::new(T::FRAC_PI_8())
    }

    fn LN_10() -> Self {
        Self::new(T::LN_10())
    }

    fn LN_2() -> Self {
        Self::new(T::LN_2())
    }

    fn LOG10_E() -> Self {
        Self::new(T::LOG10_E())
    }

    fn LOG2_E() -> Self {
        Self::new(T::LOG2_E())
    }

    fn PI() -> Self {
        Self::new(T::PI())
    }

    fn SQRT_2() -> Self {
        Self::new(T::SQRT_2())
    }
}

impl<T> Float for Algebraic<T>
where
    T: AlgebraicFloatTrait + Float + FloatConst,
{
    fn nan() -> Self {
        Self::new(T::nan())
    }

    fn infinity() -> Self {
        Self::new(T::infinity())
    }

    fn neg_infinity() -> Self {
        Self::new(T::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self::new(T::neg_zero())
    }

    fn min_value() -> Self {
        Self::new(T::min_value())
    }

    fn min_positive_value() -> Self {
        Self::new(T::min_positive_value())
    }

    fn max_value() -> Self {
        Self::new(T::max_value())
    }

    fn is_nan(self) -> bool {
        self.value.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.value.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    fn is_normal(self) -> bool {
        self.value.is_normal()
    }

    fn classify(self) -> FpCategory {
        self.value.classify()
    }

    fn floor(self) -> Self {
        Self::new(self.value.floor())
    }

    fn ceil(self) -> Self {
        Self::new(self.value.ceil())
    }

    fn round(self) -> Self {
        Self::new(self.value.round())
    }

    fn trunc(self) -> Self {
        Self::new(self.value.trunc())
    }

    fn fract(self) -> Self {
        Self::new(self.value.fract())
    }

    fn abs(self) -> Self {
        Self::new(self.value.abs())
    }

    fn signum(self) -> Self {
        Self::new(self.value.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::new(self.value.mul_add(a.value, b.value))
    }

    fn recip(self) -> Self {
        Self::new(self.value.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self::new(self.value.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self::new(self.value.powf(n.value))
    }

    fn sqrt(self) -> Self {
        Self::new(self.value.sqrt())
    }

    fn exp(self) -> Self {
        Self::new(self.value.exp())
    }

    fn exp2(self) -> Self {
        Self::new(self.value.exp2())
    }

    fn ln(self) -> Self {
        Self::new(self.value.ln())
    }

    fn log(self, base: Self) -> Self {
        Self::new(self.value.log(base.value))
    }

    fn log2(self) -> Self {
        Self::new(self.value.log2())
    }

    fn log10(self) -> Self {
        Self::new(self.value.log10())
    }

    fn max(self, other: Self) -> Self {
        Self::new(self.value.max(other.value))
    }

    fn min(self, other: Self) -> Self {
        Self::new(self.value.min(other.value))
    }

    fn abs_sub(self, other: Self) -> Self {
        Self::new(self.value.abs_sub(other.value))
    }

    fn cbrt(self) -> Self {
        Self::new(self.value.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self::new(self.value.hypot(other.value))
    }

    fn sin(self) -> Self {
        Self::new(self.value.sin())
    }

    fn cos(self) -> Self {
        Self::new(self.value.cos())
    }

    fn tan(self) -> Self {
        Self::new(self.value.tan())
    }

    fn asin(self) -> Self {
        Self::new(self.value.asin())
    }

    fn acos(self) -> Self {
        Self::new(self.value.acos())
    }

    fn atan(self) -> Self {
        Self::new(self.value.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self::new(self.value.atan2(other.value))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        (Self::new(sin), Self::new(cos))
    }

    fn exp_m1(self) -> Self {
        Self::new(self.value.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self::new(self.value.ln_1p())
    }

    fn sinh(self) -> Self {
        Self::new(self.value.sinh())
    }

    fn cosh(self) -> Self {
        Self::new(self.value.cosh())
    }

    fn tanh(self) -> Self {
        Self::new(self.value.tanh())
    }

    fn asinh(self) -> Self {
        Self::new(self.value.asinh())
    }

    fn acosh(self) -> Self {
        Self::new(self.value.acosh())
    }

    fn atanh(self) -> Self {
        Self::new(self.value.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }
}

impl<T> Inv for Algebraic<T>
where
    T: AlgebraicFloatTrait,
{
    type Output = Self;

    fn inv(self) -> Self::Output {
        Self::one() / self
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use num_traits::{FloatConst, One, Zero};

    fn assert_num_traits<T>()
    where
        T: Float
            + FloatConst
            + ConstOne
            + ConstZero
            + Inv<Output = T>
            + Signed
            + FromPrimitive
            + ToPrimitive,
    {
    }

    #[test]
    fn implements_requested_num_traits_for_float_aliases() {
        assert_num_traits::<crate::af32>();
        assert_num_traits::<crate::af64>();

        let value = crate::af64::new(-3.5);
        assert_eq!(<crate::af64 as Float>::abs(value).value(), 3.5);
        assert_eq!(
            <crate::af64 as FloatConst>::PI().value(),
            core::f64::consts::PI
        );
        assert_eq!(<crate::af64 as Zero>::zero().value(), 0.0);
        assert_eq!(<crate::af64 as One>::one().value(), 1.0);
        assert_eq!(
            <crate::af64 as Inv>::inv(crate::af64::new(4.0)).value(),
            0.25
        );
        assert_eq!(
            <crate::af64 as Num>::from_str_radix("12", 10)
                .unwrap()
                .value(),
            12.0
        );
    }
}
