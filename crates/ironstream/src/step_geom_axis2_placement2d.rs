// FILE: step_geom_axis2_placement2d.rs
// occt: StepGeom_Axis2Placement2d

//! Represents a 2-axis placement in 2D space.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CartesianPoint2d {
    x: f64,
    y: f64,
}

impl CartesianPoint2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Direction2d {
    x: f64,
    y: f64,
}

impl Direction2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct StepGeomAxis2Placement2d {
    name: Option<String>,
    location: Option<Rc<CartesianPoint2d>>,
    ref_direction: Option<Rc<Direction2d>>,
}

impl StepGeomAxis2Placement2d {
    pub fn new() -> Self {
        Self {
            name: None,
            location: None,
            ref_direction: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        location: Rc<CartesianPoint2d>,
        ref_direction: Rc<Direction2d>,
    ) {
        self.name = Some(name);
        self.location = Some(location);
        self.ref_direction = Some(ref_direction);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn location(&self) -> Option<&Rc<CartesianPoint2d>> {
        self.location.as_ref()
    }

    pub fn ref_direction(&self) -> Option<&Rc<Direction2d>> {
        self.ref_direction.as_ref()
    }
}

impl Default for StepGeomAxis2Placement2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a2p = StepGeomAxis2Placement2d::new();
        assert_eq!(a2p.name(), None);
    }

    #[test]
    fn test_cartesian_point_2d() {
        let pt = CartesianPoint2d::new(1.5, 2.5);
        assert_eq!(pt.x, 1.5);
    }

    #[test]
    fn test_init() {
        let mut a2p = StepGeomAxis2Placement2d::new();
        let loc = Rc::new(CartesianPoint2d::new(0.0, 0.0));
        let dir = Rc::new(Direction2d::new(1.0, 0.0));
        a2p.init("axis2d".to_string(), loc, dir);
        assert_eq!(a2p.name(), Some("axis2d"));
    }
}
