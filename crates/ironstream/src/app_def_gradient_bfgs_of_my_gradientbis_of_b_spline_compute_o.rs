// FILE: app_def_gradient_bfgs_of_my_gradientbis_of_b_spline_compute_o.rs
// occt: AppDef_Gradient_BFGSOfMyGradientbisOfBSplineCompute
pub struct GradientBFGSBis { pub iteration: usize }
impl GradientBFGSBis { pub fn new() -> Self { GradientBFGSBis { iteration: 0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = GradientBFGSBis::new(); } }
