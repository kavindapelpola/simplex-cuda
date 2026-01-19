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
    constraints: Vec<Vec<f32>>,
    slack_coefs: Vec<f32>,
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
                slack_coefs: vec![],
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
        let (coeffs, slack_coef) = match constraint {
            Constraint::Lt(coeffs) => (coeffs, 1.),
            Constraint::Gt(coeffs) => (coeffs, -1.),
            Constraint::Eq(coeffs) => (coeffs, 0.),
        };

        if coeffs.len() != self.constraint_width() {
            return Err(anyhow!("Invalid constraint length {}", coeffs.len()));
        }

        self.constraints.push(coeffs);
        self.slack_coefs.push(slack_coef);
        Ok(self)
    }

    /// Build the problem (matrix) from the problem and constraints
    pub fn build(mut self) -> Result<Self> {
        let cols = self.constraint_width() + self.slack_coefs.len() + 1; // include a col for the objective function rhs
        let rows = self.constraints.len() + 1; // all the constraints and a row for the objective function
        let mut matrix = FlatMatrix::<f32>::new(rows, cols)?;

        // build the constraint rows in the matrix
        self.constraints.iter().enumerate().for_each(|(c_idx, c)| {
            let row = matrix.row_mut(c_idx).unwrap();
            row[0..c.len() - 1].copy_from_slice(&c[..c.len() - 1]);
            row[self.problem.len() + c_idx] = self.slack_coefs[c_idx];
            row[row.len() - 1] = c[c.len() - 1];
        });

        // build the problem row
        matrix.last_row_mut()[0..self.problem.len()].copy_from_slice(&self.problem);
        // objective row has negative numbers when maximising, positive when minimising
        if self.is_maximise() {
            matrix.row_mul_scalar(rows - 1, -1.)?;
        }
        matrix.set(rows - 1, self.problem.len() + self.constraints.len(), 1.)?;

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

    #[test]
    fn test_build() -> Result<()> {
        let expected = FlatMatrix::from_vec(&vec![
            vec![3., 5., 1., 0., 0., 78.],
            vec![4., 1., 0., 1., 0., 36.],
            vec![-5., -4., 0., 0., 1., 0.],
        ])?;

        let p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![3., 5., 78.]))?
            .with(Constraint::Lt(vec![4., 1., 36.]))?
            .build()?;

        let m = p
            .matrix
            .ok_or_else(|| anyhow!("matrix could not compute"))?;

        assert_eq!(expected.data, m.data);

        Ok(())
    }

    #[test]
    fn test_result() -> Result<()> {
        let mut p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![3., 5., 78.]))?
            .with(Constraint::Lt(vec![4., 1., 36.]))?
            .build()?;

        solvers::cpu::solve(&mut p, None)?;

        let result = p.result()?;

        assert_eq!(vec![6., 12., 78.], result);

        Ok(())
    }
}
