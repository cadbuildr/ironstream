// FILE: iges_geom_composite_curve.rs
// occt: IGESGeom_Ucompositecurve

pub struct UcompositeUcurve {
    entity_type: i32,
}

impl UcompositeUcurve {
    pub fn new() -> Self {
        UcompositeUcurve { entity_type: 0 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for UcompositeUcurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = UcompositeUcurve::new();
    }
}
