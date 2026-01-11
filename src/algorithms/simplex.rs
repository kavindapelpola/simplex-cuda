use ndarray::array;

use crate::model::flat_matrix::FlatMatrix;

pub fn solve_ndarray() {
    let mut table = array![
        [3., 5., 1., 0., 0., 78.],  // constraint 1
        [4., 1., 0., 1., 0., 36.],  // constraint 3
        [-5., -4., 0., 0., 1., 0.], // objective
    ];

    loop {
        // Cache dimensions to avoid repeated calculations
        let last_row = table.nrows() - 1;
        let last_col = table.ncols() - 1;

        // Find entry column (most negative in objective row)
        let entry_col = table
            .row(last_row)
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();

        // Find exit row (minimum ratio test)
        let exit_row = table
            .column(entry_col)
            .iter()
            .enumerate()
            .take(last_row)
            .filter_map(|(row_index, &value)| {
                if value > 0. {
                    let ratio = table[[row_index, last_col]] / value;
                    Some((row_index, ratio))
                } else {
                    None
                }
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();

        // Normalize pivot row
        let divisor = table[[exit_row, entry_col]];
        *table.row_mut(exit_row) /= divisor;

        // CRITICAL OPTIMIZATION: Eliminate other rows without allocation
        // Instead of: let pivot_row = table.row(exit_row).to_owned();
        // Use unsafe to get non-overlapping slice borrows for better vectorization
        let nrows = table.nrows();
        let ncols = table.ncols();
        unsafe {
            let data_ptr = table.as_mut_ptr();

            // Create immutable slice for pivot row (safe: we won't modify it)
            let pivot_row = std::slice::from_raw_parts(
                data_ptr.add(exit_row * ncols),
                ncols
            );

            for row_index in 0..nrows {
                if row_index != exit_row {
                    let factor = *data_ptr.add(row_index * ncols + entry_col);

                    // Create mutable slice for target row (safe: non-overlapping with pivot)
                    let target_row = std::slice::from_raw_parts_mut(
                        data_ptr.add(row_index * ncols),
                        ncols
                    );

                    // AXPY using iterator pattern for optimal vectorization
                    for (t, &p) in target_row.iter_mut().zip(pivot_row.iter()) {
                        *t -= factor * p;
                    }
                }
            }
        }

        // Check termination condition
        if table.row(last_row).iter().all(|&x| x >= 0.) {
            break;
        }
    }
}

pub fn solve_flatmatrix() {
    let mut table = FlatMatrix::new(&vec![
        vec![3., 5., 1., 0., 0., 78.],
        vec![4., 1., 0., 1., 0., 36.],
        vec![-5., -4., 0., 0., 1., 0.],
    ])
    .unwrap();

    loop {
        let entry = table
            .last_row()
            .iter()
            .enumerate()
            .into_iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();

        let exit = table
            .col(entry)
            .unwrap()
            .iter()
            .enumerate()
            .take(table.rows - 1)
            .filter_map(|(row_index, &value)| {
                if *value > 0. {
                    let ratio = table.get(row_index, table.cols - 1).unwrap() / value;
                    Some((row_index, ratio))
                } else {
                    None
                }
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();

        let divisor = *table.get(exit, entry).unwrap();

        // Optimized row operations with slice-based memory access
        // Division: operates directly on mutable row slice
        let pivot_row = table.row_mut(exit).unwrap();
        for elem in pivot_row.iter_mut() {
            *elem /= divisor;
        }

        // AXPY operation: target_row -= factor * source_row
        for row_index in 0..table.rows {
            if row_index != exit {
                let factor = unsafe { *table.get_unchecked(row_index, entry) };
                table.row_sub_scaled_f32(row_index, factor, exit);
            }
        }

        if table.last_row().iter().all(|&x| x >= 0.) {
            break;
        }
    }
}
