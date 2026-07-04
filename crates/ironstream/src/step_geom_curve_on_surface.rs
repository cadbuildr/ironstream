// FILE: step_geom_curve_on_surface.rs
// occt: StepGeom_CurveOnSurface

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CurveOnSurface {
    name: Arc<String>,
}

impl CurveOnSurface {
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

impl Default for CurveOnSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cos = CurveOnSurface::new();
        assert_eq!(cos.name(), "");
    }
}
