//! Tests for the algebraic floating-point library

use crate::algebraic::Algebraic;
use crate::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result_a = 2.0 + 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 + b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_sub() {
        let result_a = 2.0 - 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 - b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_mul() {
        let result_a = 4.0 * 2.0;
        let b1: Algebraic<f64> = Algebraic::from(4.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 * b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_div() {
        let result_a = 4.0 / 2.0;
        let b1: Algebraic<f64> = Algebraic::from(4.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 / b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_rem() {
        let result_a = 8.0 % 2.0;
        let b1: Algebraic<f64> = Algebraic::from(8.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 % b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_sum() {
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(8.0);
        let v = vec![b1, b2];
        let result: Algebraic<f64> = v.into_iter().sum();
        let result_f64: f64 = result.into();

        assert_eq!(result_f64, 10.0);
    }

    #[test]
    fn test_product() {
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(8.0);
        let v = vec![b1, b2];
        let result: Algebraic<f64> = v.into_iter().product();
        let result_f64: f64 = result.into();
        assert_eq!(result_f64, 16.0);
    }

    #[test]
    fn test_iter_clone() {
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(8.0);
        let v = vec![b1, b2];
        let result: Algebraic<f64> = v.clone().into_iter().last().unwrap();
        let result_f64: f64 = result.into();
        assert_eq!(result_f64, 8.0);
    }

    #[test]
    fn test_zero_one() {
        let zero: Algebraic<f64> = Algebraic::zero();
        let one: Algebraic<f64> = Algebraic::one();

        assert_eq!(0.0_f64, zero.into());
        assert_eq!(1.0_f64, one.into());
    }

    #[test]
    fn test_new() {
        let value = Algebraic::new(42.0_f32);
        assert_eq!(42.0_f32, value.into());
    }

    #[test]
    fn test_value_method() {
        let algebraic = Algebraic::new(3.14_f64);
        assert_eq!(3.14_f64, algebraic.value());
    }

    #[test]
    fn test_type_aliases() {
        let a16: af16 = Algebraic::from(1.0_f16);
        let a32: af32 = Algebraic::from(2.0_f32);
        let a64: af64 = Algebraic::from(3.0_f64);
        let a128: af128 = Algebraic::from(4.0_f128);

        assert_eq!(1.0_f16, a16.into());
        assert_eq!(2.0_f32, a32.into());
        assert_eq!(3.0_f64, a64.into());
        assert_eq!(4.0_f128, a128.into());
    }

    fn example_vec() -> Vec<af64> {
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(8.0);
        let b3: Algebraic<f64> = Algebraic::from(5.5);
        let b4: Algebraic<f64> = Algebraic::from(31.0);
        let v = vec![b1, b2, b3, b4];
        v
    }

    #[test]
    fn test_iter_sum() {
        let v = example_vec();
        let sum: Algebraic<f64> = v.clone().iter().sum();
        let sum_f64: f64 = sum.into();
        assert_eq!(sum_f64, 46.5);
    }

    #[test]
    fn test_iter_sum_into() {
        let v = example_vec();
        let sum: Algebraic<f64> = v.clone().into_iter().sum();
        let sum_f64: f64 = sum.into();
        assert_eq!(sum_f64, 46.5);
    }

    #[test]
    fn test_iter_product() {
        let v = example_vec();
        let product: Algebraic<f64> = v.clone().iter().product();
        let product_f64: f64 = product.into();
        assert_eq!(product_f64, 5720.0);
    }

    #[test]
    fn test_iter_product_into() {
        let v = example_vec();
        let product: Algebraic<f64> = v.clone().into_iter().product();
        let product_f64: f64 = product.into();
        assert_eq!(product_f64, 5720.0);
    }
}
