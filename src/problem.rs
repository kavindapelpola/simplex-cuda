use crate::model::flat_matrix::FlatMatrix;
use anyhow::{Result, anyhow};
use std::fmt::Display;

pub enum Constraint {
    Lt(Vec<f32>),
    Gt(Vec<f32>),
    Eq(Vec<f32>),
}

pub struct Problem {
    problem: Vec<f32>,
    objective: Objective,
    constraints: Vec<Constraint>,
    pub matrix: Option<FlatMatrix<f32>>,
}

#[derive(PartialEq)]
enum Objective {
    Minimize,
    Maximize,
}

impl Problem {
    /// Create a new problem
    fn new(problem: &Vec<f32>, objective: Objective) -> Result<Problem> {
        if problem.len() <= 0 {
            Err(anyhow!("Problem must have at least one variable"))
        } else {
            Ok(Problem {
                problem: problem.clone(),
                constraints: vec![],
                objective,
                matrix: None,
            })
        }
    }

    /// Create a new maximize problem
    pub fn maximize(problem: &Vec<f32>) -> Result<Problem> {
        Problem::new(problem, Objective::Maximize)
    }

    /// Create a new minimize problem
    pub fn minimize(problem: &Vec<f32>) -> Result<Problem> {
        Problem::new(problem, Objective::Minimize)
    }

    /// Is this a maximize problem
    pub fn is_maximise(&self) -> bool {
        self.objective == Objective::Maximize
    }

    /// Is this a minimize problem
    pub fn is_minimize(&self) -> bool {
        self.objective == Objective::Minimize
    }

    /// Required constraint width. Must include all the problem variables and an rhs,
    /// so problem width + 1
    fn constraint_width(&self) -> usize {
        self.problem.len() + 1
    }

    /// Add a constraint
    pub fn with(mut self, constraint: Constraint) -> Result<Self> {
        let coeffs = match &constraint {
            Constraint::Lt(coeffs) | Constraint::Gt(coeffs) | Constraint::Eq(coeffs) => coeffs,
        };

        if coeffs.len() != self.constraint_width() {
            return Err(anyhow!("Invalid constraint length {}", coeffs.len()));
        }

        self.constraints.push(constraint);
        Ok(self)
    }

    /// Build the problem (matrix) from the problem and constraints
    pub fn build(mut self) -> Result<Self> {
        // Count columns needed for each constraint type
        let num_slack_cols = self.constraints.iter().fold(0usize, |acc, constraint| {
            acc + match constraint {
                Constraint::Lt(_) => 1, // slack var for less than
                Constraint::Gt(_) => 2, // surplus and artificial for greater than
                Constraint::Eq(_) => 1, // artificial for equal
            }
        });
        let cols = self.constraint_width() + num_slack_cols + 1; // include a col for the objective function rhs
        let rows = self.constraints.len() + 1; // all the constraints and a row for the objective function
        let mut matrix = FlatMatrix::<f32>::new(rows, cols)?;

        // build the constraint rows in the matrix
        let mut slack_col_idx = self.problem.len();
        self.constraints
            .iter()
            .enumerate()
            .for_each(|(c_idx, constraint)| {
                let coeffs = match constraint {
                    Constraint::Lt(coeffs) | Constraint::Gt(coeffs) | Constraint::Eq(coeffs) => {
                        coeffs
                    }
                };
                let row = matrix.row_mut(c_idx).unwrap();
                row[0..coeffs.len() - 1].copy_from_slice(&coeffs[..coeffs.len() - 1]);

                // Add slack/surplus/artificial variables
                match constraint {
                    Constraint::Lt(_) => {
                        row[slack_col_idx] = 1.;
                        slack_col_idx += 1;
                    }
                    Constraint::Gt(_) => {
                        row[slack_col_idx] = -1.; // surplus
                        row[slack_col_idx + 1] = 1.; // artificial
                        slack_col_idx += 2;
                    }
                    Constraint::Eq(_) => {
                        row[slack_col_idx] = 1.; // artificial
                        slack_col_idx += 1;
                    }
                }

                row[row.len() - 1] = coeffs[coeffs.len() - 1];
            });

        // build the problem row
        matrix.last_row_mut()[0..self.problem.len()].copy_from_slice(&self.problem);
        // objective row has negative numbers when maximising, positive when minimising
        if self.is_maximise() {
            matrix.row_mul_scalar(rows - 1, -1.)?;
        }
        matrix.set(rows - 1, cols - 2, 1.)?;

        self.matrix = Some(matrix);
        Ok(self)
    }

