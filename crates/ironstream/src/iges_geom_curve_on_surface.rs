// FILE: iges_geom_curve_on_surface.rs
// occt: IGESGeom_Ucurveonsurface

pub struct UcurveUonUsurface {
    entity_type: i32,
}

impl UcurveUonUsurface {
    pub fn new() -> Self {
        UcurveUonUsurface { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UcurveUonUsurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UcurveUonUsurface::new();
    }
}
