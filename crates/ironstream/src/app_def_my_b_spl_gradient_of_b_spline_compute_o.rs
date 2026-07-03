// FILE: app_def_my_b_spl_gradient_of_b_spline_compute_o.rs
// occt: AppDef_MyBSplGradientOfBSplineCompute
pub struct MyBSplGradient { pub norm: f64 }
impl MyBSplGradient { pub fn new() -> Self { MyBSplGradient { norm: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = MyBSplGradient::new(); } }
