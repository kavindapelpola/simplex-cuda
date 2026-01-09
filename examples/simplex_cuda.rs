use anyhow::Result;
use cust::prelude::*;
use simplex_cuda::model::cuda::Matrixf32;
use tracing_subscriber;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let _ctx = cust::quick_init()?;
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None).unwrap();

    let matrix = Matrixf32::new(
        &vec![
            vec![3., 5., 1., 0., 0., 78.],  // constraint 1
            vec![4., 1., 0., 1., 0., 36.],  // constraint 3
            vec![-5., -4., 0., 0., 1., 0.], // objective
        ],
        &stream,
    )?;

    Ok(())
}
