/// Trait defining algebraic operations for floating-point types.
///
/// This trait acts as an abstraction over primitive floating-point types, exposing
/// the compiler-assisted algebraic mathematical operations (such as `f32::algebraic_add`,
/// `f64::algebraic_add`, etc.).
///
/// If you are introducing a new custom floating-point type (e.g., behind a feature flag or a new stable float),
/// you should implement this trait to map standard arithmetic operations to their corresponding algebraic
/// variants or intrinsics.
///
/// # Examples
///
/// ```rust
/// use algebraic::AlgebraicFloatTrait;
///
/// let a = 2.0_f64;
/// let b = 3.0_f64;
///
/// // Call through the trait; `a.algebraic_add(b)` would call the inherent `f64` method instead
/// let sum = AlgebraicFloatTrait::algebraic_add(a, b);
/// assert_eq!(sum, 5.0);
/// ```
pub trait AlgebraicFloatTrait: Copy {
    /// Performs algebraic addition of `self` and `rhs`.
    #[must_use]
    fn algebraic_add(self, rhs: Self) -> Self;

    /// Performs algebraic subtraction of `rhs` from `self`.
    #[must_use]
    fn algebraic_sub(self, rhs: Self) -> Self;

    /// Performs algebraic multiplication of `self` and `rhs`.
    #[must_use]
    fn algebraic_mul(self, rhs: Self) -> Self;

    /// Performs algebraic division of `self` by `rhs`.
    #[must_use]
    fn algebraic_div(self, rhs: Self) -> Self;

    /// Performs the algebraic remainder operation (`self % rhs`).
    #[must_use]
    fn algebraic_rem(self, rhs: Self) -> Self;

    /// Returns the additive identity (zero, `0.0`) for the floating-point type.
    fn zero() -> Self;

    /// Returns the multiplicative identity (one, `1.0`) for the floating-point type.
    fn one() -> Self;
}
