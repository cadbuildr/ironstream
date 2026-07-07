// FILE: step_geom_composite_curve_on_surface.rs
// occt: StepGeom_CompositeCurveOnSurface

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CompositeCurveSegment;

#[derive(Clone)]
pub struct Surface;

#[derive(Clone)]
pub struct CompositeCurveOnSurface {
    name: Arc<String>,
    segments: Option<Vec<Arc<Mutex<CompositeCurveSegment>>>>,
    basis_surface: Option<Arc<Mutex<Surface>>>,
    self_intersect: bool,
}

impl CompositeCurveOnSurface {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            segments: None,
            basis_surface: None,
            self_intersect: false,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        segments: Option<Vec<Arc<Mutex<CompositeCurveSegment>>>>,
        basis_surface: Option<Arc<Mutex<Surface>>>,
        self_intersect: bool,
    ) {
        self.name = Arc::new(name);
        self.segments = segments;
        self.basis_surface = basis_surface;
        self.self_intersect = self_intersect;
    }

    pub fn set_basis_surface(&mut self, surf: Arc<Mutex<Surface>>) {
        self.basis_surface = Some(surf);
    }

    pub fn basis_surface(&self) -> Option<Arc<Mutex<Surface>>> {
        self.basis_surface.clone()
    }

    pub fn set_self_intersect(&mut self, intersect: bool) {
        self.self_intersect = intersect;
    }

    pub fn self_intersect(&self) -> bool {
        self.self_intersect
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for CompositeCurveOnSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cc = CompositeCurveOnSurface::new();
        assert!(!cc.self_intersect());
    }

    #[test]
    fn test_init() {
        let mut cc = CompositeCurveOnSurface::new();
        cc.init("curve_on_surf".to_string(), None, None, false);
        assert_eq!(cc.name(), "curve_on_surf");
    }
}
