// FILE: topo_ds_to_step_make_geometric_curve_set.rs
// occt: TopoDSToStep_MakeGeometricCurveSet

pub struct MakeGeometricCurveSet {
    geometric_curve_set: Option<GeometricCurveSet>,
}

pub struct GeometricCurveSet;

impl MakeGeometricCurveSet {
    pub fn new() -> Self {
        MakeGeometricCurveSet {
            geometric_curve_set: None,
        }
    }

    pub fn value(&self) -> Option<&GeometricCurveSet> {
        self.geometric_curve_set.as_ref()
    }
}

impl Default for MakeGeometricCurveSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeGeometricCurveSet::new();
        assert!(maker.value().is_none());
    }
}
