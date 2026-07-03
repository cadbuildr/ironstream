// FILE: app_def_smooth_criterion_o.rs
// occt: AppDef_SmoothCriterion
pub struct SmoothCriterion { pub weight: f64 }
impl SmoothCriterion { pub fn new(weight: f64) -> Self { SmoothCriterion { weight } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = SmoothCriterion::new(0.5); } }
