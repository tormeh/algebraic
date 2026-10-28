# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Inherent constants and methods on `af32`, `af64`, `af16` and `af128` mirroring those of the
  primitive float types available in `core`, so they can be used without importing any traits:
  `NAN`, `INFINITY`, `NEG_INFINITY`, `MIN`, `MAX`, `MIN_POSITIVE`, `EPSILON`, `is_nan`,
  `is_infinite`, `is_finite`, `is_subnormal`, `is_normal`, `classify`, `is_sign_positive`,
  `is_sign_negative`, `abs`, `signum`, `copysign`, `recip`, `to_degrees`, `to_radians`, `max`,
  `min`, `clamp`, `midpoint`, `next_up`, `next_down`, `total_cmp`, `to_bits` and `from_bits`.
  `recip` uses algebraic division.
- `FromStr` for `Algebraic<T>`, parsing exactly like `T` (e.g. `"1.5".parse::<af32>()`).
- The missing owned/borrowed combinations of `Algebraic<T>` with a primitive right-hand side:
  `Algebraic<T> op &T`, `&Algebraic<T> op T` and `&Algebraic<T> op &T` for `+`, `-`, `*`, `/`
  and `%`. This makes e.g. `values.iter().map(|x| x * 2.0)` compile.

### Changed

- Removed the redundant `Clone` supertrait from `AlgebraicFloatTrait` (it is implied by `Copy`).

### Fixed

- `Display` for `Algebraic<T>` now honors formatter options such as precision, width and sign
  (e.g. `format!("{:.2}", x)`), formatting exactly like `T`. Previously they were ignored.
- With the `num` feature, `num_traits::Float::epsilon` for `af64` returned `f32::EPSILON`, because
  it fell back to the trait's default implementation. It now returns `f64::EPSILON`. The other
  defaulted `Float` methods (`is_subnormal`, `to_degrees`, `to_radians`, `clamp` and `copysign`)
  now also forward to the wrapped type instead of using the less precise generic defaults.
- With the `num` feature, `num_traits::FloatConst::LOG10_2` and `LOG2_10` were computed as
  `LN_2 / LN_10` (and its inverse) by the trait's default implementations, which is one ulp off
  for `af64` (and for `LOG10_2` on `af32`). They now forward to the wrapped type. The
  `FloatConst` impl now additionally requires `T: Div<Output = T>`, which all primitive floats
  satisfy.

## [0.1.4] - 2026-10-15

### Added

- Optional `num` feature implementing `num-traits` numeric, floating-point, and conversion traits
  for algebraic float wrappers whose inner type provides the corresponding traits.
- `Neg` implementations for `Algebraic<T>` and `&Algebraic<T>`, for inner types that implement `Neg`.
- Reference-based compound assignment operators: `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`
  and `RemAssign` now accept `&Algebraic<T>` and `&T` on the right-hand side.

### Changed

- `PartialOrd` for `Algebraic<T>` is now implemented manually, forwarding `<`, `<=`, `>` and `>=`
  directly to the inner value. This yields better code than the derived implementation, which
  routes comparisons through `partial_cmp` and `Option<Ordering>`. Behavior is unchanged.
- Added `#[inline]` to the arithmetic operators, conversions, accessors and `AlgebraicFloatTrait`
  implementations to improve code generation.

## [0.1.3] - 2026-09-15

### Added

- Optional `serde` support (behind the new `serde` feature flag) with `Serialize` and
  `Deserialize` implementations for `Algebraic<T>`. `Algebraic<T>` serializes and deserializes
  exactly as the wrapped primitive value would, and the implementation is `no_std`-compatible.
- Operator overloads for supported primitive float types on the left-hand side of
  `Algebraic<T>` arithmetic, including assignment variants, so expressions like
  `1.0_f64 + Algebraic::from(2.0_f64)` and `value += Algebraic::from(2.0_f64)` are supported.

## [0.1.2] - 2026-08-26

### Added

- Usage example in the README.

### Changed

- Removed a superfluous documentation link.

## [0.1.1] - 2026-08-26

### Added

- `no_std` support.
- `AsRef<T>` and `AsMut<T>` implementations for `Algebraic<T>`.
- `Default` and `PartialOrd` derives for `Algebraic<T>`.
- `#[repr(transparent)]` on `Algebraic<T>`.
- Usage example in the crate-level documentation.
- Additional crate metadata.

### Changed

- Improved conformance to Rust API interoperability recommendations.

## [0.1.0] - 2026-08-25

### Added

- Initial release.
- `Algebraic<T>` wrapper type implementing `Add`, `Sub`, `Mul`, `Div`, and `Rem`, along with
  their `*Assign`, reference-based, mixed-reference, and mixed-type variants, using algebraic
  (fast-math) intrinsics.
- `AlgebraicFloatTrait` trait, the extension point for supporting new float types.
- `Sum` and `Product` iterator trait implementations.
- `af32` and `af64` type aliases, plus `af16` and `af128` behind the `f16` and `f128` feature
  flags respectively.
- `Display` implementation for `Algebraic<T>`.

[Unreleased]: https://github.com/tormeh/algebraic/compare/0.1.4...HEAD
[0.1.4]: https://github.com/tormeh/algebraic/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/tormeh/algebraic/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/tormeh/algebraic/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/tormeh/algebraic/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/tormeh/algebraic/releases/tag/0.1.0
