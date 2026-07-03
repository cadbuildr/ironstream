// FILE: app_def_par_least_square_of_my_gradientbis_of_b_spline_compute_o.rs
// occt: AppDef_ParLeastSquareOfMyGradientbisOfBSplineCompute
pub struct ParLeastSquareBis { pub residual: f64 }
impl ParLeastSquareBis { pub fn new() -> Self { ParLeastSquareBis { residual: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = ParLeastSquareBis::new(); } }
