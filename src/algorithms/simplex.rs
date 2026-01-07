use ndarray::array;

pub fn solve() {
    let mut table = array![
        [3., 5., 1., 0., 0., 78.],  // constraint 1
        [4., 1., 0., 1., 0., 36.],  // constraint 3
        [-5., -4., 0., 0., 1., 0.], // objective
    ];
    println!("{:?}", table);

    loop {
        let entry = table
            .row(table.nrows() - 1)
            .iter()
            .enumerate()
            .into_iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index);

        println!("{:?}", entry);

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
        println!("{:?}", exit);

        let divisor = table[[exit.unwrap(), entry.unwrap()]];
        println!("Divisor: {}", divisor);

        *table.row_mut(exit.unwrap()) /= divisor;

        for row_index in 0..table.nrows() {
            if row_index != exit.unwrap() {
                let factor = table[[row_index, entry.unwrap()]];
                let pivot_row = table.row(exit.unwrap()).to_owned();
                table.row_mut(row_index).scaled_add(-factor, &pivot_row);
            }
        }

        println!("{:?}", table);

        if table.row(table.nrows() - 1).iter().all(|&x| x >= 0.) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        solve();
        assert!(false);
    }
}
