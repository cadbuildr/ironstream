// FILE: step_geom_reparametrised_composite_curve_segment.rs
// occt: StepGeom_ReparametrisedCompositeCurveSegment

/// Represents a segment of a composite curve with reparametrization
pub struct StepGeomReparametrisedCompositeCurveSegment {
    curve_id: i32,
    /// Parameter range start in the composite curve
    u_start: f64,
    /// Parameter range end in the composite curve
    u_end: f64,
    /// Sense (forward or reversed)
    sense: bool,
}

impl StepGeomReparametrisedCompositeCurveSegment {
    pub fn new(curve_id: i32, u_start: f64, u_end: f64, sense: bool) -> Self {
        StepGeomReparametrisedCompositeCurveSegment {
            curve_id,
            u_start,
            u_end,
            sense,
        }
    }

    pub fn curve_id(&self) -> i32 {
        self.curve_id
    }

    pub fn u_start(&self) -> f64 {
        self.u_start
    }

    pub fn u_end(&self) -> f64 {
        self.u_end
    }

    pub fn sense(&self) -> bool {
        self.sense
    }

    pub fn set_sense(&mut self, sense: bool) {
        self.sense = sense;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_segment() {
        let segment = StepGeomReparametrisedCompositeCurveSegment::new(1, 0.0, 1.0, true);
        assert_eq!(segment.curve_id(), 1);
        assert_eq!(segment.u_start(), 0.0);
        assert_eq!(segment.u_end(), 1.0);
        assert!(segment.sense());
    }

    #[test]
    fn test_set_sense() {
        let mut segment = StepGeomReparametrisedCompositeCurveSegment::new(1, 0.0, 1.0, true);
        segment.set_sense(false);
        assert!(!segment.sense());
    }
}
