// FILE: step_kinematics_surface_pair_with_range.rs
// occt: StepKinematics_SurfacePairWithRange

/// Representation of STEP entity SurfacePairWithRange.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsSurfacePairWithRange;

impl StepKinematicsSurfacePairWithRange {
    pub fn new() -> Self {
        StepKinematicsSurfacePairWithRange
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsSurfacePairWithRange::new();
    }
}
