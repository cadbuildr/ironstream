// FILE: step_geom_cylindrical_surface.rs
// occt: StepGeom_CylindricalSurface

use std::sync::Arc;

#[derive(Clone)]
pub struct CylindricalSurface {
    name: Arc<String>,
    position: Option<Arc<String>>,
    radius: f64,
}

impl CylindricalSurface {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            position: None,
            radius: 0.0,
        }
    }

    pub fn init(&mut self, name: String, position: Option<String>, radius: f64) {
        self.name = Arc::new(name);
        self.position = position.map(|p| Arc::new(p));
        self.radius = radius;
    }

    pub fn set_position(&mut self, position: String) {
        self.position = Some(Arc::new(position));
    }

    pub fn position(&self) -> Option<String> {
        self.position.as_ref().map(|p| p.as_ref().clone())
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for CylindricalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cyl = CylindricalSurface::new();
        assert_eq!(cyl.radius(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut cyl = CylindricalSurface::new();
        cyl.init("cylinder".to_string(), None, 5.0);
        assert_eq!(cyl.radius(), 5.0);
    }
}
