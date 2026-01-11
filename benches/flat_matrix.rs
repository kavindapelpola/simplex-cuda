use criterion::{Criterion, criterion_group, criterion_main};
use simplex_cuda::model::flat_matrix::FlatMatrix;
use std::hint::black_box;

fn create(c: &mut Criterion) {
    let small = vec![vec![0., 0.], vec![0., 0.]];
    let large = vec![vec![0.; 10_000]; 10_000];

    c.bench_function("2x2 flat matrix create", |b| {
        b.iter(|| FlatMatrix::new(&small))
    });

    c.bench_function("10k x 10k flat matrix create", |b| {
        b.iter(|| FlatMatrix::new(&large))
    });
}

fn get(c: &mut Criterion) {
    let large = vec![vec![0.; 10_000]; 10_000];
    let matrix = FlatMatrix::new(&large).unwrap();

    c.bench_function("10k x 10k flat matrix get", |b| {
        let mut i = 0;
        b.iter(|| {
            // Vary access pattern: use counter to access different cells
            // Modulo ensures we stay in bounds
            let row = i % 10_000;
            let col = (i * 7) % 10_000; // Multiply by prime to avoid patterns
            i += 1;

            // black_box prevents compiler from optimizing away the access
            black_box(matrix.get(row, col))
        })
    });

    c.bench_function("10k x 10k flat matrix get unchecked", |b| {
        let mut i = 0;
        b.iter(|| {
            let row = i % 10_000;
            let col = (i * 7) % 10_000;
            i += 1;

            unsafe { black_box(matrix.get_unchecked(row, col)) }
        })
    });
}

fn row_div_scalar(c: &mut Criterion) {
    let large = vec![vec![1.0; 10_000]; 10_000];
    let mut matrix = FlatMatrix::new(&large).unwrap();

    c.bench_function("10k x 10k flat matrix row_div_scalar", |b| {
        let mut i = 0;
        b.iter(|| {
            // Vary which row we divide to prevent optimization
            let row = i % 10_000;
            i += 1;

            black_box(matrix.row_div_scalar(row, 2.0))
        })
    });
}

fn row_sub_scaled(c: &mut Criterion) {
    let large = vec![vec![1.0; 10_000]; 10_000];
    let mut matrix = FlatMatrix::new(&large).unwrap();

    c.bench_function("10k x 10k flat matrix row_sub_scaled", |b| {
        let mut i = 0;
        b.iter(|| {
            // Vary target and source rows to prevent optimization
            let target_row = i % 10_000;
            let source_row = (i * 7) % 10_000; // Different row using prime multiplier
            i += 1;

            // Ensure rows are different (required by row_sub_scaled)
            if target_row != source_row {
                black_box(matrix.row_sub_scaled(target_row, 2.0, source_row));
            }
        })
    });
}

criterion_group!(benches, create, get, row_div_scalar, row_sub_scaled);
criterion_main!(benches);
