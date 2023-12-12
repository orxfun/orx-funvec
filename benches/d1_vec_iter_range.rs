use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use orx_funvec::*;

// data
fn get_vec(n: usize) -> Vec<usize> {
    (0..n).map(|i| 2 * i + 3).collect()
}

// variants
fn use_iter(slice: &[usize]) -> usize {
    slice.iter().sum()
}
fn use_funvec_iterover<F: FunVecRef<1, usize>>(n: usize, vec: &F) -> usize {
    vec.ref_iter_over(0..n).flatten().sum()
}

fn bench_funvec_vec(c: &mut Criterion) {
    let treatments = vec![1_000_000];

    let mut group = c.benchmark_group("funvec_d1_vec_at");

    for n in &treatments {
        group.bench_with_input(BenchmarkId::new("use_iter", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                use_iter(black_box(&vec))
            })
        });

        group.bench_with_input(BenchmarkId::new("use_funvec_iterover", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                use_funvec_iterover(*n, black_box(&vec))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_funvec_vec);
criterion_main!(benches);
