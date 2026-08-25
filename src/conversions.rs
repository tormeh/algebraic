use crate::algebraic::Algebraic;

// From implementations for creating Algebraic instances from primitive float types

#[cfg(feature = "f16")]
impl From<f16> for Algebraic<f16> {
    fn from(value: f16) -> Self {
        Self { value }
    }
}

impl From<f32> for Algebraic<f32> {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<f64> for Algebraic<f64> {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

#[cfg(feature = "f128")]
impl From<f128> for Algebraic<f128> {
    fn from(value: f128) -> Self {
        Self { value }
    }
}

// Into implementations for extracting primitive float types from Algebraic instances

#[cfg(feature = "f16")]
impl From<Algebraic<Self>> for f16 {
    fn from(val: Algebraic<Self>) -> Self {
        val.value
    }
}

impl From<Algebraic<Self>> for f32 {
    fn from(val: Algebraic<Self>) -> Self {
        val.value
    }
}

impl From<Algebraic<Self>> for f64 {
    fn from(val: Algebraic<Self>) -> Self {
        val.value
    }
}

#[cfg(feature = "f128")]
impl From<Algebraic<Self>> for f128 {
    fn from(val: Algebraic<Self>) -> Self {
        val.value
    }
}
