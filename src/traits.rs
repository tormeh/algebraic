/// Trait defining algebraic operations for floating-point types
pub trait AlgebraicFloatTrait {
    /// Perform algebraic addition
    fn algebraic_add(self, rhs: Self) -> Self;

    /// Perform algebraic subtraction
    fn algebraic_sub(self, rhs: Self) -> Self;

    /// Perform algebraic multiplication
    fn algebraic_mul(self, rhs: Self) -> Self;

    /// Perform algebraic division
    fn algebraic_div(self, rhs: Self) -> Self;

    /// Perform algebraic remainder operation
    fn algebraic_rem(self, rhs: Self) -> Self;

    /// Return the additive identity (zero)
    fn zero() -> Self;

    /// Return the multiplicative identity (one)
    fn one() -> Self;
}
