//! FlatMatrix is a row-major flattened 2D matrix optimized for performance
//! The implementation uses a vec of T to represent a 2D matrix, which can
//! be copied to the gpu
use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct FlatMatrix<T: Copy> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T: Copy> FlatMatrix<T> {
    /// Create a new FlatMatrix from a vec of vecs ensuring that the
    /// vec of vecs is appropriately sized and is square
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

    /// Return a slice for row at index
    pub fn row(&self, index: usize) -> Result<&[T]> {
        if index >= self.rows {
            return Err(anyhow!("row index {} out of bounds", index));
        }

        let start = index * self.cols;
        let end = start + self.cols;
        Ok(&self.data[start..end])
    }

    /// Return a mutable slice for row at index
    pub fn row_mut(&mut self, index: usize) -> Result<&mut [T]> {
        if index >= self.rows {
            return Err(anyhow!("row index {} out of bounds", index));
        }

        let start = index * self.cols;
        let end = start + self.cols;
        Ok(&mut self.data[start..end])
    }

    /// Return a slice for the last row
    pub fn last_row(&self) -> &[T] {
        self.row(self.rows - 1).unwrap()
    }

    /// Return a slice for column at index
    pub fn col(&self, index: usize) -> Result<Vec<&T>> {
        if index >= self.cols {
            return Err(anyhow!("column index {} out of bounds", index));
        }

        Ok((0..self.rows)
            .map(|row| &self.data[row * self.cols + index])
            .collect())
    }

    /// Return the value at position row, col
    pub fn get(&self, row: usize, col: usize) -> Result<&T> {
        if row >= self.rows {
            return Err(anyhow!("row index {} out of bounds", row));
        }
        if col >= self.cols {
            return Err(anyhow!("column index {} out of bounds", col));
        }

        Ok(&self.data[row * self.cols + col])
    }

    /// Return a mutable value at position row, col
    pub fn get_mut(&mut self, row: usize, col: usize) -> Result<&mut T> {
        if row >= self.rows {
            return Err(anyhow!("row index {} out of bounds", row));
        }
        if col >= self.cols {
            return Err(anyhow!("column index {} out of bounds", col));
        }

        Ok(&mut self.data[row * self.cols + col])
    }

    /// Return the value at position row, col
    ///
    /// # Safety
    /// Caller must ensure row < self.rows and col < self.cols.
    /// No bounds checking is performed for performance.
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        unsafe { self.data.get_unchecked(row * self.cols + col) }
    }

    /// Return a mutable value at position row, col
    ///
    /// # Safety
    /// Caller must ensure row < self.rows and col < self.cols.
    /// No bounds checking is performed for performance.
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(row * self.cols + col) }
    }

    /// Divide all elements in a row by a scalar value
    pub fn row_div_scalar(&mut self, row_index: usize, divisor: T) -> Result<()>
    where
        T: std::ops::DivAssign,
    {
        let row = self.row_mut(row_index)?;
        for elem in row.iter_mut() {
            *elem /= divisor;
        }
        Ok(())
    }

    /// Subtract a scaled row from another row: target_row -= factor * source_row
    /// Optimized AXPY-style operation for simplex algorithm.
    ///
    /// # Safety
    /// Uses unsafe code to create non-overlapping row references.
    /// Caller must ensure row_index != source_row_index.
    pub fn row_sub_scaled(&mut self, row_index: usize, factor: T, source_row_index: usize)
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
    {
        let cols = self.cols;
        let data_ptr = self.data.as_mut_ptr();

        unsafe {
            // Create non-overlapping slices (safe because row_index != source_row_index)
            let source = std::slice::from_raw_parts(data_ptr.add(source_row_index * cols), cols);
            let target = std::slice::from_raw_parts_mut(data_ptr.add(row_index * cols), cols);

            // TODO: LLVM does not vectorize this loop. Assembly shows scalar d-register
            // instructions instead of vector z-register instructions.
            for i in 0..cols {
                target[i] = target[i] - factor * source[i];
            }
        }
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

    #[test]
    fn row_div_scalar_divides_all_elements() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();

        // Divide first row by 2
        matrix.row_div_scalar(0, 2.0).unwrap();

        // Verify all elements in row 0 are divided by 2
        assert_eq!(matrix.row(0).unwrap(), &[1.5, 2.5, 0.5, 0.0, 0.0, 39.0]);

        // Verify other rows unchanged
        assert_eq!(matrix.row(1).unwrap(), &[4., 1., 0., 1., 0., 36.]);
        assert_eq!(matrix.row(2).unwrap(), &[-5., -4., 0., 0., 1., 0.]);
    }

    #[test]
    fn row_div_scalar_out_of_bounds_returns_error() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();
        assert!(matrix.row_div_scalar(3, 2.0).is_err());
    }

    #[test]
    fn row_sub_scaled_performs_axpy_operation() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();

        // Perform: row[1] -= 2.0 * row[0]
        // Before: row[0] = [3., 5., 1., 0., 0., 78.]
        //         row[1] = [4., 1., 0., 1., 0., 36.]
        // After:  row[1] = [4., 1., 0., 1., 0., 36.] - 2.0 * [3., 5., 1., 0., 0., 78.]
        //                = [4.-6., 1.-10., 0.-2., 1.-0., 0.-0., 36.-156.]
        //                = [-2., -9., -2., 1., 0., -120.]

        matrix.row_sub_scaled(1, 2.0, 0);

        // Verify row 1 is updated correctly
        assert_eq!(matrix.row(1).unwrap(), &[-2., -9., -2., 1., 0., -120.]);

        // Verify other rows unchanged
        assert_eq!(matrix.row(0).unwrap(), &[3., 5., 1., 0., 0., 78.]);
        assert_eq!(matrix.row(2).unwrap(), &[-5., -4., 0., 0., 1., 0.]);
    }

    #[test]
    fn row_sub_scaled_with_zero_factor() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();

        // Perform: row[1] -= 0.0 * row[0]
        // Row should remain unchanged
        matrix.row_sub_scaled(1, 0.0, 0);

        assert_eq!(matrix.row(1).unwrap(), &[4., 1., 0., 1., 0., 36.]);
    }

    #[test]
    fn row_sub_scaled_with_negative_factor() {
        let mut matrix = FlatMatrix::new(&valid()).unwrap();

        // Perform: row[1] -= (-1.0) * row[0]
        // Which is equivalent to: row[1] += row[0]
        // Before: row[1] = [4., 1., 0., 1., 0., 36.]
        //         row[0] = [3., 5., 1., 0., 0., 78.]
        // After:  row[1] = [4., 1., 0., 1., 0., 36.] + [3., 5., 1., 0., 0., 78.]
        //                = [7., 6., 1., 1., 0., 114.]

        matrix.row_sub_scaled(1, -1.0, 0);

        assert_eq!(matrix.row(1).unwrap(), &[7., 6., 1., 1., 0., 114.]);
    }
}
