use anyhow::{Result, anyhow};

pub struct Matrix {
    data: Vec<f64>,
    row_count: usize,
    col_count: usize,
}

impl Matrix {
    pub fn new(rows: Vec<Vec<f64>>) -> Result<Matrix> {
        let row_count = rows.len();
        if row_count == 0 {
            return Err(anyhow!("empty matrix"));
        }
        let col_count = rows[0].len();
        if rows.iter().any(|r| r.len() != col_count) {
            return Err(anyhow!("non-square matrix"));
        }
        Ok(Matrix {
            data: rows.into_iter().flatten().collect(),
            row_count,
            col_count,
        })
    }
}
