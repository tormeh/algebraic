use crate::traits::AlgebraicFloatTrait;

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
