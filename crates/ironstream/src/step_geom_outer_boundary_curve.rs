// FILE: step_geom_outer_boundary_curve.rs
// occt: StepGeom_OuterBoundaryCurve

//! Represents the outer boundary curve of a surface.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Curve {
    id: String,
}

impl Curve {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone)]
pub struct StepGeomOuterBoundaryCurve {
    name: Option<String>,
    curve: Option<Rc<Curve>>,
}

impl StepGeomOuterBoundaryCurve {
    pub fn new() -> Self {
        Self {
            name: None,
            curve: None,
        }
    }

    pub fn init(&mut self, name: String, curve: Rc<Curve>) {
        self.name = Some(name);
        self.curve = Some(curve);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn curve(&self) -> Option<&Rc<Curve>> {
        self.curve.as_ref()
    }
}

impl Default for StepGeomOuterBoundaryCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let obc = StepGeomOuterBoundaryCurve::new();
        assert_eq!(obc.name(), None);
    }

    #[test]
    fn test_init() {
        let mut obc = StepGeomOuterBoundaryCurve::new();
        let curve = Rc::new(Curve::new("OUTER_CURVE".to_string()));
        obc.init("boundary".to_string(), curve);
        assert_eq!(obc.name(), Some("boundary"));
    }

    #[test]
    fn test_curve() {
        let curve = Curve::new("C1".to_string());
        assert_eq!(curve.id(), "C1");
    }
}
