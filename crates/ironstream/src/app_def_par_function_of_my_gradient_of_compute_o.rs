// FILE: app_def_par_function_of_my_gradient_of_compute_o.rs
// occt: AppDef_ParFunctionOfMyGradientOfCompute
pub struct ParFunction { pub value: f64 }
impl ParFunction { pub fn new() -> Self { ParFunction { value: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = ParFunction::new(); } }
