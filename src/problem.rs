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
    fn new(problem: &Vec<f32>, objective: Objective) -> Result<Problem> {
        if problem.len() <= 0 {
            Err(anyhow!("Problem must have at least 1 variable"))
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

    pub fn maximize(problem: &Vec<f32>) -> Result<Problem> {
        Problem::new(problem, Objective::Maximize)
    }

    pub fn minimize(problem: &Vec<f32>) -> Result<Problem> {
        Problem::new(problem, Objective::Minimize)
    }

    pub fn is_minimize(&self) -> bool {
        self.objective == Objective::Minimize
    }

    fn dimension(&self) -> usize {
        self.problem.len() + 1
    }

    pub fn with(mut self, constraint: Constraint) -> Result<Self> {
        match constraint {
            Constraint::Lt(coeffs) => {
                if coeffs.len() != self.dimension() {
                    return Err(anyhow!("Invaid constraint length {}", coeffs.len()));
                }
                self.constraints.push(coeffs);
                self.slack_coefs.push(1.);
            }
            Constraint::Gt(_) => (),
            Constraint::Eq(_) => (),
        }
        Ok(self)
    }

    pub fn build(mut self) -> Result<Self> {
        let num_variables = self.problem.len();
        let num_constraints = self.constraints.len();
        let mut table = vec![vec![0.0; num_variables + num_constraints + 2]; num_constraints + 1];
        for (c_idx, c) in self.constraints.iter().enumerate() {
            for (v_idx, v) in c.iter().enumerate() {
                if v_idx == num_variables {
                    table[c_idx][num_variables + num_constraints + 1] = *v;
                } else {
                    table[c_idx][v_idx] = *v;
                }
            }
            table[c_idx][num_variables + c_idx] = self.slack_coefs[c_idx];
        }

        // objective row has negative numbers when maximising, positive when minimising
        let coef = match &self.objective {
            Objective::Minimize => 1.0,
            Objective::Maximize => -1.0,
        };

        for (v_idx, v) in self.problem.iter().enumerate() {
            table[num_constraints][v_idx] = *v * coef;
        }
        table[num_constraints][num_variables + num_constraints] = 1.;

        self.matrix = Some(FlatMatrix::new(&table)?);
        Ok(self)
    }

    pub fn extract_result(self: Self) -> Result<Vec<f32>> {
        let num_variables = self.problem.len();
        let num_constraints = self.constraints.len();
        let mut res = vec![f32::NEG_INFINITY; num_variables + 1];
        let matrix = self.matrix.ok_or_else(|| anyhow!("matrix not available"))?;

        for r_idx in 0..num_constraints {
            let row = matrix.row(r_idx)?;
            for c_idx in 0..num_variables {
                if row[c_idx] == 1. {
                    res[c_idx] = row[num_variables + num_constraints + 1];
                }
            }
        }

        if res[..num_variables].iter().any(|v| v.is_infinite()) {
            Err(anyhow!("no solution was found"))
        } else {
            let mut val = 0.;
            for v_idx in 0..num_variables {
                val += self.problem[v_idx] * res[v_idx];
            }
            res[num_variables] = val;
            Ok(res)
        }
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
            write!(f, "matrix not computed")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers;

    #[test]
    fn test_build() -> Result<()> {
        let expected = FlatMatrix::new(&vec![
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
    fn test_extract_result() -> Result<()> {
        let mut p = Problem::maximize(&vec![5., 4.])?
            .with(Constraint::Lt(vec![3., 5., 78.]))?
            .with(Constraint::Lt(vec![4., 1., 36.]))?
            .build()?;

        solvers::cpu::solve(&mut p, None)?;

        let result = p.extract_result()?;

        assert_eq!(vec![6., 12., 78.], result);

        Ok(())
    }
}
