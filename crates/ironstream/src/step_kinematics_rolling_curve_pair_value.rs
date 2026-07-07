// FILE: step_kinematics_rolling_curve_pair_value.rs
// occt: StepKinematics_RollingCurvePairValue

/// Representation of STEP entity RollingCurvePairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsRollingCurvePairValue {
    actual_point_on_curve1: (),
    actual_point_on_curve2: (),
}

impl Default for StepKinematicsRollingCurvePairValue {
    fn default() -> Self {
        StepKinematicsRollingCurvePairValue {
            actual_point_on_curve1: (),
            actual_point_on_curve2: (),
        }
    }
}

impl StepKinematicsRollingCurvePairValue {
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
        let _value = StepKinematicsRollingCurvePairValue::new();
    }
}
