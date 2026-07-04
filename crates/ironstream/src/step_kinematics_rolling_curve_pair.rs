// FILE: step_kinematics_rolling_curve_pair.rs
// occt: StepKinematics_RollingCurvePair

/// Representation of STEP entity RollingCurvePair.
#[derive(Clone, Debug)]
pub struct StepKinematicsRollingCurvePair {
    curve1: (),
    curve2: (),
}

impl Default for StepKinematicsRollingCurvePair {
    fn default() -> Self {
        StepKinematicsRollingCurvePair {
            curve1: (),
            curve2: (),
        }
    }
}

impl StepKinematicsRollingCurvePair {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn curve1(&self) -> () {
        self.curve1
    }

    pub fn set_curve1(&mut self, _c: ()) {
        self.curve1 = ();
    }

    pub fn curve2(&self) -> () {
        self.curve2
    }

    pub fn set_curve2(&mut self, _c: ()) {
        self.curve2 = ();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsRollingCurvePair::new();
    }
}
