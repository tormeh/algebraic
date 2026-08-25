# Agent Guide: algebraic

This project provides `Algebraic<T>` wrappers for Rust floating-point types to enable algebraic reordering optimizations (fast-math) via Rust's experimental `algebraic` intrinsics.

## Core Concepts

- **`Algebraic<T>`**: The primary wrapper type. It implements standard arithmetic traits (`Add`, `Sub`, `Mul`, `Div`, `Rem`) and their `*Assign` variants, reference-based and mixed-reference variants, mixed-type variants (e.g. `Add<T> for Algebraic<T>`), and iterator traits (`Sum`, `Product`), all using algebraic intrinsics.
- **`AlgebraicFloatTrait`**: The public trait that `Algebraic<T>` is generic over. It defines the algebraic intrinsic methods (`algebraic_add`, `algebraic_sub`, etc.) and is the extension point for supporting new float types.
- **Type Aliases**: Use `af32` and `af64` for convenience. `af16` and `af128` are available behind the `f16` and `f128` feature flags.
- **Intent**: The goal is to allow the compiler to reorder operations for better vectorization while maintaining a safe Rust interface.

## Development Workflows

The project uses `just` for common tasks. Run `just --list` to see all available commands.

The toolchain is pinned to **stable 1.98** via `rust-toolchain.toml`. The `f16` and `f128` features require **nightly** and are tested separately.

**For full validation, run `just check`.** Running `cargo test` alone skips the f16/f128 paths and the linting step.

## Guidelines for Agents

- **Use algebraic intrinsics**: When modifying arithmetic implementations, always use the `algebraic_` variant of the operation (e.g., `algebraic_add` instead of standard `add`) to preserve the crate's purpose.
- **Feature flags**: Always respect `cfg` attributes for `f16` and `f128` — they depend on unstable Rust features and must be compiled with nightly.
- **Strict Clippy lints**: `Cargo.toml` denies `clippy::all`, `clippy::pedantic`, `clippy::nursery`, and `clippy::indexing_slicing`. New code must pass all of these. Run `just lint` before considering a change complete.
- **Adding a new float type**: Implement `AlgebraicFloatTrait` for the new type in `src/impls/`, add `From`/`Into` conversions in `src/conversions.rs`, and add a type alias in `src/types.rs`. Gate everything behind a feature flag if the type is unstable.
- **Adding new math operations**: Check whether an algebraic intrinsic exists in `core::intrinsics` or the standard library's experimental math modules before falling back to a non-algebraic implementation.
