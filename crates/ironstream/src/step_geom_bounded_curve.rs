// FILE: step_geom_bounded_curve.rs
// occt: StepGeom_BoundedCurve

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct BoundedCurve {
    name: Arc<String>,
}

impl BoundedCurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
        }
    }

    pub fn init(&mut self, name: String) {
        self.name = Arc::new(name);
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for BoundedCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let curve = BoundedCurve::new();
        assert_eq!(curve.name(), "");
    }

    #[test]
    fn test_init() {
        let mut curve = BoundedCurve::new();
        curve.init("bounded".to_string());
        assert_eq!(curve.name(), "bounded");
    }
}
