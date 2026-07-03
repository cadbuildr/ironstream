// FILE: app_def_my_gradientbis_of_b_spline_compute_o.rs
// occt: AppDef_MyGradientbisOfBSplineCompute
pub struct MyGradientBis { pub norm: f64 }
impl MyGradientBis { pub fn new() -> Self { MyGradientBis { norm: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = MyGradientBis::new(); } }
