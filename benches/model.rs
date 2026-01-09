use criterion::{Criterion, criterion_group, criterion_main};
use simplex_cuda::model::matrix::Matrix;

fn matrix_create(c: &mut Criterion) {
    let small = vec![vec![0., 0.], vec![0., 0.]];
    let large = vec![vec![0.0f64; 10_000]; 10_000];

    c.bench_function("small matrix create", |b| b.iter(|| Matrix::new(small)));

    c.bench_function("large matrix create", |b| b.iter(|| Matrix::new(large)));
}

criterion_group!(benches, matrix_create);
criterion_main!(benches);
