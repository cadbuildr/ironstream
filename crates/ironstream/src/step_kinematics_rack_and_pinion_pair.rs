// FILE: step_kinematics_rack_and_pinion_pair.rs
// occt: StepKinematics_RackAndPinionPair

/// Representation of STEP entity RackAndPinionPair.
#[derive(Clone, Debug)]
pub struct StepKinematicsRackAndPinionPair {
    pinion_radius: f64,
}

impl Default for StepKinematicsRackAndPinionPair {
    fn default() -> Self {
        StepKinematicsRackAndPinionPair {
            pinion_radius: 0.0,
        }
    }
}

impl StepKinematicsRackAndPinionPair {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pinion_radius(&self) -> f64 {
        self.pinion_radius
    }

    pub fn set_pinion_radius(&mut self, value: f64) {
        self.pinion_radius = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsRackAndPinionPair::new();
        assert_eq!(pair.pinion_radius(), 0.0);
    }

    #[test]
    fn test_setter() {
        let mut pair = StepKinematicsRackAndPinionPair::new();
        pair.set_pinion_radius(2.5);
        assert_eq!(pair.pinion_radius(), 2.5);
    }
}
