use anyhow::{Result, anyhow};
use tracing::warn;

pub struct CpuMatrix<T: Clone> {
    pub data: Vec<T>,
    row_count: usize,
    col_count: usize,
}

impl<T: Clone> CpuMatrix<T> {
    pub fn new(rows: &Vec<Vec<T>>) -> Result<CpuMatrix<T>> {
        // matrix cannot be empty
        let row_count = rows.len();
        if row_count == 0 {
            warn!("empty matrix");
            return Err(anyhow!("empty matrix"));
        }

        // matrix must be square
        let col_count = rows[0].len();
        if rows.iter().any(|r| r.len() != col_count) {
            warn!(
                "non-square matrix, row {:?}",
                rows.iter()
                    .enumerate()
                    .filter(|(_, r)| r.len() != col_count)
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>()
            );
            return Err(anyhow!("non-square matrix"));
        }

        Ok(CpuMatrix {
            data: rows.iter().flatten().cloned().collect(),
            row_count,
            col_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpumatrix_new_empty_returns_error() {
        assert!(CpuMatrix::<f32>::new(&vec![]).is_err());
    }

    #[test]
    fn cpumatrix_new_non_square_returns_error() {
        assert!(CpuMatrix::<f32>::new(&vec![vec![0.], vec![0., 0.]]).is_err());
    }

    #[test]
    fn cpumatrix_new_square_returns_ok() {
        let result = CpuMatrix::<f32>::new(&vec![vec![0., 0.], vec![0., 0.]]);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.row_count, 2);
        assert_eq!(result.col_count, 2);
    }
}
