// FILE: step_geom_circle.rs
// occt: StepGeom_Circle

use std::sync::Arc;

#[derive(Clone)]
pub struct Circle {
    name: Arc<String>,
    position: Option<Arc<String>>,
    radius: f64,
}

impl Circle {
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

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let circle = Circle::new();
        assert_eq!(circle.radius(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut circle = Circle::new();
        circle.init("circle1".to_string(), None, 5.0);
        assert_eq!(circle.name(), "circle1");
        assert_eq!(circle.radius(), 5.0);
    }
}
