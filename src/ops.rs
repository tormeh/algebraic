use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

impl<T: AlgebraicFloatTrait> AddAssign for Algebraic<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value = self.value.algebraic_add(rhs.value);
    }
}

impl<T: AlgebraicFloatTrait> SubAssign for Algebraic<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = self.value.algebraic_sub(rhs.value);
    }
}

impl<T: AlgebraicFloatTrait> MulAssign for Algebraic<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = self.value.algebraic_mul(rhs.value);
    }
}

impl<T: AlgebraicFloatTrait> DivAssign for Algebraic<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.value = self.value.algebraic_div(rhs.value);
    }
}

impl<T: AlgebraicFloatTrait> RemAssign for Algebraic<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.value = self.value.algebraic_rem(rhs.value);
    }
}

impl<T: AlgebraicFloatTrait> AddAssign<T> for Algebraic<T> {
    fn add_assign(&mut self, rhs: T) {
        self.value = self.value.algebraic_add(rhs);
    }
}

impl<T: AlgebraicFloatTrait> SubAssign<T> for Algebraic<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.value = self.value.algebraic_sub(rhs);
    }
}

impl<T: AlgebraicFloatTrait> MulAssign<T> for Algebraic<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.value = self.value.algebraic_mul(rhs);
    }
}

impl<T: AlgebraicFloatTrait> DivAssign<T> for Algebraic<T> {
    fn div_assign(&mut self, rhs: T) {
        self.value = self.value.algebraic_div(rhs);
    }
}

impl<T: AlgebraicFloatTrait> RemAssign<T> for Algebraic<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.value = self.value.algebraic_rem(rhs);
    }
}

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

impl<T: AlgebraicFloatTrait> Sum<T> for Algebraic<T> {
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl<'a, T: AlgebraicFloatTrait> Sum<&'a T> for Algebraic<T> {
    fn sum<I: Iterator<Item = &'a T>>(iter: I) -> Self {
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

impl<T: AlgebraicFloatTrait> Product<T> for Algebraic<T> {
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}

impl<'a, T: AlgebraicFloatTrait> Product<&'a T> for Algebraic<T> {
    fn product<I: Iterator<Item = &'a T>>(iter: I) -> Self {
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

// Shared logic for the reversed mixed-type operations: `T op Algebraic<T>`.
//
// Rust's orphan rules forbid a blanket `impl<T: AlgebraicFloatTrait> Add<Algebraic<T>> for T`,
// since `T` would be an uncovered type parameter used as `Self` for a foreign trait
// (`core::ops::Add`) applied to a foreign type. Each concrete floating-point type
// (see `src/impls`) must therefore provide its own `impl Add<Algebraic<T>> for T`, etc.
// These generic helpers let those per-type impls share a single implementation of the
// underlying arithmetic instead of duplicating it per type.

pub fn reverse_add<T: AlgebraicFloatTrait>(lhs: T, rhs: Algebraic<T>) -> Algebraic<T> {
    Algebraic {
        value: lhs.algebraic_add(rhs.value),
    }
}

pub fn reverse_sub<T: AlgebraicFloatTrait>(lhs: T, rhs: Algebraic<T>) -> Algebraic<T> {
    Algebraic {
        value: lhs.algebraic_sub(rhs.value),
    }
}

pub fn reverse_mul<T: AlgebraicFloatTrait>(lhs: T, rhs: Algebraic<T>) -> Algebraic<T> {
    Algebraic {
        value: lhs.algebraic_mul(rhs.value),
    }
}

pub fn reverse_div<T: AlgebraicFloatTrait>(lhs: T, rhs: Algebraic<T>) -> Algebraic<T> {
    Algebraic {
        value: lhs.algebraic_div(rhs.value),
    }
}

pub fn reverse_rem<T: AlgebraicFloatTrait>(lhs: T, rhs: Algebraic<T>) -> Algebraic<T> {
    Algebraic {
        value: lhs.algebraic_rem(rhs.value),
    }
}

pub fn reverse_add_assign<T: AlgebraicFloatTrait>(lhs: &mut T, rhs: Algebraic<T>) {
    *lhs = lhs.algebraic_add(rhs.value);
}

pub fn reverse_sub_assign<T: AlgebraicFloatTrait>(lhs: &mut T, rhs: Algebraic<T>) {
    *lhs = lhs.algebraic_sub(rhs.value);
}

pub fn reverse_mul_assign<T: AlgebraicFloatTrait>(lhs: &mut T, rhs: Algebraic<T>) {
    *lhs = lhs.algebraic_mul(rhs.value);
}

pub fn reverse_div_assign<T: AlgebraicFloatTrait>(lhs: &mut T, rhs: Algebraic<T>) {
    *lhs = lhs.algebraic_div(rhs.value);
}

pub fn reverse_rem_assign<T: AlgebraicFloatTrait>(lhs: &mut T, rhs: Algebraic<T>) {
    *lhs = lhs.algebraic_rem(rhs.value);
}
