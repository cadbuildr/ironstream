// FILE: iges_geom_b_spline_curve.rs
// occt: IGESGeom_BSplineCurve

pub struct BSplineCurve {
    entity_type: i32,
}

impl BSplineCurve {
    pub fn new() -> Self {
        BSplineCurve { entity_type: 106 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for BSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = BSplineCurve::new();
        assert_eq!(curve.entity_type(), 106);
    }
}
