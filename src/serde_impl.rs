//! `serde` support for [`Algebraic<T>`], enabled via the **`serde`** crate feature.
//!
//! `Algebraic<T>` serializes and deserializes exactly as its wrapped primitive floating-point
//! value would, i.e. as a plain number rather than a struct with a `value` field. This keeps
//! the wire format identical to `T`, so switching between `T` and `Algebraic<T>` in a data
//! structure is not a breaking change for serialization purposes.
//!
//! This implementation depends only on `serde`'s `core`-only functionality, so it works in
//! `no_std` environments without requiring `alloc` or `std`.

use crate::algebraic::Algebraic;
use crate::traits::AlgebraicFloatTrait;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<T> Serialize for Algebraic<T>
where
    T: AlgebraicFloatTrait + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Algebraic<T>
where
    T: AlgebraicFloatTrait + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::new)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::Algebraic;
    use serde_test::{Token, assert_ser_tokens, assert_tokens};

    #[test]
    fn test_serde_round_trip_f32() {
        let value: Algebraic<f32> = Algebraic::new(1.5);
        assert_tokens(&value, &[Token::F32(1.5)]);
    }

    #[test]
    fn test_serde_round_trip_f64() {
        let value: Algebraic<f64> = Algebraic::new(3.25);
        assert_tokens(&value, &[Token::F64(3.25)]);
    }

    #[test]
    fn test_serde_matches_primitive_representation() {
        // `Algebraic<T>` must serialize identically to the wrapped primitive, i.e. as a bare
        // number rather than as a struct with a `value` field.
        let wrapped: Algebraic<f64> = Algebraic::new(42.0);
        assert_ser_tokens(&wrapped, &[Token::F64(42.0)]);
    }
}
