#![feature(f16)]
#![feature(f128)]
#![feature(float_algebraic)]

pub trait AlgebraicFloatTrait {
    fn algebraic_add(self, rhs: Self) -> Self;
    fn algebraic_sub(self, rhs: Self) -> Self;
    fn algebraic_mul(self, rhs: Self) -> Self;
    fn algebraic_div(self, rhs: Self) -> Self;
    fn algebraic_rem(self, rhs: Self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

impl AlgebraicFloatTrait for f16 {
    fn algebraic_add(self, rhs: Self) -> Self {
        Self::algebraic_add(self, rhs)
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Self::algebraic_sub(self, rhs)
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Self::algebraic_mul(self, rhs)
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Self::algebraic_div(self, rhs)
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        Self::algebraic_rem(self, rhs)
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl AlgebraicFloatTrait for f32 {
    fn algebraic_add(self, rhs: Self) -> Self {
        Self::algebraic_add(self, rhs)
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Self::algebraic_sub(self, rhs)
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Self::algebraic_mul(self, rhs)
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Self::algebraic_div(self, rhs)
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        Self::algebraic_rem(self, rhs)
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl AlgebraicFloatTrait for f64 {
    fn algebraic_add(self, rhs: Self) -> Self {
        Self::algebraic_add(self, rhs)
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Self::algebraic_sub(self, rhs)
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Self::algebraic_mul(self, rhs)
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Self::algebraic_div(self, rhs)
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        Self::algebraic_rem(self, rhs)
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl AlgebraicFloatTrait for f128 {
    fn algebraic_add(self, rhs: Self) -> Self {
        Self::algebraic_add(self, rhs)
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Self::algebraic_sub(self, rhs)
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Self::algebraic_mul(self, rhs)
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Self::algebraic_div(self, rhs)
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        Self::algebraic_rem(self, rhs)
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

pub struct Algebraic<T: AlgebraicFloatTrait + Sized + Clone> {
    value: T,
}

#[allow(non_camel_case_types)]
pub type af16 = Algebraic<f16>;
#[allow(non_camel_case_types)]
pub type af32 = Algebraic<f32>;
#[allow(non_camel_case_types)]
pub type af64 = Algebraic<f64>;
#[allow(non_camel_case_types)]
pub type af128 = Algebraic<f128>;

impl<T: AlgebraicFloatTrait + Sized + Clone> Algebraic<T> {
    fn zero() -> Self {
        Self { value: T::zero() }
    }
    fn one() -> Self {
        Self { value: T::one() }
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> std::ops::Add for Algebraic<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> Clone for Algebraic<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> std::ops::Sub for Algebraic<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> std::ops::Mul for Algebraic<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> std::ops::Div for Algebraic<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> std::ops::Rem for Algebraic<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> std::iter::Sum for Algebraic<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}
impl<T: AlgebraicFloatTrait + Sized + Clone> std::iter::Product for Algebraic<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

impl From<f16> for Algebraic<f16> {
    fn from(value: f16) -> Self {
        Algebraic { value }
    }
}

impl From<f32> for Algebraic<f32> {
    fn from(value: f32) -> Self {
        Algebraic { value }
    }
}
impl From<f64> for Algebraic<f64> {
    fn from(value: f64) -> Self {
        Algebraic { value }
    }
}
impl From<f128> for Algebraic<f128> {
    fn from(value: f128) -> Self {
        Algebraic { value }
    }
}

impl Into<f16> for Algebraic<f16> {
    fn into(self) -> f16 {
        self.value
    }
}
impl Into<f32> for Algebraic<f32> {
    fn into(self) -> f32 {
        self.value
    }
}
impl Into<f64> for Algebraic<f64> {
    fn into(self) -> f64 {
        self.value
    }
}
impl Into<f128> for Algebraic<f128> {
    fn into(self) -> f128 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result_a = 2.0 + 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 + b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn sub() {
        let result_a = 2.0 - 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 - b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn mul() {
        let result_a = 2.0 * 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 * b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn div() {
        let result_a = 2.0 / 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 / b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn rem() {
        let result_a = 2.0 % 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 % b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn sum() {
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(8.0);
        let v = vec![b1, b2];
        let result: Algebraic<f64> = v.into_iter().sum();
        let result_f64: f64 = result.into();

        assert_eq!(result_f64, 10.0);
    }

    #[test]
    fn product() {
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(8.0);
        let v = vec![b1, b2];
        let result: Algebraic<f64> = v.into_iter().product();
        let result_f64: f64 = result.into();
        assert_eq!(result_f64, 16.0);
    }

    #[test]
    fn iter_clone() {
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(8.0);
        let v = vec![b1, b2];
        let result: Algebraic<f64> = v.clone().into_iter().last().unwrap();
        let result_f64: f64 = result.into();
        assert_eq!(result_f64, 8.0);
    }
}
