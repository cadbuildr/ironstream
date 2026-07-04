// FILE: step_geom_bounded_surface.rs
// occt: StepGeom_BoundedSurface

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct BoundedSurface {
    name: Arc<String>,
}

impl BoundedSurface {
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

impl Default for BoundedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = BoundedSurface::new();
        assert_eq!(surf.name(), "");
    }

    #[test]
    fn test_init() {
        let mut surf = BoundedSurface::new();
        surf.init("bounded_surf".to_string());
        assert_eq!(surf.name(), "bounded_surf");
    }
}
