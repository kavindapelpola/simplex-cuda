use anyhow::{Result, anyhow};

pub struct FlatMatrix<T: Clone> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T: Clone> FlatMatrix<T> {
    pub fn new(data: &Vec<Vec<T>>) -> Result<FlatMatrix<T>> {
        // matrix cannot be empty
        let rows = data.len();
        if rows < 1 {
            return Err(anyhow!("matrix must contain at least one row"));
        }

        // matrix must be square
        let cols = data[0].len();
        if data.iter().any(|r| r.len() != cols) {
            return Err(anyhow!("matrix must be square"));
        }

        Ok(FlatMatrix {
            rows,
            cols,
            data: data.iter().flatten().cloned().collect(),
        })
    }

    pub fn row(&self, index: usize) -> Result<&[T]> {
        if index >= self.rows {
            return Err(anyhow!("row index {} out of bounds", index));
        }

        let start = index * self.cols;
        let end = start + self.cols;
        Ok(&self.data[start..end])
    }

    pub fn row_mut(&mut self, index: usize) -> Result<&mut [T]> {
        if index >= self.rows {
            return Err(anyhow!("row index {} out of bounds", index));
        }

        let start = index * self.cols;
        let end = start + self.cols;
        Ok(&mut self.data[start..end])
    }

    pub fn last_row(&self) -> &[T] {
        self.row(self.rows - 1).unwrap()
    }

    pub fn get(&self, row: usize, col: usize) -> Result<&T> {
        if row >= self.rows {
            return Err(anyhow!("row index {} out of bounds", row));
        }
        if col >= self.cols {
            return Err(anyhow!("column index {} out of bounds", col));
        }

        Ok(&self.data[row * self.cols + col])
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Result<&mut T> {
        if row >= self.rows {
            return Err(anyhow!("row index {} out of bounds", row));
        }
        if col >= self.cols {
            return Err(anyhow!("column index {} out of bounds", col));
        }

        Ok(&mut self.data[row * self.cols + col])
    }

    pub fn col(&self, index: usize) -> Result<Vec<&T>> {
        if index >= self.cols {
            return Err(anyhow!("column index {} out of bounds", index));
        }

        Ok((0..self.rows)
            .map(|row| &self.data[row * self.cols + index])
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> Vec<Vec<f32>> {
        vec![
            vec![3., 5., 1., 0., 0., 78.],
            vec![4., 1., 0., 1., 0., 36.],
            vec![-5., -4., 0., 0., 1., 0.],
        ]
    }

    #[test]
    fn flatmatrix_new_empty_returns_error() {
        assert!(FlatMatrix::<f32>::new(&vec![]).is_err());
    }

    #[test]
    fn flatmatrix_new_non_square_returns_error() {
        assert!(FlatMatrix::<f32>::new(&vec![vec![0.], vec![0., 0.]]).is_err());
    }

    #[test]
    fn flatmatrix_new_square_returns_ok() {
        let result = FlatMatrix::new(&valid());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.rows, 3);
        assert_eq!(result.cols, 6);
    }

    #[test]
    fn last_row_returns_last_row() {
        assert_eq!(
            FlatMatrix::new(&valid()).unwrap().last_row(),
            [-5., -4., 0., 0., 1., 0.]
        )
    }

    #[test]
    fn row_returns_row_values() {
        let matrix = FlatMatrix::new(&valid()).unwrap();

        // Test first row
        assert_eq!(matrix.row(0).unwrap(), &[3., 5., 1., 0., 0., 78.]);

        // Test second row
        assert_eq!(matrix.row(1).unwrap(), &[4., 1., 0., 1., 0., 36.]);

        // Test last row (objective function)
        assert_eq!(matrix.row(2).unwrap(), &[-5., -4., 0., 0., 1., 0.]);
    }

    #[test]
    fn row_out_of_bounds_returns_error() {
        let matrix = FlatMatrix::new(&valid()).unwrap();
        assert!(matrix.row(3).is_err());
    }

    #[test]
    fn row_mut_modifies_row() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();

        // Modify the first row
        let row = matrix.row_mut(0).unwrap();
        row[0] = 99.;
        row[1] = 88.;

        // Verify the modifications
        assert_eq!(matrix.row(0).unwrap(), &[99., 88., 1., 0., 0., 78.]);
    }

    #[test]
    fn row_mut_out_of_bounds_returns_error() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();
        assert!(matrix.row_mut(3).is_err());
    }

    #[test]
    fn get_returns_value_at_position() {
        let matrix = FlatMatrix::new(&valid()).unwrap();

        // Test various positions
        assert_eq!(*matrix.get(0, 0).unwrap(), 3.);
        assert_eq!(*matrix.get(0, 1).unwrap(), 5.);
        assert_eq!(*matrix.get(1, 0).unwrap(), 4.);
        assert_eq!(*matrix.get(2, 5).unwrap(), 0.);
    }

    #[test]
    fn get_out_of_bounds_returns_error() {
        let matrix = FlatMatrix::new(&valid()).unwrap();
        assert!(matrix.get(3, 0).is_err());
        assert!(matrix.get(0, 6).is_err());
    }

    #[test]
    fn get_mut_modifies_value_at_position() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();

        // Modify a value
        *matrix.get_mut(1, 1).unwrap() = 99.;

        // Verify the modification
        assert_eq!(*matrix.get(1, 1).unwrap(), 99.);
        assert_eq!(matrix.row(1).unwrap(), &[4., 99., 0., 1., 0., 36.]);
    }

    #[test]
    fn get_mut_out_of_bounds_returns_error() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();
        assert!(matrix.get_mut(3, 0).is_err());
        assert!(matrix.get_mut(0, 6).is_err());
    }

    #[test]
    fn col_returns_column_values() {
        let matrix = FlatMatrix::new(&valid()).unwrap();

        // Test first column (x1)
        assert_eq!(matrix.col(0).unwrap(), vec![&3., &4., &-5.]);

        // Test second column (x2)
        assert_eq!(matrix.col(1).unwrap(), vec![&5., &1., &-4.]);

        // Test last column (rhs)
        assert_eq!(matrix.col(5).unwrap(), vec![&78., &36., &0.]);
    }

    #[test]
    fn col_out_of_bounds_returns_error() {
        let matrix = FlatMatrix::new(&valid()).unwrap();
        assert!(matrix.col(6).is_err());
    }
}
