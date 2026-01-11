use anyhow::Result;
use simplex_cuda::model::flat_matrix::FlatMatrix;
use tracing_subscriber;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut table = FlatMatrix::<f32>::new(&vec![
        vec![3., 5., 1., 0., 0., 78.],  // constraint 1
        vec![4., 1., 0., 1., 0., 36.],  // constraint 3
        vec![-5., -4., 0., 0., 1., 0.], // objective
    ])?;

    simplex_cuda::solvers::cpu::solve(&mut table, None)?;

    Ok(())
}
