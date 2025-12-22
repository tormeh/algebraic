use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use std::iter::{Product, Sum};
use std::ops::{Add, Div, Mul, Rem, Sub};

impl<T: AlgebraicFloatTrait + Sized + Clone> Add for Algebraic<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait + Sized + Clone> Sub for Algebraic<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait + Sized + Clone> Mul for Algebraic<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait + Sized + Clone> Div for Algebraic<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait + Sized + Clone> Rem for Algebraic<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait + Sized + Clone> Sum for Algebraic<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl<T: AlgebraicFloatTrait + Sized + Clone> Product for Algebraic<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}
