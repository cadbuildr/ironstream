// FILE: step_geom_conical_surface.rs
// occt: StepGeom_ConicalSurface

use std::sync::Arc;

#[derive(Clone)]
pub struct ConicalSurface {
    name: Arc<String>,
    position: Option<Arc<String>>,
    radius: f64,
    semi_vertical_angle: f64,
}

impl ConicalSurface {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            position: None,
            radius: 0.0,
            semi_vertical_angle: 0.0,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        position: Option<String>,
        radius: f64,
        semi_vertical_angle: f64,
    ) {
        self.name = Arc::new(name);
        self.position = position.map(|p| Arc::new(p));
        self.radius = radius;
        self.semi_vertical_angle = semi_vertical_angle;
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

    pub fn set_semi_vertical_angle(&mut self, angle: f64) {
        self.semi_vertical_angle = angle;
    }

    pub fn semi_vertical_angle(&self) -> f64 {
        self.semi_vertical_angle
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for ConicalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = ConicalSurface::new();
        assert_eq!(surf.radius(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut surf = ConicalSurface::new();
        surf.init("cone".to_string(), None, 10.0, 0.5);
        assert_eq!(surf.name(), "cone");
        assert_eq!(surf.radius(), 10.0);
    }
}
