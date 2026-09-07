# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/tormeh/algebraic/compare/0.1.2...HEAD
[0.1.2]: https://github.com/tormeh/algebraic/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/tormeh/algebraic/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/tormeh/algebraic/releases/tag/0.1.0
