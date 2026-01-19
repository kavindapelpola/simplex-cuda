use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use simplex_cuda::{Problem, Constraint};

fn simplex_solve_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("simplex_small");

    group.bench_function("problem", |b| {
        b.iter_batched(
            || create_small_problem(),
            |mut problem| simplex_cuda::solvers::cpu::solve(&mut problem, None),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn simplex_solve_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("simplex_large");

    // Set longer measurement time for large tables
    group.sample_size(10);

    group.bench_function("problem", |b| {
        b.iter_batched(
            || create_large_problem(),
            |mut problem| simplex_cuda::solvers::cpu::solve(&mut problem, None),
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

pub fn create_small_problem() -> Problem {
    // Maximize: 5x + 4y
    // Subject to: 3x + 5y <= 78
    //             4x + y <= 36
    Problem::maximize(&vec![5., 4.])
        .unwrap()
        .with(Constraint::Lt(vec![3., 5., 78.]))
        .unwrap()
        .with(Constraint::Lt(vec![4., 1., 36.]))
        .unwrap()
        .build()
        .unwrap()
}

pub fn create_large_problem() -> Problem {
    let n_constraints = 999;
    let n_decision_vars = 500;

    // Build objective function (minimize since the original had positive coefficients in the objective row)
    let mut objective = vec![0.0_f32; n_decision_vars];
    for j in 0..n_decision_vars {
        objective[j] = (j % 15) as f32 * 0.2 + 0.5;
    }

    let mut problem = Problem::minimize(&objective).unwrap();

    // Add constraints
    for i in 0..n_constraints {
        let mut constraint_coeffs = vec![0.0_f32; n_decision_vars];

        // Decision variable coefficients
        for j in 0..n_decision_vars {
            constraint_coeffs[j] = ((i + j * 7) % 10) as f32 * 0.3 + 0.1;
        }

        // RHS
        let rhs = ((i + 1) * 113 % 500) as f32 + 100.0;
        constraint_coeffs.push(rhs);

        // Add as less-than constraint (since the original had slack variables)
        problem = problem.with(Constraint::Lt(constraint_coeffs)).unwrap();
    }

    problem.build().unwrap()
}

criterion_group!(benches, simplex_solve_small, simplex_solve_large);
criterion_main!(benches);
