use anyhow::Result;
use simplex_cuda::{Problem, problem::Constraint};
use tracing_subscriber;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut p = Problem::maximize(&vec![5., 4.])?
        .with(Constraint::Lt(vec![3., 5., 78.]))?
        .with(Constraint::Lt(vec![4., 1., 36.]))?
        .build()?;

    simplex_cuda::solvers::cpu::solve(&mut p, None)?;

    println!("{}", p);
    println!("{:?}", p.extract_result());

    Ok(())
}
