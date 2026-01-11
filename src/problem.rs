use crate::model::flat_matrix::FlatMatrix;
use anyhow::{Result, anyhow};
use crate::solvers;

pub struct Problem {
    problem: Vec<f32>,
    constraints: Vec<Vec<f32>>,
    slack_coefs: Vec<f32>,
    objective: Option<Objective>,
    matrix: Option<FlatMatrix<f32>>
}

enum Objective {
    Minimise,
    Maximise,
}

pub struct ProblemBuilder {
    problem: Problem,
}

pub struct ConstraintBuilder {
    constraint: Vec<f32>,
    problem: ProblemBuilder,
}

impl ProblemBuilder {
    pub fn constraint(self, arg: Vec<f32>) -> Result<ConstraintBuilder> {
        if arg.len() != self.problem.problem.len() + 1 {
            return Err(anyhow!(
                "Constraint length {} (including constraint num) doesn't have same amount of variables {}",
                arg.len(),
                self.problem.problem.len()
            ));
        }
        Ok(ConstraintBuilder{
            constraint: arg,
            problem: self,
        })
    }

    pub fn minimise(mut self) -> Self {
        self.problem.objective = Some(Objective::Minimise);
        self
    }

    pub fn maximise(mut self) -> Self {
        self.problem.objective = Some(Objective::Maximise);
        self
    }

    pub fn build(mut self) -> Result<Problem> {
        if self.problem.objective.is_none() {
            return Err(anyhow!("Please specify objective type Minimise or Maximise"));
        }
        self.problem.calculate_matrix()?;
        Ok(self.problem)
    }
}

impl ConstraintBuilder{
    pub fn less(mut self) -> Result<ProblemBuilder>{
        self.problem.problem.constraints.push(self.constraint);
        self.problem.problem.slack_coefs.push(1.);
        Ok(self.problem)
    }
    pub fn more(mut self) -> Result<ProblemBuilder>{
        self.problem.problem.constraints.push(self.constraint);
        self.problem.problem.slack_coefs.push(-1.);
        Ok(self.problem)
    }
}

impl Problem{
    pub fn new(problem: Vec<f32>) -> Result<ProblemBuilder>{
        if problem.len() <= 0{
            Err(anyhow!("Problem must have at least 1 variable"))
        }else{
            Ok(ProblemBuilder {
                problem: Problem {
                    problem,
                    constraints: vec![],
                    slack_coefs: vec![],
                    objective: None,
                    matrix: None,
                },
            })
        }
    }
    fn calculate_matrix(&mut self) -> Result<()>{
        let num_variables = self.problem.len();
        let num_constraints = self.constraints.len();
        let mut table = vec![vec![0.0; num_variables + num_constraints + 1]; num_constraints + 1];
        for (c_idx, c) in self.constraints.iter().enumerate(){
            for (v_idx, v) in c.iter().enumerate() {
                if v_idx == num_variables{
                    table[c_idx][num_variables + num_constraints] = *v;
                }else{
                    table[c_idx][v_idx] = *v;
                }
            }
            table[c_idx][num_variables + c_idx] = self.slack_coefs[c_idx];
        }

        // objective row has negative numbers when maximising, positive when minimising
        let coef = match &self.objective {
            Some(Objective::Minimise) => 1.0,
            Some(Objective::Maximise) => -1.0,
            None => return Err(anyhow!("objective not set")),
        };

        for (v_idx, v) in self.problem.iter().enumerate() {
            table[num_constraints][v_idx] = *v * coef;
        }
        table[num_constraints][num_variables + num_constraints - 1] = 1.;

        println!("{:?}", table);

        self.matrix = Some(FlatMatrix::new(&table)?);
        Ok(())
    }

    pub fn solve_cpu(mut self) -> Result<Vec<f32>>{
        let matrix = self.matrix.as_mut()
            .ok_or_else(|| anyhow!("matrix not calculated"))?;

        solvers::cpu::solve(matrix, None)?;
        self.extract_result()
    }

    fn extract_result(self: Self) -> Result<Vec<f32>>{
        let num_variables = self.problem.len();
        let num_constraints = self.constraints.len();
        let mut res = vec![f32::NEG_INFINITY; num_variables + 1];
        let matrix = self.matrix.ok_or_else(|| anyhow!("matrix not available"))?;

        for r_idx in 0..num_constraints{
            let row = matrix.row(r_idx)?;
            for c_idx in 0..num_variables{
                if row[c_idx] == 1.{
                    res[c_idx] = row[num_variables + num_constraints];
                }
            }
        }

        if res[..num_variables].iter().any(|v| v.is_infinite()){
            Err(anyhow!("no solution was found"))
        }else{
            let mut val = 0.;
            for v_idx in 0..num_variables{
                val += self.problem[v_idx] * res[v_idx];
            }
            res[num_variables] = val;
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_matrix() -> Result<()> {
        let expected = FlatMatrix::new(&vec![
            vec![3., 5., 1., 0., 0.],
            vec![4., 1., 0., 1., 0.],
            vec![-5., -4., 0., 0., 1.],
        ])?;

        let p = Problem::new(vec![5., 4.])?
            .maximise()
            .constraint(vec![3., 5., 78.])?.less()?
            .constraint(vec![4., 1., 36.])?.less()?
            .maximise().build()?;

        let m = p.matrix.ok_or_else(||anyhow!("matrix could not compute"))?;

        assert_eq!(expected.data, m.data);

        Ok(())
    }

    #[test]
    fn test_solve_cpu() -> Result<()> {
        let p = Problem::new(vec![5., 4.])?
            .maximise()
            .constraint(vec![3., 5., 78.])?.less()?
            .constraint(vec![4., 1., 36.])?.less()?
            .build()?;

        let result = p.solve_cpu()?;

        assert_eq!(vec![6., 12., 78.], result);

        Ok(())
    }
}
