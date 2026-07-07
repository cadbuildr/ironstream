// FILE: iges_geom_surface_of_revolution.rs
// occt: IGESGeom_SurfaceOfRevolution

/// Defines IGESSurfaceOfRevolution, Type <120> in package IGESGeom.
#[derive(Clone, Debug)]
pub struct SurfaceOfRevolution {
    entity_type: i32,
}

impl SurfaceOfRevolution {
    pub fn new() -> Self {
        SurfaceOfRevolution { entity_type: 120 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for SurfaceOfRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let surface = SurfaceOfRevolution::new();
        assert_eq!(surface.entity_type(), 120);
    }
}
