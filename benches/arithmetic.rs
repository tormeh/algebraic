//! Benchmarks comparing `Algebraic<T>` slice-oriented workloads against their
//! standard floating-point equivalents, for both `f32`/`af32` and `f64`/`af64`.
//!
//! The benefit of `Algebraic<T>` comes entirely from allowing LLVM to reorder and
//! auto-vectorize loops; a single scalar operation (`af64 + af64` vs `f64 + f64`) will not
//! show a meaningful difference. These benchmarks therefore operate on slices, which is the
//! shape of workload where auto-vectorization actually has something to exploit.
//!
//! Whether vectorization actually happens, and how much it helps, depends heavily on the
//! target CPU's available SIMD instruction set. The baseline `x86-64` target (SSE2 only) is a
//! poor demonstration of this crate's value; compiling for a wider instruction set makes the
//! difference far more visible. Run these benchmarks with:
//!
//! ```sh
//! RUSTFLAGS="-C target-cpu=x86-64-v3" cargo bench
//! ```
//!
//! (or `just bench-v3`), which targets the widely available `x86-64-v3` microarchitecture
//! level (AVX2, FMA, BMI1/2, ...). On other architectures, substitute an appropriate
//! `target-cpu`/`target-feature` value.
//!
//! Wall-clock numbers should still be treated as a smoke test rather than a proof of
//! vectorization; pair them with an assembly inspection (see the `algebraic_test` crate) when
//! you need certainty about codegen.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "benchmark input sizes and synthetic data are small enough that lossy casts are harmless"
)]

use algebraic::{Algebraic, AlgebraicFloatTrait};
use criterion::measurement::WallTime;
use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};
use std::hint::black_box;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

const SIZES: [usize; 3] = [64, 1_024, 65_536];

/// The primitive float types exercised by these benchmarks (`f32` and `f64`).
///
/// This exists purely to let the benchmark bodies be written once and instantiated for both
/// types, rather than duplicating every function.
trait BenchFloat:
    AlgebraicFloatTrait + Add<Output = Self> + Mul<Output = Self> + Sum<Self> + Product<Self>
{
    /// Display name used in benchmark IDs (e.g. `"f32"`).
    const NAME: &'static str;
    /// A value slightly above `1.0`, used to generate bounded product-benchmark data.
    const NEAR_ONE_UP: Self;
    /// The reciprocal of [`Self::NEAR_ONE_UP`].
    const NEAR_ONE_DOWN: Self;

    /// Converts an index into a data point used for sum/dot/elementwise-add benchmarks.
    fn from_index(i: usize) -> Self;

    /// The sine of `self`, used to generate bounded, non-degenerate benchmark data.
    fn sin(self) -> Self;
}

impl BenchFloat for f32 {
    const NAME: &'static str = "f32";
    const NEAR_ONE_UP: Self = 1.001;
    const NEAR_ONE_DOWN: Self = 1.0 / Self::NEAR_ONE_UP;

    fn from_index(i: usize) -> Self {
        i as Self
    }

    fn sin(self) -> Self {
        Self::sin(self)
    }
}

impl BenchFloat for f64 {
    const NAME: &'static str = "f64";
    const NEAR_ONE_UP: Self = 1.001;
    const NEAR_ONE_DOWN: Self = 1.0 / Self::NEAR_ONE_UP;

    fn from_index(i: usize) -> Self {
        i as Self
    }

    fn sin(self) -> Self {
        Self::sin(self)
    }
}

/// Generates bounded, non-degenerate synthetic data for sum/dot/elementwise benchmarks.
fn make_data<F: BenchFloat>(size: usize) -> Vec<F> {
    (0..size).map(|i| F::from_index(i).sin()).collect()
}

/// Generates data for the product benchmark that stays close to `1.0` regardless of `size`,
/// avoiding overflow, underflow, and subnormal-related slowdowns.
fn make_product_data<F: BenchFloat>(size: usize) -> Vec<F> {
    (0..size)
        .map(|i| {
            if i % 2 == 0 {
                F::NEAR_ONE_UP
            } else {
                F::NEAR_ONE_DOWN
            }
        })
        .collect()
}

fn to_algebraic<F: BenchFloat>(data: &[F]) -> Vec<Algebraic<F>> {
    data.iter().copied().map(Algebraic::new).collect()
}

fn sum_regular<F: BenchFloat>(data: &[F]) -> F {
    data.iter().copied().sum()
}

fn sum_algebraic<F: BenchFloat>(data: &[Algebraic<F>]) -> Algebraic<F> {
    data.iter().copied().sum()
}

fn product_regular<F: BenchFloat>(data: &[F]) -> F {
    data.iter().copied().product()
}

