// FILE: step_kinematics_rolling_surface_pair.rs
// occt: StepKinematics_RollingSurfacePair

/// Representation of STEP entity RollingSurfacePair.
#[derive(Clone, Debug)]
pub struct StepKinematicsRollingSurfacePair {
    surface1: (),
    surface2: (),
}

impl Default for StepKinematicsRollingSurfacePair {
    fn default() -> Self {
        StepKinematicsRollingSurfacePair {
            surface1: (),
            surface2: (),
        }
    }
}

impl StepKinematicsRollingSurfacePair {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn surface1(&self) -> () {
        self.surface1
    }

    pub fn set_surface1(&mut self, _s: ()) {
        self.surface1 = ();
    }

    pub fn surface2(&self) -> () {
        self.surface2
    }

    pub fn set_surface2(&mut self, _s: ()) {
        self.surface2 = ();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsRollingSurfacePair::new();
    }
}
