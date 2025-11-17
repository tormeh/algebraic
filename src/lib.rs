#![feature(f16)]
#![feature(f128)]
#![feature(float_algebraic)]

impl std::ops::Add for Algebraic<f16> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}
impl std::ops::Add for Algebraic<f32> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}
impl std::ops::Add for Algebraic<f64> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}
impl std::ops::Add for Algebraic<f128> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}

impl std::ops::Sub for Algebraic<f16> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
}
impl std::ops::Sub for Algebraic<f32> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}
impl std::ops::Sub for Algebraic<f64> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}
impl std::ops::Sub for Algebraic<f128> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
}

impl std::ops::Mul for Algebraic<f16> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}
impl std::ops::Mul for Algebraic<f32> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}
impl std::ops::Mul for Algebraic<f64> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}
impl std::ops::Mul for Algebraic<f128> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
}

impl std::ops::Div for Algebraic<f16> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl std::ops::Div for Algebraic<f32> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl std::ops::Div for Algebraic<f64> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl std::ops::Div for Algebraic<f128> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
}

impl std::ops::Rem for Algebraic<f16> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl std::ops::Rem for Algebraic<f32> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl std::ops::Rem for Algebraic<f64> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl std::ops::Rem for Algebraic<f128> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

pub struct Algebraic<T> {
    value: T,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_a = 2.0 + 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 + b2;

        assert_eq!(result_a, result_b.into());
    }
}
