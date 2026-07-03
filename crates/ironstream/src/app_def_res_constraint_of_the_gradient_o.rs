// FILE: app_def_res_constraint_of_the_gradient_o.rs
// occt: AppDef_ResConstraintOfTheGradient
pub struct TheResConstraint { pub error: f64 }
impl TheResConstraint { pub fn new() -> Self { TheResConstraint { error: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = TheResConstraint::new(); } }
