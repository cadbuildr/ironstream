// FILE: step_kinematics_screw_pair.rs
// occt: StepKinematics_ScrewPair

/// Representation of STEP entity ScrewPair.
#[derive(Clone, Debug)]
pub struct StepKinematicsScrewPair {
    pitch: f64,
}

impl Default for StepKinematicsScrewPair {
    fn default() -> Self {
        StepKinematicsScrewPair {
            pitch: 0.0,
        }
    }
}

impl StepKinematicsScrewPair {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pitch(&self) -> f64 {
        self.pitch
    }

    pub fn set_pitch(&mut self, value: f64) {
        self.pitch = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsScrewPair::new();
        assert_eq!(pair.pitch(), 0.0);
    }

    #[test]
    fn test_setter() {
        let mut pair = StepKinematicsScrewPair::new();
        pair.set_pitch(2.0);
        assert_eq!(pair.pitch(), 2.0);
    }
}
