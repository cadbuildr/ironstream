// FILE: step_geom_axis1_placement.rs
// occt: StepGeom_Axis1Placement

//! Represents a 1-axis placement in 3D space.

use std::rc::Rc;

/// Cartesian point in 3D
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

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
}

/// Direction vector
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

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
}

/// Placement base
#[derive(Debug, Clone)]
pub struct Placement {
    name: Option<String>,
    location: Option<Rc<CartesianPoint>>,
}

impl Placement {
    pub fn new() -> Self {
        Self {
            name: None,
            location: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_location(&mut self, location: Rc<CartesianPoint>) {
        self.location = Some(location);
    }

    pub fn location(&self) -> Option<&Rc<CartesianPoint>> {
        self.location.as_ref()
    }
}

impl Default for Placement {
    fn default() -> Self {
        Self::new()
    }
}

/// Axis1Placement extends Placement with an optional axis
#[derive(Debug, Clone)]
pub struct StepGeomAxis1Placement {
    placement: Placement,
    axis: Option<Rc<Direction>>,
    has_axis: bool,
}

impl StepGeomAxis1Placement {
    pub fn new() -> Self {
        Self {
            placement: Placement::new(),
            axis: None,
            has_axis: false,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        location: Rc<CartesianPoint>,
        has_axis: bool,
        axis: Option<Rc<Direction>>,
    ) {
        self.placement.set_name(name);
        self.placement.set_location(location);
        self.has_axis = has_axis;
        self.axis = axis;
    }

    pub fn name(&self) -> Option<&str> {
        self.placement.name()
    }

    pub fn location(&self) -> Option<&Rc<CartesianPoint>> {
        self.placement.location()
    }

    pub fn set_axis(&mut self, axis: Rc<Direction>) {
        self.axis = Some(axis);
        self.has_axis = true;
    }

    pub fn unset_axis(&mut self) {
        self.axis = None;
        self.has_axis = false;
    }

    pub fn axis(&self) -> Option<&Rc<Direction>> {
        self.axis.as_ref()
    }

    pub fn has_axis(&self) -> bool {
        self.has_axis
    }
}

impl Default for StepGeomAxis1Placement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_point() {
        let pt = CartesianPoint::new(1.0, 2.0, 3.0);
        assert_eq!(pt.x(), 1.0);
        assert_eq!(pt.y(), 2.0);
        assert_eq!(pt.z(), 3.0);
    }

    #[test]
    fn test_direction() {
        let dir = Direction::new(0.0, 0.0, 1.0);
        assert_eq!(dir.z(), 1.0);
    }

    #[test]
    fn test_new() {
        let a1p = StepGeomAxis1Placement::new();
        assert!(!a1p.has_axis());
        assert_eq!(a1p.name(), None);
    }

    #[test]
    fn test_init() {
        let mut a1p = StepGeomAxis1Placement::new();
        let loc = Rc::new(CartesianPoint::new(0.0, 0.0, 0.0));
        let axis = Rc::new(Direction::new(0.0, 0.0, 1.0));
        a1p.init("axis1".to_string(), loc, true, Some(axis));
        assert!(a1p.has_axis());
        assert_eq!(a1p.name(), Some("axis1"));
    }

    #[test]
    fn test_set_axis() {
        let mut a1p = StepGeomAxis1Placement::new();
        let axis = Rc::new(Direction::new(1.0, 0.0, 0.0));
        a1p.set_axis(axis);
        assert!(a1p.has_axis());
    }

    #[test]
    fn test_unset_axis() {
        let mut a1p = StepGeomAxis1Placement::new();
        let axis = Rc::new(Direction::new(0.0, 1.0, 0.0));
        a1p.set_axis(axis);
        assert!(a1p.has_axis());
        a1p.unset_axis();
        assert!(!a1p.has_axis());
    }
}
