use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// Implements `$Op` and `$OpAssign` for every owned/borrowed combination of `Algebraic<T>` and
/// `T` on the right-hand side, all via the algebraic intrinsic `$alg`.
macro_rules! impl_op {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident, $alg:ident) => {
        impl<T: AlgebraicFloatTrait> $Op<T> for Algebraic<T> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: T) -> Self {
                Self::new(self.value.$alg(rhs))
            }
        }

        impl<T: AlgebraicFloatTrait> $Op<&T> for Algebraic<T> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: &T) -> Self {
                self.$op(*rhs)
            }
        }

        impl<T: AlgebraicFloatTrait> $Op<T> for &Algebraic<T> {
            type Output = Algebraic<T>;

            #[inline]
            fn $op(self, rhs: T) -> Algebraic<T> {
                (*self).$op(rhs)
            }
        }

        impl<T: AlgebraicFloatTrait> $Op<&T> for &Algebraic<T> {
            type Output = Algebraic<T>;

            #[inline]
            fn $op(self, rhs: &T) -> Algebraic<T> {
                (*self).$op(*rhs)
            }
        }

        impl<T: AlgebraicFloatTrait> $Op for Algebraic<T> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: Self) -> Self {
                self.$op(rhs.value)
            }
        }

        impl<T: AlgebraicFloatTrait> $Op<&Self> for Algebraic<T> {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: &Self) -> Self {
                self.$op(rhs.value)
            }
        }

        impl<T: AlgebraicFloatTrait> $Op<Algebraic<T>> for &Algebraic<T> {
            type Output = Algebraic<T>;

            #[inline]
            fn $op(self, rhs: Algebraic<T>) -> Algebraic<T> {
                (*self).$op(rhs.value)
            }
        }

        impl<T: AlgebraicFloatTrait> $Op for &Algebraic<T> {
            type Output = Algebraic<T>;

            #[inline]
            fn $op(self, rhs: Self) -> Algebraic<T> {
                (*self).$op(rhs.value)
            }
        }

        impl<T: AlgebraicFloatTrait> $OpAssign<T> for Algebraic<T> {
            #[inline]
            fn $op_assign(&mut self, rhs: T) {
                *self = (*self).$op(rhs);
            }
        }

        impl<T: AlgebraicFloatTrait> $OpAssign<&T> for Algebraic<T> {
            #[inline]
            fn $op_assign(&mut self, rhs: &T) {
                *self = (*self).$op(*rhs);
            }
        }

        impl<T: AlgebraicFloatTrait> $OpAssign for Algebraic<T> {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                *self = (*self).$op(rhs.value);
            }
        }

        impl<T: AlgebraicFloatTrait> $OpAssign<&Self> for Algebraic<T> {
            #[inline]
            fn $op_assign(&mut self, rhs: &Self) {
                *self = (*self).$op(rhs.value);
            }
        }
    };
}

impl_op!(Add, add, AddAssign, add_assign, algebraic_add);
impl_op!(Sub, sub, SubAssign, sub_assign, algebraic_sub);
impl_op!(Mul, mul, MulAssign, mul_assign, algebraic_mul);
impl_op!(Div, div, DivAssign, div_assign, algebraic_div);
impl_op!(Rem, rem, RemAssign, rem_assign, algebraic_rem);

impl<T: AlgebraicFloatTrait> Sum for Algebraic<T> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T: AlgebraicFloatTrait> Sum<&'a Self> for Algebraic<T> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<T: AlgebraicFloatTrait> Sum<T> for Algebraic<T> {
    #[inline]
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T: AlgebraicFloatTrait> Sum<&'a T> for Algebraic<T> {
    #[inline]
    fn sum<I: Iterator<Item = &'a T>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<T: AlgebraicFloatTrait> Product for Algebraic<T> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl<'a, T: AlgebraicFloatTrait> Product<&'a Self> for Algebraic<T> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl<T: AlgebraicFloatTrait> Product<T> for Algebraic<T> {
    #[inline]
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl<'a, T: AlgebraicFloatTrait> Product<&'a T> for Algebraic<T> {
    #[inline]
    fn product<I: Iterator<Item = &'a T>>(iter: I) -> Self {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl<T: AlgebraicFloatTrait + Neg<Output = T>> Neg for Algebraic<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

impl<T: AlgebraicFloatTrait + Neg<Output = T>> Neg for &Algebraic<T> {
    type Output = Algebraic<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        -*self
    }
}
