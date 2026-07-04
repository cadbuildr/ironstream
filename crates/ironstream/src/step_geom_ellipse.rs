// FILE: step_geom_ellipse.rs
// occt: StepGeom_Ellipse

use std::sync::Arc;

#[derive(Clone)]
pub struct Ellipse {
    name: Arc<String>,
    position: Option<Arc<String>>,
    semi_major_axis: f64,
    semi_minor_axis: f64,
}

impl Ellipse {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            position: None,
            semi_major_axis: 0.0,
            semi_minor_axis: 0.0,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        position: Option<String>,
        semi_major_axis: f64,
        semi_minor_axis: f64,
    ) {
        self.name = Arc::new(name);
        self.position = position.map(|p| Arc::new(p));
        self.semi_major_axis = semi_major_axis;
        self.semi_minor_axis = semi_minor_axis;
    }

    pub fn set_position(&mut self, position: String) {
        self.position = Some(Arc::new(position));
    }

    pub fn position(&self) -> Option<String> {
        self.position.as_ref().map(|p| p.as_ref().clone())
    }

    pub fn set_semi_major_axis(&mut self, axis: f64) {
        self.semi_major_axis = axis;
    }

    pub fn semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    pub fn set_semi_minor_axis(&mut self, axis: f64) {
        self.semi_minor_axis = axis;
    }

    pub fn semi_minor_axis(&self) -> f64 {
        self.semi_minor_axis
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for Ellipse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ellipse = Ellipse::new();
        assert_eq!(ellipse.semi_major_axis(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut ellipse = Ellipse::new();
        ellipse.init("ellipse1".to_string(), None, 10.0, 5.0);
        assert_eq!(ellipse.semi_major_axis(), 10.0);
        assert_eq!(ellipse.semi_minor_axis(), 5.0);
    }
}