    /// Return the results of a solve
    pub fn result(self) -> Result<Vec<f32>> {
        let matrix = self.matrix.ok_or_else(|| anyhow!("matrix not available"))?;
        let num_variables = self.problem.len();
        let mut res = vec![f32::NEG_INFINITY; num_variables + 1];

        // Extract basic variable values from constraint rows
        for row in matrix.rows().take(matrix.rows - 1) {
            if let Some((c_idx, _)) = row[..num_variables]
                .iter()
                .enumerate()
                .find(|(_, val)| **val == 1.)
            {
                res[c_idx] = *row.last().unwrap();
            }
        }

        if res[..num_variables].iter().any(|v| v.is_infinite()) {
            return Err(anyhow!("no solution was found"));
        }

        // Calculate objective value
        res[num_variables] = self
            .problem
            .iter()
            .zip(&res[..num_variables])
            .map(|(p, r)| p * r)
            .sum();

        Ok(res)
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(table) = &self.matrix {
            for i in 0..table.rows {
                for j in 0..table.cols - 1 {
                    write!(f, "{:>6.2} ", table.get(i, j).unwrap())?;
                }
                writeln!(f, "| {:>6.2}", table.get(i, table.cols - 1).unwrap())?;
            }
            Ok(())
        } else {
            write!(f, "matrix not computed, did you forget to build?")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers;

    // Build tests - Less Than constraints
    #[test]
    fn test_build_with_lt_constraints() -> Result<()> {
        let expected = FlatMatrix::from_vec(&vec![
            vec![3., 5., 1., 0., 0., 78.],
            vec![4., 1., 0., 1., 0., 36.],
            vec![-5., -4., 0., 0., 1., 0.],
        ])?;

        let p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![3., 5., 78.]))?
            .with(Constraint::Lt(vec![4., 1., 36.]))?
            .build()?;

        let m = p.matrix.ok_or_else(|| anyhow!("matrix not built"))?;

        assert_eq!(expected.data, m.data);
        Ok(())
    }

    // Build tests - Greater Than constraints
    #[test]
    fn test_build_with_gt_constraints() -> Result<()> {
        // Greater than adds 2 columns per constraint: surplus (-1) and artificial (+1)
        let expected = FlatMatrix::from_vec(&vec![
            vec![3., 5., -1., 1., 0., 0., 0., 78.],
            vec![4., 1., 0., 0., -1., 1., 0., 36.],
            vec![-5., -4., 0., 0., 0., 0., 1., 0.],
        ])?;

        let p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Gt(vec![3., 5., 78.]))?
            .with(Constraint::Gt(vec![4., 1., 36.]))?
            .build()?;

        let m = p.matrix.ok_or_else(|| anyhow!("matrix not built"))?;

        assert_eq!(expected.data, m.data);
        Ok(())
    }

    // Build tests - Equality constraints
    #[test]
    fn test_build_with_eq_constraints() -> Result<()> {
        // Equality adds 1 column per constraint: artificial (+1)
        let expected = FlatMatrix::from_vec(&vec![
            vec![3., 5., 1., 0., 0., 78.],
            vec![4., 1., 0., 1., 0., 36.],
            vec![-5., -4., 0., 0., 1., 0.],
        ])?;

        let p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Eq(vec![3., 5., 78.]))?
            .with(Constraint::Eq(vec![4., 1., 36.]))?
            .build()?;

        let m = p.matrix.ok_or_else(|| anyhow!("matrix not built"))?;

        assert_eq!(expected.data, m.data);
        Ok(())
    }

    // Build tests - Mixed constraints
    #[test]
    fn test_build_with_mixed_constraints() -> Result<()> {
        // Lt (1 col) + Gt (2 cols) + Eq (1 col) = 4 extra columns
        let expected = FlatMatrix::from_vec(&vec![
            vec![2., 3., 1., 0., 0., 0., 0., 10.],  // Lt: slack
            vec![1., 2., 0., -1., 1., 0., 0., 8.],   // Gt: surplus + artificial
            vec![3., 1., 0., 0., 0., 1., 0., 12.],   // Eq: artificial
            vec![-5., -4., 0., 0., 0., 0., 1., 0.],  // objective + w
        ])?;

        let p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![2., 3., 10.]))?
            .with(Constraint::Gt(vec![1., 2., 8.]))?
            .with(Constraint::Eq(vec![3., 1., 12.]))?
            .build()?;

        let m = p.matrix.ok_or_else(|| anyhow!("matrix not built"))?;

        assert_eq!(expected.data, m.data);
        Ok(())
    }

    // Build tests - Maximize negates objective
    #[test]
    fn test_build_maximize_negates_objective() -> Result<()> {
        let p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![3., 5., 78.]))?
            .build()?;

        let m = p.matrix.ok_or_else(|| anyhow!("matrix not built"))?;
        let objective_row = m.last_row();

        assert_eq!(objective_row[0], -5.);
        assert_eq!(objective_row[1], -4.);
        Ok(())
    }

    // Build tests - Minimize keeps objective positive
    #[test]
    fn test_build_minimize_keeps_objective_positive() -> Result<()> {
        let p = Problem::minimize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![3., 5., 78.]))?
            .build()?;

        let m = p.matrix.ok_or_else(|| anyhow!("matrix not built"))?;
        let objective_row = m.last_row();

        assert_eq!(objective_row[0], 5.);
        assert_eq!(objective_row[1], 4.);
        Ok(())
    }

    // Constraint validation tests
    #[test]
    fn test_constraint_wrong_width_returns_error() {
        let result = Problem::maximize(&vec![5., 4.])
            .unwrap()
            .with(Constraint::Lt(vec![3., 5.])); // Missing RHS

        assert!(result.is_err());
    }

    // Result extraction tests
    #[test]
    fn test_result_extracts_solution() -> Result<()> {
        let mut p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![3., 5., 78.]))?
            .with(Constraint::Lt(vec![4., 1., 36.]))?
            .build()?;

        solvers::cpu::solve(&mut p, None)?;

        let result = p.result()?;

        assert_eq!(vec![6., 12., 78.], result);
        Ok(())
    }

    #[test]
    fn test_result_no_matrix_returns_error() {
        let p = Problem::maximize(&vec![5., 4.])
            .unwrap()
            .with(Constraint::Lt(vec![3., 5., 78.]))
            .unwrap();

        let result = p.result();

        assert!(result.is_err());
    }
}
