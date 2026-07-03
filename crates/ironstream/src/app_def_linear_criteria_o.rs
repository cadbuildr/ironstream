// FILE: app_def_linear_criteria_o.rs
// occt: AppDef_LinearCriteria
pub struct LinearCriteria { pub tolerance: f64 }
impl LinearCriteria { pub fn new(tolerance: f64) -> Self { LinearCriteria { tolerance } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = LinearCriteria::new(1e-6); } }
