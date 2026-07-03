// FILE: app_def_gradient_bfgs_of_my_gradient_of_compute_o.rs
// occt: AppDef_Gradient_BFGSOfMyGradientOfCompute
pub struct GradientBFGS { pub iteration: usize }
impl GradientBFGS { pub fn new() -> Self { GradientBFGS { iteration: 0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = GradientBFGS::new(); } }
