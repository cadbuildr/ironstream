// FILE: iges_geom_b_spline_surface.rs
// occt: IGESGeom_BSplineSurface

pub struct BSplineSurface {
    entity_type: i32,
}

impl BSplineSurface {
    pub fn new() -> Self {
        BSplineSurface { entity_type: 128 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for BSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let surface = BSplineSurface::new();
        assert_eq!(surface.entity_type(), 128);
    }
}
