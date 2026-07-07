// FILE: step_geom_oriented_surface.rs
// occt: StepGeom_OrientedSurface

//! Represents a surface with an orientation.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Surface {
    id: String,
}

impl Surface {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Forward,
    Reversed,
}

impl Orientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Orientation::Forward => "FORWARD",
            Orientation::Reversed => "REVERSED",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StepGeomOrientedSurface {
    name: Option<String>,
    base_surface: Option<Rc<Surface>>,
    orientation: Orientation,
}

impl StepGeomOrientedSurface {
    pub fn new() -> Self {
        Self {
            name: None,
            base_surface: None,
            orientation: Orientation::Forward,
        }
    }

    pub fn init(&mut self, name: String, base_surface: Rc<Surface>, orientation: Orientation) {
        self.name = Some(name);
        self.base_surface = Some(base_surface);
        self.orientation = orientation;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
}

impl Default for StepGeomOrientedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let os = StepGeomOrientedSurface::new();
        assert_eq!(os.name(), None);
        assert_eq!(os.orientation(), Orientation::Forward);
    }

    #[test]
    fn test_init() {
        let mut os = StepGeomOrientedSurface::new();
        let surf = Rc::new(Surface::new("SURF".to_string()));
        os.init("oriented".to_string(), surf, Orientation::Reversed);
        assert_eq!(os.orientation(), Orientation::Reversed);
    }

    #[test]
    fn test_orientation() {
        assert_eq!(Orientation::Forward.as_str(), "FORWARD");
        assert_eq!(Orientation::Reversed.as_str(), "REVERSED");
    }
}
