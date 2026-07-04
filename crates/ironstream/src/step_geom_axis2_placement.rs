// FILE: step_geom_axis2_placement.rs
// occt: StepGeom_Axis2Placement

//! Represents a 2-axis placement in 3D space.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CartesianPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl CartesianPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct Direction {
    x: f64,
    y: f64,
    z: f64,
}

impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct StepGeomAxis2Placement {
    name: Option<String>,
    location: Option<Rc<CartesianPoint>>,
    main_axis: Option<Rc<Direction>>,
    ref_axis: Option<Rc<Direction>>,
}

impl StepGeomAxis2Placement {
    pub fn new() -> Self {
        Self {
            name: None,
            location: None,
            main_axis: None,
            ref_axis: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        location: Rc<CartesianPoint>,
        main_axis: Rc<Direction>,
        ref_axis: Rc<Direction>,
    ) {
        self.name = Some(name);
        self.location = Some(location);
        self.main_axis = Some(main_axis);
        self.ref_axis = Some(ref_axis);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn main_axis(&self) -> Option<&Rc<Direction>> {
        self.main_axis.as_ref()
    }

    pub fn ref_axis(&self) -> Option<&Rc<Direction>> {
        self.ref_axis.as_ref()
    }
}

impl Default for StepGeomAxis2Placement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a2p = StepGeomAxis2Placement::new();
        assert_eq!(a2p.name(), None);
    }

    #[test]
    fn test_init() {
        let mut a2p = StepGeomAxis2Placement::new();
        let loc = Rc::new(CartesianPoint::new(0.0, 0.0, 0.0));
        let main = Rc::new(Direction::new(0.0, 0.0, 1.0));
        let r_axis = Rc::new(Direction::new(1.0, 0.0, 0.0));
        a2p.init("axis2".to_string(), loc, main, r_axis);
        assert_eq!(a2p.name(), Some("axis2"));
    }
}
