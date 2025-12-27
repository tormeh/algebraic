use crate::traits::AlgebraicFloatTrait;
use std::fmt::{Display, Formatter, Result};

/// A wrapper struct for algebraic floating-point operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Algebraic<T: AlgebraicFloatTrait> {
    pub(crate) value: T,
}

impl<T: AlgebraicFloatTrait> Algebraic<T> {
    /// Create a new Algebraic instance with the given value
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Return the additive identity (zero)
    pub fn zero() -> Self {
        Self { value: T::zero() }
    }

    /// Return the multiplicative identity (one)
    pub fn one() -> Self {
        Self { value: T::one() }
    }

    /// Get the inner value
    pub fn value(self) -> T {
        self.value
    }
}

// Display implementations

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
