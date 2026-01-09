use criterion::{Criterion, criterion_group, criterion_main};
use cust::prelude::*;
use simplex_cuda::model::{cpu::CpuMatrix, cuda::Matrixf32};

fn matrix_create(c: &mut Criterion) {
    let _ctx = cust::quick_init();
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None).unwrap();

    let small = vec![vec![0., 0.], vec![0., 0.]];
    let large = vec![vec![0.; 10_000]; 10_000];

    c.bench_function("small cpu matrix create", |b| {
        b.iter(|| CpuMatrix::<f32>::new(&small))
    });

    c.bench_function("small cuda matrix create", |b| {
        b.iter(|| Matrixf32::new(&small, &stream))
    });

    c.bench_function("large cpu cpu matrix create", |b| {
        b.iter(|| CpuMatrix::<f32>::new(&large))
    });

    c.bench_function("large cuda cpu matrix create", |b| {
        b.iter(|| Matrixf32::new(&large, &stream))
    });
}

criterion_group!(benches, matrix_create);
criterion_main!(benches);
