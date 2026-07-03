// FILE: app_def_res_constraint_of_my_gradient_of_compute_o.rs
// occt: AppDef_ResConstraintOfMyGradientOfCompute
pub struct ResConstraint { pub error: f64 }
impl ResConstraint { pub fn new() -> Self { ResConstraint { error: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = ResConstraint::new(); } }
