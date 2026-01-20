/// Constraint in the problem, defined as the coefficients of the variable and
/// a less than, greater than or equal to relationship to a rhs
pub enum Constraint {
    Lt(Vec<f32>),
    Gt(Vec<f32>),
    Eq(Vec<f32>),
}

impl Constraint {
    /// Get the coefficients of the constraint
    pub fn coeffs(&self) -> &Vec<f32> {
        match self {
            Constraint::Lt(coeffs) | Constraint::Gt(coeffs) | Constraint::Eq(coeffs) => coeffs,
        }
    }

    /// Get the number of auxilliary variables (slack, surplus or artificial) associated with this constraint
    pub fn aux_vars(&self) -> usize {
        match self {
            Constraint::Lt(_) => 1, // slack var for less than
            Constraint::Gt(_) => 2, // surplus and artificial for greater than
            Constraint::Eq(_) => 1, // artificial for equal
        }
    }
}
