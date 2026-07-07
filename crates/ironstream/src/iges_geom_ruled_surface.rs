// FILE: iges_geom_ruled_surface.rs
// occt: IGESGeom_RuledSurface

/// Defines IGESRuledSurface, Type <118> in package IGESGeom.
#[derive(Clone, Debug)]
pub struct RuledSurface {
    entity_type: i32,
}

impl RuledSurface {
    pub fn new() -> Self {
        RuledSurface { entity_type: 118 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for RuledSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let surface = RuledSurface::new();
        assert_eq!(surface.entity_type(), 118);
    }
}
