use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use std::iter::{Product, Sum};
use std::ops::{Add, Div, Mul, Rem, Sub};

impl<T: AlgebraicFloatTrait> Add for Algebraic<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Sub for Algebraic<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Mul for Algebraic<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Div for Algebraic<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Rem for Algebraic<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Add for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Sub for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Mul for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Div for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Rem for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Sum for Algebraic<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl<'a, T: AlgebraicFloatTrait> Sum<&'a Self> for Algebraic<T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc: Self, x| acc + *x)
    }
}

impl<T: AlgebraicFloatTrait> Product for Algebraic<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

impl<'a, T: AlgebraicFloatTrait> Product<&'a Self> for Algebraic<T> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc: Self, x| acc * *x)
    }
}

// Mixed implementations: Algebraic<T> op &Algebraic<T>

impl<'a, T: AlgebraicFloatTrait> Add<&'a Self> for Algebraic<T> {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {
        Self {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Sub<&'a Self> for Algebraic<T> {
    type Output = Self;

    fn sub(self, rhs: &'a Self) -> Self::Output {
        Self {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Mul<&'a Self> for Algebraic<T> {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        Self {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Div<&'a Self> for Algebraic<T> {
    type Output = Self;

    fn div(self, rhs: &'a Self) -> Self::Output {
        Self {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Rem<&'a Self> for Algebraic<T> {
    type Output = Self;

    fn rem(self, rhs: &'a Self) -> Self::Output {
        Self {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Add<Algebraic<T>> for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn add(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Sub<Algebraic<T>> for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn sub(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Mul<Algebraic<T>> for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn mul(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Div<Algebraic<T>> for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn div(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Rem<Algebraic<T>> for &Algebraic<T> {
    type Output = Algebraic<T>;

    fn rem(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl<T: AlgebraicFloatTrait> Add<T> for Algebraic<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.algebraic_add(rhs),
        }
    }
}

impl<T: AlgebraicFloatTrait> Sub<T> for Algebraic<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.algebraic_sub(rhs),
        }
    }
}

impl<T: AlgebraicFloatTrait> Mul<T> for Algebraic<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.algebraic_mul(rhs),
        }
    }
}

impl<T: AlgebraicFloatTrait> Div<T> for Algebraic<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.algebraic_div(rhs),
        }
    }
}

impl<T: AlgebraicFloatTrait> Rem<T> for Algebraic<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.algebraic_rem(rhs),
        }
    }
}
