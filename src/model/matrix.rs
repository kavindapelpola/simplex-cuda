use anyhow::{Result, anyhow};

pub type Matrixf32 = Matrix<f32>;

pub struct Matrix<T: Clone> {
    data: Vec<T>,
    row_count: usize,
    col_count: usize,
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: &Vec<Vec<T>>) -> Result<Matrix<T>> {
        let row_count = rows.len();
        if row_count == 0 {
            return Err(anyhow!("empty matrix"));
        }
        let col_count = rows[0].len();
        if rows.iter().any(|r| r.len() != col_count) {
            return Err(anyhow!("non-square matrix"));
        }
        Ok(Matrix {
            data: rows.iter().flatten().cloned().collect(),
            row_count,
            col_count,
        })
    }
}
