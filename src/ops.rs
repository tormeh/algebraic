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

impl<'a, T: AlgebraicFloatTrait> Add for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Sub for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Mul for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Div for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Rem for &'a Algebraic<T> {
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

impl<'a, T: AlgebraicFloatTrait> Sum<&'a Algebraic<T>> for Algebraic<T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Algebraic<T> {
        iter.fold(Algebraic::zero(), |acc: Algebraic<T>, x| acc + *x)
    }
}

impl<T: AlgebraicFloatTrait> Product for Algebraic<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

impl<'a, T: AlgebraicFloatTrait> Product<&'a Algebraic<T>> for Algebraic<T> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Algebraic<T> {
        iter.fold(Algebraic::one(), |acc: Algebraic<T>, x| acc * *x)
    }
}

// Mixed implementations: Algebraic<T> op &Algebraic<T>

impl<'a, T: AlgebraicFloatTrait> Add<&'a Algebraic<T>> for Algebraic<T> {
    type Output = Algebraic<T>;

    fn add(self, rhs: &'a Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Sub<&'a Algebraic<T>> for Algebraic<T> {
    type Output = Algebraic<T>;

    fn sub(self, rhs: &'a Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Mul<&'a Algebraic<T>> for Algebraic<T> {
    type Output = Algebraic<T>;

    fn mul(self, rhs: &'a Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Div<&'a Algebraic<T>> for Algebraic<T> {
    type Output = Algebraic<T>;

    fn div(self, rhs: &'a Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Rem<&'a Algebraic<T>> for Algebraic<T> {
    type Output = Algebraic<T>;

    fn rem(self, rhs: &'a Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Add<Algebraic<T>> for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn add(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Sub<Algebraic<T>> for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn sub(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Mul<Algebraic<T>> for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn mul(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Div<Algebraic<T>> for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn div(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl<'a, T: AlgebraicFloatTrait> Rem<Algebraic<T>> for &'a Algebraic<T> {
    type Output = Algebraic<T>;

    fn rem(self, rhs: Algebraic<T>) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}
