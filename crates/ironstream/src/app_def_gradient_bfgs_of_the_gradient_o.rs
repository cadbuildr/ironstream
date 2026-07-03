// FILE: app_def_gradient_bfgs_of_the_gradient_o.rs
// occt: AppDef_Gradient_BFGSOfTheGradient
pub struct TheGradientBFGS { pub iteration: usize }
impl TheGradientBFGS { pub fn new() -> Self { TheGradientBFGS { iteration: 0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = TheGradientBFGS::new(); } }
