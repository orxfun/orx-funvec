use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use orx_funvec::FunVecRefD1;

// data
fn get_vec(n: usize) -> Vec<usize> {
    (0..n).map(|i| 2 * i + 3).collect()
}

// variants
fn use_slice(slice: &[usize]) -> usize {
    let mut sum = 0;
    for x in slice {
        sum += x;
    }
    sum
}
fn use_slice_get(n: usize, slice: &[usize]) -> usize {
    let mut sum = 0;
    for i in 0..n {
        sum += slice.get(i).unwrap();
    }
    sum
}
fn use_funvec<F: FunVecRefD1<usize>>(n: usize, vec: &F) -> usize {
    let mut sum = 0;
    for i in 0..n {
        sum += vec.ref_at(i).unwrap();
    }
    sum
}

fn bench_funvec_vec(c: &mut Criterion) {
    let treatments = vec![1_000_000];

    let mut group = c.benchmark_group("funvec_d1_vec_at");

    for n in &treatments {
        group.bench_with_input(BenchmarkId::new("use_slice", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                use_slice(black_box(&vec))
            })
        });

        group.bench_with_input(BenchmarkId::new("use_slice_get", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                use_slice_get(*n, black_box(&vec))
            })
        });

        group.bench_with_input(BenchmarkId::new("use_funvec", n), n, |b, n| {
            b.iter(|| {
                let vec = get_vec(*n);
                use_funvec(*n, black_box(&vec))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_funvec_vec);
criterion_main!(benches);
