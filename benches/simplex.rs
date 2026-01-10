use criterion::{Criterion, criterion_group, criterion_main};
use simplex_cuda::algorithms::simplex;

fn simplex_solve(c: &mut Criterion) {
    c.bench_function("simplex solve ndarray", |b| {
        b.iter(|| simplex::solve_ndarray())
    });

    c.bench_function("simplex solve flatmatrix", |b| {
        b.iter(|| simplex::solve_flatmatrix())
    });
}

criterion_group!(benches, simplex_solve);
criterion_main!(benches);
