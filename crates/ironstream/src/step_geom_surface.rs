// FILE: step_geom_surface.rs
// occt: StepGeom_Surface

/// Base class representing a surface in STEP format
pub struct StepGeomSurface {
    name: String,
    id: i32,
}

impl StepGeomSurface {
    pub fn new(name: String, id: i32) -> Self {
        StepGeomSurface { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface() {
        let surface = StepGeomSurface::new("Surface1".to_string(), 1);
        assert_eq!(surface.name(), "Surface1");
        assert_eq!(surface.id(), 1);
    }

    #[test]
    fn test_set_name() {
        let mut surface = StepGeomSurface::new("Surface1".to_string(), 1);
        surface.set_name("Surface2".to_string());
        assert_eq!(surface.name(), "Surface2");
    }
}
