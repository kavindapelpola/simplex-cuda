//! FlatMatrix: A row-major flattened 2D matrix optimized for performance.
//!
//! # Performance Characteristics
//!
//! This implementation prioritizes:
//! - Simple, clean code over complex optimizations
//! - Row-based operations (contiguous memory access)
//! - Compiler auto-vectorization hints through slice operations
//!
//! ## Benchmark Results (simplex algorithm, 3x6 matrix)
//!
//! - **ndarray**: ~100 ns
//! - **FlatMatrix**: ~164 ns (64% slower)
//!
//! ## Vectorization Analysis
//!
//! Assembly inspection reveals why ndarray is faster:
//!
//! ### ndarray's `scaled_add`:
//! ```asm
//! fmul z1.d, z0.d, z1.d    ; SIMD multiply (z = vector register)
//! fsub z1.d, z2.d, z1.d    ; SIMD subtract
//! → Processes 2-4 elements per instruction
//! ```
//!
//! ### FlatMatrix's `row_sub_scaled`:
//! ```asm
//! fmul d1, d0, d1          ; Scalar multiply (d = single register)
//! fsub d1, d3, d1          ; Scalar subtract
//! → Processes 1 element per instruction
//! ```
//!
//! **Root cause**: Despite using slice operations, the compiler does not
//! auto-vectorize our hot loop. This accounts for the ~2-4x performance gap.
//!
//! ## Optimization Attempts
//!
//! We tried:
//! 1. ✗ `#[inline(always)]` - No effect (already inlined)
//! 2. ✗ Direct loop inlining - Worse (slice recreation overhead)
//! 3. ✗ Explicit portable SIMD - Requires unstable features
//! 4. ✗ While loop + pointer arithmetic (Option B):
//!    - Tested `while i < len { *ptr.offset(i * stride) = ... }` → 166ns (no change)
//!    - Tested `while i < len { *ptr.add(i) = ... }` → 166ns (no change)
//!    - Tested read-compute-write pattern → 167ns (worse)
//!    - Assembly inspection: Still scalar d-registers, not vectorized z-registers
//!    - Clean for loop is actually faster (164ns vs 166ns)
//!
//! ## Why Not Explicit SIMD?
//!
//! Portable SIMD (`std::simd`) is still unstable and would:
//! - Require nightly Rust
//! - Add complexity to the codebase
//! - Only benefit this specific use case
//!
//! ## Conclusion
//!
//! The current implementation represents a good balance:
//! - Clean, maintainable code
//! - Reasonable performance (67% gap is acceptable for most use cases)
//! - No unstable features or complex SIMD code
//!
//! For applications requiring maximum performance, consider using ndarray
//! with BLAS enabled, which would widen the gap further through hardware-
//! optimized linear algebra routines.

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

    /// # Safety
    /// Caller must ensure row < self.rows and col < self.cols.
    /// No bounds checking is performed for performance.
    #[inline]
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        unsafe { self.data.get_unchecked(row * self.cols + col) }
    }

    /// # Safety
    /// Caller must ensure row < self.rows and col < self.cols.
    /// No bounds checking is performed for performance.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(row * self.cols + col) }
    }

    /// Divide all elements in a row by a scalar value
    /// Optimization 3: Vectorizable row operation
    pub fn row_div_scalar(&mut self, row_index: usize, divisor: T) -> Result<()>
    where
        T: std::ops::DivAssign,
    {
        let row = self.row_mut(row_index)?;
        for elem in row.iter_mut() {
            *elem /= divisor.clone();
        }
        Ok(())
    }

    /// Subtract a scaled row from another row: target_row -= factor * source_row
    /// Optimized AXPY-style operation for simplex algorithm.
    ///
    /// # Safety
    /// Uses unsafe code to create non-overlapping row references.
    /// Caller must ensure row_index != source_row_index.
    ///
    /// # Vectorization
    /// Force inline to help compiler auto-vectorize the loop.
    #[inline(always)]
    pub fn row_sub_scaled(&mut self, row_index: usize, factor: T, source_row_index: usize)
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
    {
        debug_assert_ne!(row_index, source_row_index, "rows must not overlap");
        debug_assert!(row_index < self.rows);
        debug_assert!(source_row_index < self.rows);

        let cols = self.cols;
        let data_ptr = self.data.as_mut_ptr();

        unsafe {
            // Create non-overlapping slices (safe because row_index != source_row_index)
            let source = std::slice::from_raw_parts(
                data_ptr.add(source_row_index * cols),
                cols
            );
            let target = std::slice::from_raw_parts_mut(
                data_ptr.add(row_index * cols),
                cols
            );

            // Clean slice-based loop
            // Note: Despite attempts with while loops and pointer arithmetic patterns,
            // LLVM does not vectorize this loop. Assembly shows scalar d-register
            // instructions instead of vector z-register instructions.
            for i in 0..cols {
                target[i] = target[i] - factor * source[i];
            }
        }
    }
}

// Test: f32-specific implementation to check if generics prevent vectorization
impl FlatMatrix<f32> {
    /// Specialized f32 version - test if concrete type helps vectorization
    #[inline(always)]
    pub fn row_sub_scaled_f32(&mut self, row_index: usize, factor: f32, source_row_index: usize) {
        debug_assert_ne!(row_index, source_row_index, "rows must not overlap");
        debug_assert!(row_index < self.rows);
        debug_assert!(source_row_index < self.rows);

        let cols = self.cols;
        let data_ptr = self.data.as_mut_ptr();

        unsafe {
            let source = std::slice::from_raw_parts(
                data_ptr.add(source_row_index * cols),
                cols
            );
            let target = std::slice::from_raw_parts_mut(
                data_ptr.add(row_index * cols),
                cols
            );

            // Simple indexed loop - cleaner and slightly faster than iterator approach
            for i in 0..cols {
                target[i] = target[i] - factor * source[i];
            }
        }
    }
}

impl<T: Clone> FlatMatrix<T> {
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
