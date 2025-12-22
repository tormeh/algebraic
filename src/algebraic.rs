use crate::traits::AlgebraicFloatTrait;

/// A wrapper struct for algebraic floating-point operations
#[derive(Debug, PartialEq)]
pub struct Algebraic<T: AlgebraicFloatTrait + Sized + Clone> {
    pub(crate) value: T,
}

impl<T: AlgebraicFloatTrait + Sized + Clone> Algebraic<T> {
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

impl<T: AlgebraicFloatTrait + Sized + Clone> Clone for Algebraic<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
