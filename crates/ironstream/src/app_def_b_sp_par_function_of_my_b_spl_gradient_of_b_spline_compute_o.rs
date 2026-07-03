// FILE: app_def_b_sp_par_function_of_my_b_spl_gradient_of_b_spline_compute_o.rs
// occt: AppDef_BSpParFunctionOfMyBSplGradientOfBSplineCompute
pub struct BSpParFunction { pub value: f64 }
impl BSpParFunction { pub fn new() -> Self { BSpParFunction { value: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = BSpParFunction::new(); } }
