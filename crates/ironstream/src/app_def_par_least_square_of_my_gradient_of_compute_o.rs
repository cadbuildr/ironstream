// FILE: app_def_par_least_square_of_my_gradient_of_compute_o.rs
// occt: AppDef_ParLeastSquareOfMyGradientOfCompute
pub struct ParLeastSquare { pub residual: f64 }
impl ParLeastSquare { pub fn new() -> Self { ParLeastSquare { residual: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = ParLeastSquare::new(); } }
