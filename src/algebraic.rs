use crate::traits::AlgebraicFloatTrait;
use core::fmt::{Display, Formatter, Result};

/// A wrapper for float types, which allows for algebraic reordering by the compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Algebraic<T: AlgebraicFloatTrait> {
    pub(crate) value: T,
}

/// Compares the wrapped values with the semantics of `T`'s `PartialOrd`
/// (so any comparison involving `NaN` is `false`).
///
/// # Why are `lt`, `le`, `gt` and `ge` implemented explicitly?
///
/// They look redundant, since the trait provides default implementations in
/// terms of `partial_cmp`. Do not remove them. With `#[derive(PartialOrd)]`
/// (or with only `partial_cmp` implemented), `x < y` is compiled via
/// `Option<Ordering>`. For floats the optimizer does not reliably reduce this
/// to a single compare-and-branch; it emitted two compares plus a chain of
/// `setcc`/`cmov`/`test` instructions. Forwarding each operator directly to the
/// inner primitive yields the same code as using plain `f32`/`f64`. This
/// mattered, for example, in `num-quaternion`'s `Quaternion::norm`, where
/// `Quaternion<af32>` was slower than `Q32` because of these comparisons.
impl<T: AlgebraicFloatTrait + PartialOrd> PartialOrd for Algebraic<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }
}

impl<T: AlgebraicFloatTrait> Algebraic<T> {
    /// Creates a new `Algebraic` instance wrapping the given primitive floating-point value.
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns the additive identity (zero, `0.0`) of the wrapped floating-point type.
    #[must_use]
    pub fn zero() -> Self {
        Self { value: T::zero() }
    }

    /// Returns the multiplicative identity (one, `1.0`) of the wrapped floating-point type.
    #[must_use]
    pub fn one() -> Self {
        Self { value: T::one() }
    }

    /// Returns the wrapped inner raw floating-point value.
    ///
    /// This is equivalent to converting using `Into<T>` or `From`.
    pub const fn value(self) -> T {
        self.value
    }
}

// Display implementations
impl<T: AlgebraicFloatTrait + Display> Display for Algebraic<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl<T: AlgebraicFloatTrait> AsRef<T> for Algebraic<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: AlgebraicFloatTrait> AsMut<T> for Algebraic<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
