// FILE: step_geom_offset_surface.rs
// occt: StepGeom_OffsetSurface

//! Represents a surface offset from a base surface.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Surface {
    id: String,
}

impl Surface {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone)]
pub struct StepGeomOffsetSurface {
    name: Option<String>,
    base_surface: Option<Rc<Surface>>,
    offset_value: f64,
    self_intersect: bool,
}

impl StepGeomOffsetSurface {
    pub fn new() -> Self {
        Self {
            name: None,
            base_surface: None,
            offset_value: 0.0,
            self_intersect: false,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        base_surface: Rc<Surface>,
        offset_value: f64,
        self_intersect: bool,
    ) {
        self.name = Some(name);
        self.base_surface = Some(base_surface);
        self.offset_value = offset_value;
        self.self_intersect = self_intersect;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn offset_value(&self) -> f64 {
        self.offset_value
    }

    pub fn self_intersect(&self) -> bool {
        self.self_intersect
    }
}

impl Default for StepGeomOffsetSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let os = StepGeomOffsetSurface::new();
        assert_eq!(os.name(), None);
    }

    #[test]
    fn test_init() {
        let mut os = StepGeomOffsetSurface::new();
        let surf = Rc::new(Surface::new("SURFACE".to_string()));
        os.init("offset_surf".to_string(), surf, 2.0, true);
        assert_eq!(os.offset_value(), 2.0);
        assert!(os.self_intersect());
    }
}
