use crate::algebraic::Algebraic;

// From implementations for creating Algebraic instances from primitive float types

impl From<f16> for Algebraic<f16> {
    fn from(value: f16) -> Self {
        Algebraic { value }
    }
}

impl From<f32> for Algebraic<f32> {
    fn from(value: f32) -> Self {
        Algebraic { value }
    }
}

impl From<f64> for Algebraic<f64> {
    fn from(value: f64) -> Self {
        Algebraic { value }
    }
}

impl From<f128> for Algebraic<f128> {
    fn from(value: f128) -> Self {
        Algebraic { value }
    }
}

// Into implementations for extracting primitive float types from Algebraic instances

impl Into<f16> for Algebraic<f16> {
    fn into(self) -> f16 {
        self.value
    }
}

impl Into<f32> for Algebraic<f32> {
    fn into(self) -> f32 {
        self.value
    }
}

impl Into<f64> for Algebraic<f64> {
    fn into(self) -> f64 {
        self.value
    }
}

impl Into<f128> for Algebraic<f128> {
    fn into(self) -> f128 {
        self.value
    }
}
