// FILE: step_kinematics_sliding_surface_pair.rs
// occt: StepKinematics_SlidingSurfacePair

/// Representation of STEP entity SlidingSurfacePair.
#[derive(Clone, Debug)]
pub struct StepKinematicsSlidingSurfacePair {
    surface1: (),
    surface2: (),
}

impl Default for StepKinematicsSlidingSurfacePair {
    fn default() -> Self {
        StepKinematicsSlidingSurfacePair {
            surface1: (),
            surface2: (),
        }
    }
}

impl StepKinematicsSlidingSurfacePair {
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
        let _pair = StepKinematicsSlidingSurfacePair::new();
    }
}
