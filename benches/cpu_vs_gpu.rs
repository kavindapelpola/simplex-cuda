use criterion::{Criterion, criterion_group, criterion_main};
use cust::prelude::*;
use simplex_cuda::model::{cuda::Matrixf32, flat_matrix::FlatMatrix};

fn matrix_create(c: &mut Criterion) {
    let _ctx = cust::quick_init();
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None).unwrap();

    let large = vec![vec![0.; 10_000]; 10_000];

    c.bench_function("10k x 10k flat matrix create", |b| {
        b.iter(|| FlatMatrix::new(&large))
    });

    c.bench_function("10k x 10k cuda matrix create", |b| {
        b.iter(|| Matrixf32::new(&large, &stream))
    });
}

criterion_group!(benches, matrix_create);
criterion_main!(benches);
