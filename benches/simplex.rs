use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use simplex_cuda::model::flat_matrix::FlatMatrix;

fn simplex_solve_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("simplex_small");

    group.bench_function("flatmatrix", |b| {
        b.iter_batched(
            || create_small_table_flatmatrix(),
            |mut table| simplex_cuda::solvers::cpu::solve(&mut table, None),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn simplex_solve_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("simplex_large");

    // Set longer measurement time for large tables
    group.sample_size(10);

    group.bench_function("flatmatrix", |b| {
        b.iter_batched(
            || create_large_table_flatmatrix(),
            |mut table| simplex_cuda::solvers::cpu::solve(&mut table, None),
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

pub fn create_small_table_flatmatrix() -> FlatMatrix<f32> {
    FlatMatrix::new(&vec![
        vec![3., 5., 1., 0., 0., 78.],
        vec![4., 1., 0., 1., 0., 36.],
        vec![-5., -4., 0., 0., 1., 0.],
    ])
    .unwrap()
}

pub fn create_large_table_flatmatrix() -> FlatMatrix<f32> {
    let n_constraints = 999;
    let n_decision_vars = 500;
    let n_slack_vars = 499;

    let nrows = n_constraints + 1;
    let ncols = n_decision_vars + n_slack_vars + 1;

    let mut data = Vec::with_capacity(nrows);

    // Fill constraint rows
    for i in 0..n_constraints {
        let mut row = vec![0.0_f32; ncols];

        // Decision variable coefficients
        for j in 0..n_decision_vars {
            row[j] = ((i + j * 7) % 10) as f32 * 0.3 + 0.1;
        }

        // Slack variable: identity matrix
        if i < n_slack_vars {
            row[n_decision_vars + i] = 1.0;
        }

        // RHS
        row[ncols - 1] = ((i + 1) * 113 % 500) as f32 + 100.0;

        data.push(row);
    }

    // Objective row
    let mut obj_row = vec![0.0_f32; ncols];
    for j in 0..n_decision_vars {
        obj_row[j] = -((j % 15) as f32 * 0.2 + 0.5);
    }
    obj_row[ncols - 1] = 0.0;
    data.push(obj_row);

    FlatMatrix::new(&data).unwrap()
}

criterion_group!(benches, simplex_solve_small, simplex_solve_large);
criterion_main!(benches);
