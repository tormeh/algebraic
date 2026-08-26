use crate::traits::AlgebraicFloatTrait;
use core::fmt::{Display, Formatter, Result};

/// A wrapper for float types, which allows for algebraic reordering by the compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
#[repr(transparent)]
pub struct Algebraic<T: AlgebraicFloatTrait> {
    pub(crate) value: T,
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
