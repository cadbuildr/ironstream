// FILE: app_def_par_function_of_my_gradientbis_of_b_spline_compute_o.rs
// occt: AppDef_ParFunctionOfMyGradientbisOfBSplineCompute
pub struct ParFunctionBis { pub value: f64 }
impl ParFunctionBis { pub fn new() -> Self { ParFunctionBis { value: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = ParFunctionBis::new(); } }
