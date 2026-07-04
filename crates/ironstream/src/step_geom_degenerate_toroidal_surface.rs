// FILE: step_geom_degenerate_toroidal_surface.rs
// occt: StepGeom_DegenerateToroidalSurface

use std::sync::Arc;

#[derive(Clone)]
pub struct DegenerateToroidalSurface {
    name: Arc<String>,
    position: Option<Arc<String>>,
    major_radius: f64,
    minor_radius: f64,
}

impl DegenerateToroidalSurface {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            position: None,
            major_radius: 0.0,
            minor_radius: 0.0,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        position: Option<String>,
        major_radius: f64,
        minor_radius: f64,
    ) {
        self.name = Arc::new(name);
        self.position = position.map(|p| Arc::new(p));
        self.major_radius = major_radius;
        self.minor_radius = minor_radius;
    }

    pub fn set_position(&mut self, position: String) {
        self.position = Some(Arc::new(position));
    }

    pub fn position(&self) -> Option<String> {
        self.position.as_ref().map(|p| p.as_ref().clone())
    }

    pub fn set_major_radius(&mut self, radius: f64) {
        self.major_radius = radius;
    }

    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    pub fn set_minor_radius(&mut self, radius: f64) {
        self.minor_radius = radius;
    }

    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for DegenerateToroidalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let dts = DegenerateToroidalSurface::new();
        assert_eq!(dts.major_radius(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut dts = DegenerateToroidalSurface::new();
        dts.init("torus".to_string(), None, 10.0, 2.0);
        assert_eq!(dts.major_radius(), 10.0);
        assert_eq!(dts.minor_radius(), 2.0);
    }
}
