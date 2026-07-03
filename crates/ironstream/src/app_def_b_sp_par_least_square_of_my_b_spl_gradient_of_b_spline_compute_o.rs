// FILE: app_def_b_sp_par_least_square_of_my_b_spl_gradient_of_b_spline_compute_o.rs
// occt: AppDef_BSpParLeastSquareOfMyBSplGradientOfBSplineCompute
pub struct BSpParLeastSquare { pub residual: f64 }
impl BSpParLeastSquare { pub fn new() -> Self { BSpParLeastSquare { residual: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = BSpParLeastSquare::new(); } }
