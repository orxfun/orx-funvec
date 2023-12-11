use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use orx_funvec::{FunVecD1, ScalarAsVec};
use std::ops::Range;

// data
fn get_vec(n: usize) -> Vec<i32> {
    (0..n).map(|_| 42).collect()
}

// variants
fn use_scalar(range: Range<usize>, value: i32) -> i32 {
    let mut sum = 0;
    for _ in range {
        sum += value;
    }
    sum
}
fn use_slice(vec: &[i32]) -> i32 {
    vec.iter().sum()
}
fn use_funvec_at<F: FunVecD1<i32>>(range: Range<usize>, vec: &F) -> i32 {
    let mut sum = 0;
    for i in range {
        sum += vec.at(i).unwrap();
    }
    sum
}
fn use_funvec_iter<F: FunVecD1<i32>>(range: Range<usize>, vec: &F) -> i32 {
    vec.iter_over(range).flatten().sum()
}

fn bench_scalar_optimization(c: &mut Criterion) {
    let treatments = vec![1_000_000];

    let mut group = c.benchmark_group("bench_scalar_optimization");

    for n in &treatments {
        group.bench_with_input(BenchmarkId::new("use_scalar", n), n, |b, n| {
            b.iter(|| use_scalar(black_box(0..*n), 42))
        });

        group.bench_with_input(BenchmarkId::new("use_slice", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                use_slice(&vec)
            })
        });

        group.bench_with_input(BenchmarkId::new("use_funvec_at", n), n, |b, n| {
            b.iter(|| {
                let vec = ScalarAsVec(42);
                use_funvec_at(black_box(0..*n), &vec)
            })
        });

        group.bench_with_input(BenchmarkId::new("use_funvec_iter", n), n, |b, n| {
            b.iter(|| {
                let vec = ScalarAsVec(42);
                use_funvec_iter(black_box(0..*n), &vec)
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_scalar_optimization);
criterion_main!(benches);