fn product_algebraic<F: BenchFloat>(data: &[Algebraic<F>]) -> Algebraic<F> {
    data.iter().copied().product()
}

fn dot_regular<F: BenchFloat>(a: &[F], b: &[F]) -> F {
    a.iter().zip(b).map(|(x, y)| *x * *y).sum()
}

fn dot_algebraic<F: BenchFloat>(a: &[Algebraic<F>], b: &[Algebraic<F>]) -> Algebraic<F> {
    a.iter().zip(b).map(|(x, y)| *x * *y).sum()
}

fn elementwise_add_regular<F: BenchFloat>(a: &[F], b: &[F], out: &mut [F]) {
    for (o, (x, y)) in out.iter_mut().zip(a.iter().zip(b)) {
        *o = *x + *y;
    }
}

fn elementwise_add_algebraic<F: BenchFloat>(
    a: &[Algebraic<F>],
    b: &[Algebraic<F>],
    out: &mut [Algebraic<F>],
) {
    for (o, (x, y)) in out.iter_mut().zip(a.iter().zip(b)) {
        *o = *x + *y;
    }
}

fn bench_sum_for<F: BenchFloat>(group: &mut BenchmarkGroup<'_, WallTime>) {
    for size in SIZES {
        let data = make_data::<F>(size);
        let alg_data = to_algebraic(&data);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new(F::NAME, size), &data, |b, data| {
            b.iter(|| sum_regular::<F>(black_box(data)));
        });
        group.bench_with_input(
            BenchmarkId::new(format!("a{}", F::NAME), size),
            &alg_data,
            |b, data| {
                b.iter(|| sum_algebraic::<F>(black_box(data)));
            },
        );
    }
}

fn bench_product_for<F: BenchFloat>(group: &mut BenchmarkGroup<'_, WallTime>) {
    for size in SIZES {
        let data = make_product_data::<F>(size);
        let alg_data = to_algebraic(&data);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new(F::NAME, size), &data, |b, data| {
            b.iter(|| product_regular::<F>(black_box(data)));
        });
        group.bench_with_input(
            BenchmarkId::new(format!("a{}", F::NAME), size),
            &alg_data,
            |b, data| {
                b.iter(|| product_algebraic::<F>(black_box(data)));
            },
        );
    }
}

fn bench_dot_for<F: BenchFloat>(group: &mut BenchmarkGroup<'_, WallTime>) {
    for size in SIZES {
        let a = make_data::<F>(size);
        let b_data = make_data::<F>(size);
        let alg_a = to_algebraic(&a);
        let alg_b = to_algebraic(&b_data);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new(F::NAME, size), &size, |bencher, _| {
            bencher.iter(|| dot_regular::<F>(black_box(&a), black_box(&b_data)));
        });
        group.bench_with_input(
            BenchmarkId::new(format!("a{}", F::NAME), size),
            &size,
            |bencher, _| {
                bencher.iter(|| dot_algebraic::<F>(black_box(&alg_a), black_box(&alg_b)));
            },
        );
    }
}

fn bench_elementwise_add_for<F: BenchFloat>(group: &mut BenchmarkGroup<'_, WallTime>) {
    for size in SIZES {
        let a = make_data::<F>(size);
        let b_data = make_data::<F>(size);
        let alg_a = to_algebraic(&a);
        let alg_b = to_algebraic(&b_data);
        let mut out = vec![F::zero(); size];
        let mut alg_out = vec![Algebraic::<F>::zero(); size];

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new(F::NAME, size), &size, |bencher, _| {
            bencher.iter(|| {
                elementwise_add_regular::<F>(black_box(&a), black_box(&b_data), &mut out);
                black_box(&out);
            });
        });
        group.bench_with_input(
            BenchmarkId::new(format!("a{}", F::NAME), size),
            &size,
            |bencher, _| {
                bencher.iter(|| {
                    elementwise_add_algebraic::<F>(
                        black_box(&alg_a),
                        black_box(&alg_b),
                        &mut alg_out,
                    );
                    black_box(&alg_out);
                });
            },
        );
    }
}

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    bench_sum_for::<f32>(&mut group);
    bench_sum_for::<f64>(&mut group);
    group.finish();
}

fn bench_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("product");
    bench_product_for::<f32>(&mut group);
    bench_product_for::<f64>(&mut group);
    group.finish();
}

fn bench_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_product");
    bench_dot_for::<f32>(&mut group);
    bench_dot_for::<f64>(&mut group);
    group.finish();
}

fn bench_elementwise_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("elementwise_add");
    bench_elementwise_add_for::<f32>(&mut group);
    bench_elementwise_add_for::<f64>(&mut group);
    group.finish();
}

criterion_group!(
    benches,
    bench_sum,
    bench_product,
    bench_dot,
    bench_elementwise_add
);
criterion_main!(benches);
