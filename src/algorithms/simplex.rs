use ndarray::array;

use crate::model::flat_matrix::FlatMatrix;

pub fn solve_ndarray() {
    let mut table = array![
        [3., 5., 1., 0., 0., 78.],  // constraint 1
        [4., 1., 0., 1., 0., 36.],  // constraint 3
        [-5., -4., 0., 0., 1., 0.], // objective
    ];

    loop {
        let entry = table
            .row(table.nrows() - 1)
            .iter()
            .enumerate()
            .into_iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);

        let exit = table
            .column(entry.unwrap())
            .iter()
            .enumerate()
            .take(table.nrows() - 1)
            .filter_map(|(row_index, &value)| {
                if value > 0. {
                    let ratio = table[[row_index, table.ncols() - 1]] / value;
                    Some((row_index, ratio))
                } else {
                    None
                }
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);

        let divisor = table[[exit.unwrap(), entry.unwrap()]];

        *table.row_mut(exit.unwrap()) /= divisor;

        for row_index in 0..table.nrows() {
            if row_index != exit.unwrap() {
                let factor = table[[row_index, entry.unwrap()]];
                let pivot_row = table.row(exit.unwrap()).to_owned();
                table.row_mut(row_index).scaled_add(-factor, &pivot_row);
            }
        }

        if table.row(table.nrows() - 1).iter().all(|&x| x >= 0.) {
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

        // Divide pivot row by divisor
        for col in 0..table.cols {
            *table.get_mut(exit, col).unwrap() /= divisor;
        }

        // Eliminate column in other rows
        for row_index in 0..table.rows {
            if row_index != exit {
                let factor = *table.get(row_index, entry).unwrap();
                for col in 0..table.cols {
                    let pivot_value = *table.get(exit, col).unwrap();
                    *table.get_mut(row_index, col).unwrap() -= factor * pivot_value;
                }
            }
        }

        if table.last_row().iter().all(|&x| x >= 0.) {
            break;
        }
    }
}
