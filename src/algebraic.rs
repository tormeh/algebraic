use crate::traits::AlgebraicFloatTrait;
use std::fmt::{Display, Formatter, Result};

/// A wrapper struct for algebraic floating-point operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Algebraic<T: AlgebraicFloatTrait> {
    pub(crate) value: T,
}

impl<T: AlgebraicFloatTrait> Algebraic<T> {
    /// Create a new Algebraic instance with the given value
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Return the additive identity (zero)
    #[must_use]
    pub fn zero() -> Self {
        Self { value: T::zero() }
    }

    /// Return the multiplicative identity (one)
    #[must_use]
    pub fn one() -> Self {
        Self { value: T::one() }
    }

    /// Get the inner value
    pub const fn value(self) -> T {
        self.value
    }
}

// Display implementations
#[cfg(feature = "f16")]
impl Display for Algebraic<f16> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Algebraic<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Algebraic<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}
