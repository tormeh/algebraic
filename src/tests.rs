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
        assert_eq!(product_f64, 2728.0);
    }

    #[test]
    fn test_iter_product_into() {
        let v = example_vec();
        let product: Algebraic<f64> = v.clone().into_iter().product();
        let product_f64: f64 = product.into();
        assert_eq!(product_f64, 2728.0);
    }

    #[test]
    fn test_add_ref() {
        let result_a = 2.0 + 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = &b1 + &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_sub_ref() {
        let result_a = 5.0 - 3.0;
        let b1: Algebraic<f64> = Algebraic::from(5.0);
        let b2: Algebraic<f64> = Algebraic::from(3.0);
        let result_b: Algebraic<f64> = &b1 - &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_mul_ref() {
        let result_a = 4.0 * 2.0;
        let b1: Algebraic<f64> = Algebraic::from(4.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = &b1 * &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_div_ref() {
        let result_a = 8.0 / 2.0;
        let b1: Algebraic<f64> = Algebraic::from(8.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = &b1 / &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_add_mixed() {
        let result_a = 2.0 + 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 + &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_sub_mixed() {
        let result_a = 5.0 - 3.0;
        let b1: Algebraic<f64> = Algebraic::from(5.0);
        let b2: Algebraic<f64> = Algebraic::from(3.0);
        let result_b: Algebraic<f64> = b1 - &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_mul_mixed() {
        let result_a = 4.0 * 2.0;
        let b1: Algebraic<f64> = Algebraic::from(4.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 * &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_div_mixed() {
        let result_a = 8.0 / 2.0;
        let b1: Algebraic<f64> = Algebraic::from(8.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = b1 / &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_all_operation_combinations() {
        let a = Algebraic::from(10.0_f64);
        let b = Algebraic::from(5.0_f64);

        // Test all combinations for addition
        let r1 = Algebraic::from(10.0) + Algebraic::from(5.0); // owned + owned
        let r2 = &a + &b; // ref + ref
        let r3 = Algebraic::from(10.0) + &b; // owned + ref
        let r4 = &a + Algebraic::from(5.0); // ref + owned

        assert_eq!(15.0_f64, r1.into());
        assert_eq!(15.0_f64, r2.into());
        assert_eq!(15.0_f64, r3.into());
        assert_eq!(15.0_f64, r4.into());

        // Test all combinations for subtraction
        let r1 = Algebraic::from(10.0) - Algebraic::from(5.0); // owned - owned
        let r2 = &a - &b; // ref - ref
        let r3 = Algebraic::from(10.0) - &b; // owned - ref
        let r4 = &a - Algebraic::from(5.0); // ref - owned

        assert_eq!(5.0_f64, r1.into());
        assert_eq!(5.0_f64, r2.into());
        assert_eq!(5.0_f64, r3.into());
        assert_eq!(5.0_f64, r4.into());

        // Test all combinations for multiplication
        let r1 = Algebraic::from(10.0) * Algebraic::from(5.0); // owned * owned
        let r2 = &a * &b; // ref * ref
        let r3 = Algebraic::from(10.0) * &b; // owned * ref
        let r4 = &a * Algebraic::from(5.0); // ref * owned

        assert_eq!(50.0_f64, r1.into());
        assert_eq!(50.0_f64, r2.into());
        assert_eq!(50.0_f64, r3.into());
        assert_eq!(50.0_f64, r4.into());

        // Test all combinations for division
        let r1 = Algebraic::from(10.0) / Algebraic::from(5.0); // owned / owned
        let r2 = &a / &b; // ref / ref
        let r3 = Algebraic::from(10.0) / &b; // owned / ref
        let r4 = &a / Algebraic::from(5.0); // ref / owned

        assert_eq!(2.0_f64, r1.into());
        assert_eq!(2.0_f64, r2.into());
        assert_eq!(2.0_f64, r3.into());
        assert_eq!(2.0_f64, r4.into());

        // Test all combinations for remainder
        let c = Algebraic::from(13.0_f64);
        let d = Algebraic::from(5.0_f64);

        let r1 = Algebraic::from(13.0) % Algebraic::from(5.0); // owned % owned
        let r2 = &c % &d; // ref % ref
        let r3 = Algebraic::from(13.0) % &d; // owned % ref
        let r4 = &c % Algebraic::from(5.0); // ref % owned

        assert_eq!(3.0_f64, r1.into());
        assert_eq!(3.0_f64, r2.into());
        assert_eq!(3.0_f64, r3.into());
        assert_eq!(3.0_f64, r4.into());
    }

    #[test]
    fn test_add_mixed_ref_owned() {
        let result_a = 2.0 + 2.0;
        let b1: Algebraic<f64> = Algebraic::from(2.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = &b1 + b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_sub_mixed_ref_owned() {
        let result_a = 5.0 - 3.0;
        let b1: Algebraic<f64> = Algebraic::from(5.0);
        let b2: Algebraic<f64> = Algebraic::from(3.0);
        let result_b: Algebraic<f64> = &b1 - b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_mul_mixed_ref_owned() {
        let result_a = 4.0 * 2.0;
        let b1: Algebraic<f64> = Algebraic::from(4.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = &b1 * b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_div_mixed_ref_owned() {
        let result_a = 8.0 / 2.0;
        let b1: Algebraic<f64> = Algebraic::from(8.0);
        let b2: Algebraic<f64> = Algebraic::from(2.0);
        let result_b: Algebraic<f64> = &b1 / b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_rem_ref() {
        let result_a = 8.0 % 3.0;
        let b1: Algebraic<f64> = Algebraic::from(8.0);
        let b2: Algebraic<f64> = Algebraic::from(3.0);
        let result_b: Algebraic<f64> = &b1 % &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_rem_mixed() {
        let result_a = 8.0 % 3.0;
        let b1: Algebraic<f64> = Algebraic::from(8.0);
        let b2: Algebraic<f64> = Algebraic::from(3.0);
        let result_b: Algebraic<f64> = b1 % &b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_rem_mixed_ref_owned() {
        let result_a = 8.0 % 3.0;
        let b1: Algebraic<f64> = Algebraic::from(8.0);
        let b2: Algebraic<f64> = Algebraic::from(3.0);
        let result_b: Algebraic<f64> = &b1 % b2;

        assert_eq!(result_a, result_b.into());
    }

    #[test]
    fn test_add_f64_primitive() {
        let a: Algebraic<f64> = Algebraic::from(10.0);
        let result = a + 5.0;
        assert_eq!(15.0, result.into());
    }

    #[test]
    fn test_sub_f64_primitive() {
        let a: Algebraic<f64> = Algebraic::from(10.0);
        let result = a - 5.0;
        assert_eq!(5.0, result.into());
    }

    #[test]
    fn test_mul_f64_primitive() {
        let a: Algebraic<f64> = Algebraic::from(10.0);
        let result = a * 5.0;
        assert_eq!(50.0, result.into());
    }

    #[test]
    fn test_div_f64_primitive() {
        let a: Algebraic<f64> = Algebraic::from(10.0);
        let result = a / 5.0;
        assert_eq!(2.0, result.into());
    }

    #[test]
    fn test_rem_f64_primitive() {
        let a: Algebraic<f64> = Algebraic::from(13.0);
        let result = a % 5.0;
        assert_eq!(3.0, result.into());
    }

    #[test]
    fn test_all_f64_primitive_operations() {
        let a = Algebraic::from(20.0_f64);

        assert_eq!(25.0, (a + 5.0).into());
        assert_eq!(15.0, (a - 5.0).into());
        assert_eq!(100.0, (a * 5.0).into());
        assert_eq!(4.0, (a / 5.0).into());
        assert_eq!(2.0, (a % 3.0).into());
    }

    #[test]
    fn test_add_f16_primitive() {
        let a: Algebraic<f16> = Algebraic::from(10.0_f16);
        let result = a + 5.0_f16;
        assert_eq!(15.0_f16, result.into());
    }

    #[test]
    fn test_sub_f16_primitive() {
        let a: Algebraic<f16> = Algebraic::from(10.0_f16);
        let result = a - 5.0_f16;
        assert_eq!(5.0_f16, result.into());
    }

    #[test]
    fn test_mul_f16_primitive() {
        let a: Algebraic<f16> = Algebraic::from(10.0_f16);
        let result = a * 5.0_f16;
        assert_eq!(50.0_f16, result.into());
    }

    #[test]
    fn test_div_f16_primitive() {
        let a: Algebraic<f16> = Algebraic::from(10.0_f16);
        let result = a / 5.0_f16;
        assert_eq!(2.0_f16, result.into());
    }

    #[test]
    fn test_rem_f16_primitive() {
        let a: Algebraic<f16> = Algebraic::from(13.0_f16);
        let result = a % 5.0_f16;
        assert_eq!(3.0_f16, result.into());
    }

    #[test]
    fn test_add_f32_primitive() {
        let a: Algebraic<f32> = Algebraic::from(10.0_f32);
        let result = a + 5.0_f32;
        assert_eq!(15.0_f32, result.into());
    }

    #[test]
    fn test_sub_f32_primitive() {
        let a: Algebraic<f32> = Algebraic::from(10.0_f32);
        let result = a - 5.0_f32;
        assert_eq!(5.0_f32, result.into());
    }

    #[test]
    fn test_mul_f32_primitive() {
        let a: Algebraic<f32> = Algebraic::from(10.0_f32);
        let result = a * 5.0_f32;
        assert_eq!(50.0_f32, result.into());
    }

    #[test]
    fn test_div_f32_primitive() {
        let a: Algebraic<f32> = Algebraic::from(10.0_f32);
        let result = a / 5.0_f32;
        assert_eq!(2.0_f32, result.into());
    }

    #[test]
    fn test_rem_f32_primitive() {
        let a: Algebraic<f32> = Algebraic::from(13.0_f32);
        let result = a % 5.0_f32;
        assert_eq!(3.0_f32, result.into());
    }

    #[test]
    fn test_add_f128_primitive() {
        let a: Algebraic<f128> = Algebraic::from(10.0_f128);
        let result = a + 5.0_f128;
        assert_eq!(15.0_f128, result.into());
    }

    #[test]
    fn test_sub_f128_primitive() {
        let a: Algebraic<f128> = Algebraic::from(10.0_f128);
        let result = a - 5.0_f128;
        assert_eq!(5.0_f128, result.into());
    }

    #[test]
    fn test_mul_f128_primitive() {
        let a: Algebraic<f128> = Algebraic::from(10.0_f128);
        let result = a * 5.0_f128;
        assert_eq!(50.0_f128, result.into());
    }

    #[test]
    fn test_div_f128_primitive() {
        let a: Algebraic<f128> = Algebraic::from(10.0_f128);
        let result = a / 5.0_f128;
        assert_eq!(2.0_f128, result.into());
    }

    #[test]
    fn test_rem_f128_primitive() {
        let a: Algebraic<f128> = Algebraic::from(13.0_f128);
        let result = a % 5.0_f128;
        assert_eq!(3.0_f128, result.into());
    }

    #[test]
    fn test_all_primitive_types_operations() {
        // Test f16
        let a16 = Algebraic::from(20.0_f16);
        assert_eq!(25.0_f16, (a16 + 5.0_f16).into());
        assert_eq!(15.0_f16, (a16 - 5.0_f16).into());
        assert_eq!(100.0_f16, (a16 * 5.0_f16).into());
        assert_eq!(4.0_f16, (a16 / 5.0_f16).into());

        // Test f32
        let a32 = Algebraic::from(20.0_f32);
        assert_eq!(25.0_f32, (a32 + 5.0_f32).into());
        assert_eq!(15.0_f32, (a32 - 5.0_f32).into());
        assert_eq!(100.0_f32, (a32 * 5.0_f32).into());
        assert_eq!(4.0_f32, (a32 / 5.0_f32).into());

        // Test f64
        let a64 = Algebraic::from(20.0_f64);
        assert_eq!(25.0_f64, (a64 + 5.0_f64).into());
        assert_eq!(15.0_f64, (a64 - 5.0_f64).into());
        assert_eq!(100.0_f64, (a64 * 5.0_f64).into());
        assert_eq!(4.0_f64, (a64 / 5.0_f64).into());

        // Test f128
        let a128 = Algebraic::from(20.0_f128);
        assert_eq!(25.0_f128, (a128 + 5.0_f128).into());
        assert_eq!(15.0_f128, (a128 - 5.0_f128).into());
        assert_eq!(100.0_f128, (a128 * 5.0_f128).into());
        assert_eq!(4.0_f128, (a128 / 5.0_f128).into());
    }
}
