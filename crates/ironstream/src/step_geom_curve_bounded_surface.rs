// FILE: step_geom_curve_bounded_surface.rs
// occt: StepGeom_CurveBoundedSurface

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Surface;

#[derive(Clone)]
pub struct BoundaryEdgeLoop;

#[derive(Clone)]
pub struct CurveBoundedSurface {
    name: Arc<String>,
    basis_surface: Option<Arc<Mutex<Surface>>>,
    boundaries: Option<Vec<Arc<Mutex<BoundaryEdgeLoop>>>>,
    implicit_outer: bool,
}

impl CurveBoundedSurface {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            basis_surface: None,
            boundaries: None,
            implicit_outer: false,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        basis_surface: Option<Arc<Mutex<Surface>>>,
        boundaries: Option<Vec<Arc<Mutex<BoundaryEdgeLoop>>>>,
        implicit_outer: bool,
    ) {
        self.name = Arc::new(name);
        self.basis_surface = basis_surface;
        self.boundaries = boundaries;
        self.implicit_outer = implicit_outer;
    }

    pub fn set_basis_surface(&mut self, surf: Arc<Mutex<Surface>>) {
        self.basis_surface = Some(surf);
    }

    pub fn basis_surface(&self) -> Option<Arc<Mutex<Surface>>> {
        self.basis_surface.clone()
    }

    pub fn set_boundaries(&mut self, bounds: Vec<Arc<Mutex<BoundaryEdgeLoop>>>) {
        self.boundaries = Some(bounds);
    }

    pub fn boundaries(&self) -> Option<Vec<Arc<Mutex<BoundaryEdgeLoop>>>> {
        self.boundaries.clone()
    }

    pub fn boundaries_value(&self, num: i32) -> Option<Arc<Mutex<BoundaryEdgeLoop>>> {
        self.boundaries
            .as_ref()
            .and_then(|b| b.get((num - 1) as usize).cloned())
    }

    pub fn nb_boundaries(&self) -> i32 {
        self.boundaries.as_ref().map_or(0, |b| b.len() as i32)
    }

    pub fn set_implicit_outer(&mut self, implicit: bool) {
        self.implicit_outer = implicit;
    }

    pub fn implicit_outer(&self) -> bool {
        self.implicit_outer
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for CurveBoundedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cbs = CurveBoundedSurface::new();
        assert!(!cbs.implicit_outer());
    }

    #[test]
    fn test_init() {
        let mut cbs = CurveBoundedSurface::new();
        cbs.init("cbs".to_string(), None, None, true);
        assert!(cbs.implicit_outer());
    }
}
