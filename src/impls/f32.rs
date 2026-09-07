use crate::algebraic::Algebraic;
use crate::ops::{
    reverse_add, reverse_add_assign, reverse_div, reverse_div_assign, reverse_mul,
    reverse_mul_assign, reverse_rem, reverse_rem_assign, reverse_sub, reverse_sub_assign,
};
use crate::traits::AlgebraicFloatTrait;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

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

// Reversed mixed-type operations: `f32 op Algebraic<f32>`

impl Add<Algebraic<Self>> for f32 {
    type Output = Algebraic<Self>;

    fn add(self, rhs: Algebraic<Self>) -> Self::Output {
        reverse_add(self, rhs)
    }
}

impl Sub<Algebraic<Self>> for f32 {
    type Output = Algebraic<Self>;

    fn sub(self, rhs: Algebraic<Self>) -> Self::Output {
        reverse_sub(self, rhs)
    }
}

impl Mul<Algebraic<Self>> for f32 {
    type Output = Algebraic<Self>;

    fn mul(self, rhs: Algebraic<Self>) -> Self::Output {
        reverse_mul(self, rhs)
    }
}

impl Div<Algebraic<Self>> for f32 {
    type Output = Algebraic<Self>;

    fn div(self, rhs: Algebraic<Self>) -> Self::Output {
        reverse_div(self, rhs)
    }
}

impl Rem<Algebraic<Self>> for f32 {
    type Output = Algebraic<Self>;

    fn rem(self, rhs: Algebraic<Self>) -> Self::Output {
        reverse_rem(self, rhs)
    }
}

impl AddAssign<Algebraic<Self>> for f32 {
    fn add_assign(&mut self, rhs: Algebraic<Self>) {
        reverse_add_assign(self, rhs);
    }
}

impl SubAssign<Algebraic<Self>> for f32 {
    fn sub_assign(&mut self, rhs: Algebraic<Self>) {
        reverse_sub_assign(self, rhs);
    }
}

impl MulAssign<Algebraic<Self>> for f32 {
    fn mul_assign(&mut self, rhs: Algebraic<Self>) {
        reverse_mul_assign(self, rhs);
    }
}

impl DivAssign<Algebraic<Self>> for f32 {
    fn div_assign(&mut self, rhs: Algebraic<Self>) {
        reverse_div_assign(self, rhs);
    }
}

impl RemAssign<Algebraic<Self>> for f32 {
    fn rem_assign(&mut self, rhs: Algebraic<Self>) {
        reverse_rem_assign(self, rhs);
    }
}
