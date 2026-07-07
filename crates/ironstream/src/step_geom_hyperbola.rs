// FILE: step_geom_hyperbola.rs
// occt: StepGeom_Hyperbola

//! Represents a hyperbola curve in 3D space.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Axis2Placement {
    x: f64,
    y: f64,
    z: f64,
}

impl Axis2Placement {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct StepGeomHyperbola {
    name: Option<String>,
    position: Option<Rc<Axis2Placement>>,
    semi_major_axis: f64,
    semi_minor_axis: f64,
}

impl StepGeomHyperbola {
    pub fn new() -> Self {
        Self {
            name: None,
            position: None,
            semi_major_axis: 0.0,
            semi_minor_axis: 0.0,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        position: Rc<Axis2Placement>,
        semi_major: f64,
        semi_minor: f64,
    ) {
        self.name = Some(name);
        self.position = Some(position);
        self.semi_major_axis = semi_major;
        self.semi_minor_axis = semi_minor;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    pub fn semi_minor_axis(&self) -> f64 {
        self.semi_minor_axis
    }
}

impl Default for StepGeomHyperbola {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let h = StepGeomHyperbola::new();
        assert_eq!(h.name(), None);
    }

    #[test]
    fn test_init() {
        let mut h = StepGeomHyperbola::new();
        let pos = Rc::new(Axis2Placement::new(0.0, 0.0, 0.0));
        h.init("hyperbola".to_string(), pos, 5.0, 3.0);
        assert_eq!(h.semi_major_axis(), 5.0);
        assert_eq!(h.semi_minor_axis(), 3.0);
    }
}
