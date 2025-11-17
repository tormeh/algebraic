#![feature(f16)]
#![feature(f128)]
#![feature(float_algebraic)]

pub trait AlgebraicFloatTrait {
    fn algebraic_add(self, rhs: Self) -> Self;
    fn algebraic_sub(self, rhs: Self) -> Self;
    fn algebraic_mul(self, rhs: Self) -> Self;
    fn algebraic_div(self, rhs: Self) -> Self;
    fn algebraic_rem(self, rhs: Self) -> Self;
}

impl AlgebraicFloatTrait for Algebraic<f16> {
    fn algebraic_add(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl AlgebraicFloatTrait for Algebraic<f32> {
    fn algebraic_add(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl AlgebraicFloatTrait for Algebraic<f64> {
    fn algebraic_add(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_rem(rhs.value),
        }
    }
}

impl AlgebraicFloatTrait for Algebraic<f128> {
    fn algebraic_add(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_add(rhs.value),
        }
    }
    fn algebraic_sub(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_sub(rhs.value),
        }
    }
    fn algebraic_mul(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_mul(rhs.value),
        }
    }
    fn algebraic_div(self, rhs: Self) -> Self {
        Algebraic {
            value: self.value.algebraic_div(rhs.value),
        }
    }
    fn algebraic_rem(self, rhs: Self) -> Self {
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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
