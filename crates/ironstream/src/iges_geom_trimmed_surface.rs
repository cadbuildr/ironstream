// FILE: iges_geom_trimmed_surface.rs
// occt: IGESGeom_Utrimmedsurface

pub struct UtrimmedUsurface {
    entity_type: i32,
}

impl UtrimmedUsurface {
    pub fn new() -> Self {
        UtrimmedUsurface { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UtrimmedUsurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UtrimmedUsurface::new();
    }
}
