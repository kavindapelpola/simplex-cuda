use anyhow::{Result, anyhow};
use cust::memory::{DeviceCopy, mem_get_info};
use cust::prelude::*;
use std::mem;
use tracing::{info, warn};

use crate::model::flat_matrix::FlatMatrix;

// matrix with the constraints with variables and
// slack, as well as the second last column as the
// coefficient of the objective and last column as the rhs.
// The objective function is the last row
// for example:
//   x1   x2  s1  s2   P  rhs
// [ 3.,  5., 1., 0., 0., 78.],  constraint 1
// [ 4.,  1., 0., 1., 0., 36.],  constraint 3
// [-5., -4., 0., 0., 1.,  0.],  objective

// variables + slack + rhs
// // number of constraints + objective function
pub type Matrixf32 = Matrix<f32>;

pub struct Matrix<T: Clone + DeviceCopy> {
    cpu_matrix: FlatMatrix<T>,
    gpu_data: DeviceBuffer<T>,
}

impl<T: Clone + DeviceCopy> Matrix<T> {
    pub fn new(rows: &Vec<Vec<T>>, stream: &Stream) -> Result<Matrix<T>> {
        let (avail, total) = mem_get_info()?;
        info!("gpu memory: total {}, avail {}", total, avail);

        // matrix must fit in gpu memory
        if mem::size_of_val(rows) > avail {
            warn!(
                "insufficient gpu memory, require {}",
                mem::size_of_val(rows)
            );
            return Err(anyhow!("insufficient gpu memory"));
        }

        let cpu_matrix = FlatMatrix::new(rows)?;
        let gpu_data = cpu_matrix.data.as_slice().as_dbuf()?;
        stream.synchronize()?;
        info!("gpu memory synchronized");

        Ok(Matrix {
            cpu_matrix,
            gpu_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ctx = cust::quick_init();
        let stream = Stream::new(StreamFlags::NON_BLOCKING, None).unwrap();

        let x = Matrixf32::new(&vec![], &stream);
        assert!(false);
    }
}
