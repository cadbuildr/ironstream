// FILE: app_def_par_least_square_of_the_gradient_o.rs
// occt: AppDef_ParLeastSquareOfTheGradient
pub struct TheParLeastSquare { pub residual: f64 }
impl TheParLeastSquare { pub fn new() -> Self { TheParLeastSquare { residual: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = TheParLeastSquare::new(); } }
