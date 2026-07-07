// FILE: step_kinematics_unconstrained_pair_value.rs
// occt: StepKinematics_UnconstrainedPairValue

/// Representation of STEP entity UnconstrainedPairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsUnconstrainedPairValue {
    actual_placement: (),
}

impl Default for StepKinematicsUnconstrainedPairValue {
    fn default() -> Self {
        StepKinematicsUnconstrainedPairValue {
            actual_placement: (),
        }
    }
}

impl StepKinematicsUnconstrainedPairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actual_placement(&self) -> () {
        self.actual_placement
    }

    pub fn set_actual_placement(&mut self, _p: ()) {
        self.actual_placement = ();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _value = StepKinematicsUnconstrainedPairValue::new();
    }
}
