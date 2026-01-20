use super::super::Problem;
use anyhow::{Result, anyhow};
use tracing::info;
const MAX_LOOPS: usize = 1_000_000;

pub fn solve(problem: &mut Problem, max_loops: Option<usize>) -> Result<()> {
    let mut counter = 0;
    let is_minimize = problem.is_minimize();
    let matrix = problem.matrix_mut()?;
    loop {
        if counter >= max_loops.unwrap_or(MAX_LOOPS) {
            return Err(anyhow!("no solution found after {} loops", counter));
        }
        counter += 1;

        let entry = matrix
            .last_row()
            .iter()
            .take(matrix.cols - 2)
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .ok_or_else(|| anyhow!("no negative coefficients found in objective function"))?;

        let exit = matrix
            .col(entry)
            .unwrap()
            .iter()
            .enumerate()
            .take(matrix.rows - 1)
            .filter_map(|(row_index, &value)| {
                if *value > 0. {
                    let ratio = matrix.get(row_index, matrix.cols - 1).unwrap() / value;
                    Some((row_index, ratio))
                } else {
                    None
                }
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .ok_or_else(|| anyhow!("no valid pivot row found"))?;

        let divisor = *matrix.get(exit, entry).unwrap();
        let pivot_row = matrix.row_mut(exit).unwrap();
        for elem in pivot_row.iter_mut() {
            *elem /= divisor;
        }

        for row_index in 0..matrix.rows {
            if row_index != exit {
                let factor = unsafe { *matrix.get_unchecked(row_index, entry) };
                matrix.row_sub_scaled(row_index, factor, exit);
            }
        }

        let optimal = match is_minimize {
            true => matrix.last_row().iter().all(|&x| x <= 0.),
            false => matrix.last_row().iter().all(|&x| x >= 0.),
        };

        if optimal {
            break;
        }
    }
    info!("solution found after {} loops", counter);
    Ok(())
}
