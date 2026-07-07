// FILE: step_geom_offset_curve3d.rs
// occt: StepGeom_OffsetCurve3d

//! Represents a 3D curve offset from a base curve.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Curve {
    id: String,
}

impl Curve {
    pub fn new(id: String) -> Self {
        Self { id }
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
pub struct StepGeomOffsetCurve3d {
    name: Option<String>,
    base_curve: Option<Rc<Curve>>,
    distance: f64,
    self_intersect: bool,
    ref_direction: Option<Rc<Direction>>,
}

impl StepGeomOffsetCurve3d {
    pub fn new() -> Self {
        Self {
            name: None,
            base_curve: None,
            distance: 0.0,
            self_intersect: false,
            ref_direction: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        base_curve: Rc<Curve>,
        distance: f64,
        self_intersect: bool,
        ref_direction: Rc<Direction>,
    ) {
        self.name = Some(name);
        self.base_curve = Some(base_curve);
        self.distance = distance;
        self.self_intersect = self_intersect;
        self.ref_direction = Some(ref_direction);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn self_intersect(&self) -> bool {
        self.self_intersect
    }
}

impl Default for StepGeomOffsetCurve3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let oc = StepGeomOffsetCurve3d::new();
        assert_eq!(oc.name(), None);
    }

    #[test]
    fn test_init() {
        let mut oc = StepGeomOffsetCurve3d::new();
        let base = Rc::new(Curve::new("CURVE".to_string()));
        let dir = Rc::new(Direction::new(0.0, 0.0, 1.0));
        oc.init(
            "offset".to_string(),
            base,
            1.5,
            false,
            dir,
        );
        assert_eq!(oc.distance(), 1.5);
        assert!(!oc.self_intersect());
    }
}
