use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use orx_funvec::FunVecD1Ref;

// data
fn get_vec(n: usize) -> Vec<usize> {
    (0..n).map(|i| 2 * i + 3).collect()
}
fn get_indices(n: usize) -> Vec<usize> {
    (0..n)
        .filter(|x| x % 2 == 0)
        .chain((0..n).filter(|x| x % 2 == 1))
        .collect()
}

// variants
fn use_iter(slice: &[usize], indices: &[usize]) -> usize {
    indices.iter().map(|i| &slice[*i]).sum()
}
fn use_funvec_iterover<F: FunVecD1Ref<usize>>(vec: &F, indices: &[usize]) -> usize {
    vec.ref_iter_over(indices.iter().cloned()).flatten().sum()
}

fn bench_funvec_vec(c: &mut Criterion) {
    let treatments = vec![1_000_000];

    let mut group = c.benchmark_group("funvec_d1_vec_at");

    for n in &treatments {
        group.bench_with_input(BenchmarkId::new("use_iter", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                let ind = get_indices(*n);
                use_iter(black_box(&vec), black_box(&ind))
            })
        });

        group.bench_with_input(BenchmarkId::new("use_funvec_iterover", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                let ind = get_indices(*n);
                use_funvec_iterover(black_box(&vec), black_box(&ind))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_funvec_vec);
criterion_main!(benches);
