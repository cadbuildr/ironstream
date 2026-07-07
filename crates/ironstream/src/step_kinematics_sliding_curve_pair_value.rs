// FILE: step_kinematics_sliding_curve_pair_value.rs
// occt: StepKinematics_SlidingCurvePairValue

/// Representation of STEP entity SlidingCurvePairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsSlidingCurvePairValue {
    actual_point_on_curve1: (),
    actual_point_on_curve2: (),
}

impl Default for StepKinematicsSlidingCurvePairValue {
    fn default() -> Self {
        StepKinematicsSlidingCurvePairValue {
            actual_point_on_curve1: (),
            actual_point_on_curve2: (),
        }
    }
}

impl StepKinematicsSlidingCurvePairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actual_point_on_curve1(&self) -> () {
        self.actual_point_on_curve1
    }

    pub fn set_actual_point_on_curve1(&mut self, _p: ()) {
        self.actual_point_on_curve1 = ();
    }

    pub fn actual_point_on_curve2(&self) -> () {
        self.actual_point_on_curve2
    }

    pub fn set_actual_point_on_curve2(&mut self, _p: ()) {
        self.actual_point_on_curve2 = ();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _value = StepKinematicsSlidingCurvePairValue::new();
    }
}
