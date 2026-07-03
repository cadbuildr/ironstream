// FILE: app_def_par_function_of_the_gradient_o.rs
// occt: AppDef_ParFunctionOfTheGradient
pub struct TheParFunction { pub value: f64 }
impl TheParFunction { pub fn new() -> Self { TheParFunction { value: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = TheParFunction::new(); } }
