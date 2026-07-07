// FILE: step_geom_line.rs
// occt: StepGeom_Line

//! Represents a line in 3D space.

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
pub struct StepGeomLine {
    name: Option<String>,
    point: Option<Rc<CartesianPoint>>,
    direction: Option<Rc<Direction>>,
}

impl StepGeomLine {
    pub fn new() -> Self {
        Self {
            name: None,
            point: None,
            direction: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        point: Rc<CartesianPoint>,
        direction: Rc<Direction>,
    ) {
        self.name = Some(name);
        self.point = Some(point);
        self.direction = Some(direction);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn point(&self) -> Option<&Rc<CartesianPoint>> {
        self.point.as_ref()
    }

    pub fn direction(&self) -> Option<&Rc<Direction>> {
        self.direction.as_ref()
    }
}

impl Default for StepGeomLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let line = StepGeomLine::new();
        assert_eq!(line.name(), None);
    }

    #[test]
    fn test_init() {
        let mut line = StepGeomLine::new();
        let pt = Rc::new(CartesianPoint::new(0.0, 0.0, 0.0));
        let dir = Rc::new(Direction::new(1.0, 0.0, 0.0));
        line.init("line1".to_string(), pt, dir);
        assert_eq!(line.name(), Some("line1"));
    }
}
