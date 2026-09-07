# Type-check stable and nightly (with f16/f128 features)
check-types:
    cargo check
    cargo +nightly check --features "f16,f128"

# Lint stable and nightly (with f16/f128 features)
lint:
    cargo clippy --all-targets
    cargo +nightly clippy --features "f16,f128" --all-targets
    rumdl check --disable MD013

# Apply clippy fixes for stable and nightly (with f16/f128 features)
fix:
    cargo clippy --fix
    cargo +nightly clippy --fix --features "f16,f128"

# Run tests for stable and nightly (with f16/f128 features)
test:
    cargo test
    cargo +nightly test --features "f16,f128"
    cargo test --doc

# Run benchmarks (stable types only; f16/f128 require nightly and are not benchmarked here)
bench:
    cargo bench

# Run benchmarks compiled for the x86-64-v3 microarchitecture level (AVX2, FMA, BMI1/2, ...).
# This is the recommended way to evaluate Algebraic<T>, since the auto-vectorization it
# enables is barely visible at the x86-64 baseline (SSE2 only). Requires an x86_64 host.
bench-v3:
    RUSTFLAGS="-C target-cpu=x86-64-v3" cargo bench

# Run check-types, lint, and test (full validation)
check: check-types lint test

# Open docs in the browser
docs:
    cargo +nightly doc --all-features --open
