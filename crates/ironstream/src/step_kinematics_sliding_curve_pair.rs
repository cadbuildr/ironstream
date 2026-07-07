// FILE: step_kinematics_sliding_curve_pair.rs
// occt: StepKinematics_SlidingCurvePair

/// Representation of STEP entity SlidingCurvePair.
#[derive(Clone, Debug)]
pub struct StepKinematicsSlidingCurvePair {
    curve1: (),
    curve2: (),
}

impl Default for StepKinematicsSlidingCurvePair {
    fn default() -> Self {
        StepKinematicsSlidingCurvePair {
            curve1: (),
            curve2: (),
        }
    }
}

impl StepKinematicsSlidingCurvePair {
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
        let _pair = StepKinematicsSlidingCurvePair::new();
    }
}
