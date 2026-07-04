// FILE: step_geom_boundary_curve.rs
// occt: StepGeom_BoundaryCurve

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Curve;

#[derive(Clone)]
pub struct Surface;

#[derive(Clone)]
pub struct BoundaryCurve {
    name: Arc<String>,
    curve_3d: Option<Arc<Mutex<Curve>>>,
    surfaces_list: Option<Vec<Arc<Mutex<Surface>>>>,
}

impl BoundaryCurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            curve_3d: None,
            surfaces_list: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        curve_3d: Option<Arc<Mutex<Curve>>>,
        surfaces_list: Option<Vec<Arc<Mutex<Surface>>>>,
    ) {
        self.name = Arc::new(name);
        self.curve_3d = curve_3d;
        self.surfaces_list = surfaces_list;
    }

    pub fn set_curve_3d(&mut self, curve: Arc<Mutex<Curve>>) {
        self.curve_3d = Some(curve);
    }

    pub fn curve_3d(&self) -> Option<Arc<Mutex<Curve>>> {
        self.curve_3d.clone()
    }

    pub fn set_surfaces_list(&mut self, surfaces: Vec<Arc<Mutex<Surface>>>) {
        self.surfaces_list = Some(surfaces);
    }

    pub fn surfaces_list(&self) -> Option<Vec<Arc<Mutex<Surface>>>> {
        self.surfaces_list.clone()
    }

    pub fn surfaces_list_value(&self, num: i32) -> Option<Arc<Mutex<Surface>>> {
        self.surfaces_list
            .as_ref()
            .and_then(|s| s.get((num - 1) as usize).cloned())
    }

    pub fn nb_surfaces_list(&self) -> i32 {
        self.surfaces_list.as_ref().map_or(0, |s| s.len() as i32)
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for BoundaryCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let bc = BoundaryCurve::new();
        assert_eq!(bc.name(), "");
    }

    #[test]
    fn test_surfaces_list() {
        let mut bc = BoundaryCurve::new();
        bc.set_surfaces_list(vec![
            Arc::new(Mutex::new(Surface)),
            Arc::new(Mutex::new(Surface)),
        ]);
        assert_eq!(bc.nb_surfaces_list(), 2);
    }
}
