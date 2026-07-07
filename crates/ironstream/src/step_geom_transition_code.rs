// FILE: step_geom_transition_code.rs
// occt: StepGeom_TransitionCode

/// Enumeration for transition codes between curve segments
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepGeomTransitionCode {
    /// Discontinuous transition
    Discontinuous = 0,
    /// Continuous transition
    Continuous = 1,
    /// Continuous with continuous derivative
    ContWithContDeriv = 2,
    /// Continuous with continuous second derivative
    ContWithContDerivAndContinuousCurvature = 3,
}

impl StepGeomTransitionCode {
    pub fn is_discontinuous(&self) -> bool {
        matches!(self, StepGeomTransitionCode::Discontinuous)
    }

    pub fn is_continuous(&self) -> bool {
        matches!(self, StepGeomTransitionCode::Continuous)
    }

    pub fn is_smooth(&self) -> bool {
        matches!(
            self,
            StepGeomTransitionCode::ContWithContDeriv
                | StepGeomTransitionCode::ContWithContDerivAndContinuousCurvature
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discontinuous() {
        let code = StepGeomTransitionCode::Discontinuous;
        assert!(code.is_discontinuous());
        assert!(!code.is_continuous());
    }

    #[test]
    fn test_continuous() {
        let code = StepGeomTransitionCode::Continuous;
        assert!(code.is_continuous());
        assert!(!code.is_discontinuous());
    }

    #[test]
    fn test_smooth() {
        let code = StepGeomTransitionCode::ContWithContDeriv;
        assert!(code.is_smooth());
    }
}
