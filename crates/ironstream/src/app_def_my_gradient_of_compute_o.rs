// FILE: app_def_my_gradient_of_compute_o.rs
// occt: AppDef_MyGradientOfCompute
pub struct MyGradient { pub norm: f64 }
impl MyGradient { pub fn new() -> Self { MyGradient { norm: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = MyGradient::new(); } }
