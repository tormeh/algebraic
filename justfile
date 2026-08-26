# Type-check stable and nightly (with f16/f128 features)
check-types:
    cargo check
    cargo +nightly check --features "f16,f128"

# Lint stable and nightly (with f16/f128 features)
lint:
    cargo clippy
    cargo +nightly clippy --features "f16,f128"

# Apply clippy fixes for stable and nightly (with f16/f128 features)
fix:
    cargo clippy --fix
    cargo +nightly clippy --fix --features "f16,f128"

# Run tests for stable and nightly (with f16/f128 features)
test:
    cargo test
    cargo +nightly test --features "f16,f128"

# Run check-types, lint, and test (full validation)
check: check-types lint test

# Open docs in the browser
docs:
    cargo +nightly doc --all-features --open
