#![feature(f16)]
#![feature(f128)]

struct Algebraic<T> {
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
