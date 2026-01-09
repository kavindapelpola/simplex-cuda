use anyhow::{Result, anyhow};
use cust::memory::{DeviceCopy, mem_get_info};
use cust::prelude::*;
use std::mem;
use tracing::{info, warn};

use crate::model::cpu::CpuMatrix;

pub type Matrixf32 = Matrix<f32>;

pub struct Matrix<T: Clone + DeviceCopy> {
    cpu_matrix: CpuMatrix<T>,
    gpu_matrix: DeviceBuffer<T>,
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

        let cpu_matrix = CpuMatrix::new(rows)?;
        let gpu_matrix = cpu_matrix.data.as_slice().as_dbuf()?;
        stream.synchronize()?;
        info!("gpu memory synchronized");

        Ok(Matrix {
            cpu_matrix,
            gpu_matrix,
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
