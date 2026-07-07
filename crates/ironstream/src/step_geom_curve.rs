// FILE: step_geom_curve.rs
// occt: StepGeom_Curve

use std::sync::Arc;

#[derive(Clone)]
pub struct Curve {
    name: Arc<String>,
}

impl Curve {
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

impl Default for Curve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let curve = Curve::new();
        assert_eq!(curve.name(), "");
    }

    #[test]
    fn test_init() {
        let mut curve = Curve::new();
        curve.init("curve".to_string());
        assert_eq!(curve.name(), "curve");
    }
}
