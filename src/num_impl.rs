use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use core::num::FpCategory;
use core::ops::Neg;
use num_traits::{
    ConstOne, ConstZero, Float, FloatConst, FromPrimitive, Inv, Num, NumCast, One, Signed,
    ToPrimitive, Zero,
};

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

macro_rules! forward_float_static {
    ($($method:ident),+ $(,)?) => {
        $(
            fn $method() -> Self {
                Self::new(T::$method())
            }
        )+
    };
}

macro_rules! forward_float_unary {
    ($($method:ident),+ $(,)?) => {
        $(
            fn $method(self) -> Self {
                Self::new(self.value.$method())
            }
        )+
    };
}

macro_rules! forward_float_predicates {
    ($($method:ident),+ $(,)?) => {
        $(
            fn $method(self) -> bool {
                self.value.$method()
            }
        )+
    };
}

macro_rules! forward_float_binary {
    ($($method:ident),+ $(,)?) => {
        $(
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

impl<T> Float for Algebraic<T>
where
    T: AlgebraicFloatTrait + Float + FloatConst,
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

    fn recip(self) -> Self {
        Self::one() / self
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::new(self.value.mul_add(a.value, b.value))
    }

    fn powi(self, n: i32) -> Self {
        Self::new(self.value.powi(n))
    }

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

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        (Self::new(sin), Self::new(cos))
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
