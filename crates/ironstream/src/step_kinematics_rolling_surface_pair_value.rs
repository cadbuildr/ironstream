// FILE: step_kinematics_rolling_surface_pair_value.rs
// occt: StepKinematics_RollingSurfacePairValue

/// Representation of STEP entity RollingSurfacePairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsRollingSurfacePairValue {
    actual_point_on_surface1: (),
    actual_point_on_surface2: (),
}

impl Default for StepKinematicsRollingSurfacePairValue {
    fn default() -> Self {
        StepKinematicsRollingSurfacePairValue {
            actual_point_on_surface1: (),
            actual_point_on_surface2: (),
        }
    }
}

impl StepKinematicsRollingSurfacePairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actual_point_on_surface1(&self) -> () {
        self.actual_point_on_surface1
    }

    pub fn set_actual_point_on_surface1(&mut self, _p: ()) {
        self.actual_point_on_surface1 = ();
    }

    pub fn actual_point_on_surface2(&self) -> () {
        self.actual_point_on_surface2
    }

    pub fn set_actual_point_on_surface2(&mut self, _p: ()) {
        self.actual_point_on_surface2 = ();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _value = StepKinematicsRollingSurfacePairValue::new();
    }
}
