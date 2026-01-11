use anyhow::{Result, anyhow};
use tracing::info;
use crate::model::flat_matrix::FlatMatrix;
const MAX_LOOPS: usize = 1_000_000;

pub fn solve(table: &mut FlatMatrix<f32>, max_loops: Option<usize>) -> Result<()> {
    let mut counter = 0;
    loop {
        if counter >= max_loops.unwrap_or(MAX_LOOPS) {
            return Err(anyhow!("no solution found after {} loops", counter));
        }
        counter += 1;

        let entry = table
            .last_row()
            .iter()
            .enumerate()
            .into_iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .ok_or_else(|| anyhow!("no negative coefficients found in objective function"))?;

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
            .ok_or_else(|| anyhow!("no valid pivot row found"))?;

        let divisor = *table.get(exit, entry).unwrap();
        let pivot_row = table.row_mut(exit).unwrap();
        for elem in pivot_row.iter_mut() {
            *elem /= divisor;
        }

        for row_index in 0..table.rows {
            if row_index != exit {
                let factor = unsafe { *table.get_unchecked(row_index, entry) };
                table.row_sub_scaled(row_index, factor, exit);
            }
        }

        if table.last_row().iter().all(|&x| x >= 0.) {
            break;
        }
    }
    info!("solution found after {} loops", counter);
    Ok(())
}