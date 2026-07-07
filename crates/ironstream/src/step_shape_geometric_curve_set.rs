// FILE: step_shape_geometric_curve_set.rs
// occt: StepShape_GeometricCurveSet

//! Representation of STEP entity GeometricCurveSet

#[derive(Clone, Debug)]
pub struct GeometricCurveSet {
    // Inherits from GeometricSet
}

impl GeometricCurveSet {
    /// Returns a GeometricCurveSet
    pub fn new() -> Self {
        GeometricCurveSet {}
    }
}

impl Default for GeometricCurveSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = GeometricCurveSet::new();
        let _ = set;
    }

    #[test]
    fn test_default() {
        let set = GeometricCurveSet::default();
        let _ = set;
    }
}
