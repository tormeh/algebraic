/// Trait defining algebraic operations for floating-point types
pub trait AlgebraicFloatTrait: Copy + Clone {
    /// Perform algebraic addition
    #[must_use]
    fn algebraic_add(self, rhs: Self) -> Self;

    /// Perform algebraic subtraction
    #[must_use]
    fn algebraic_sub(self, rhs: Self) -> Self;

    /// Perform algebraic multiplication
    #[must_use]
    fn algebraic_mul(self, rhs: Self) -> Self;

    /// Perform algebraic division
    #[must_use]
    fn algebraic_div(self, rhs: Self) -> Self;

    /// Perform algebraic remainder operation
    #[must_use]
    fn algebraic_rem(self, rhs: Self) -> Self;

    /// Return the additive identity (zero)
    fn zero() -> Self;

    /// Return the multiplicative identity (one)
    fn one() -> Self;
}
