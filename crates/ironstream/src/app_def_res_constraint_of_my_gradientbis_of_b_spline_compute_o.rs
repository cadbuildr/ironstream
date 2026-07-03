// FILE: app_def_res_constraint_of_my_gradientbis_of_b_spline_compute_o.rs
// occt: AppDef_ResConstraintOfMyGradientbisOfBSplineCompute
pub struct ResConstraintBis { pub error: f64 }
impl ResConstraintBis { pub fn new() -> Self { ResConstraintBis { error: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = ResConstraintBis::new(); } }
