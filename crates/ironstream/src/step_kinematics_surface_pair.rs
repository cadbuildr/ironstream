// FILE: step_kinematics_surface_pair.rs
// occt: StepKinematics_SurfacePair

/// Representation of STEP entity SurfacePair.
#[derive(Clone, Debug)]
pub struct StepKinematicsSurfacePair {
    surface1: (),
    surface2: (),
}

impl Default for StepKinematicsSurfacePair {
    fn default() -> Self {
        StepKinematicsSurfacePair {
            surface1: (),
            surface2: (),
        }
    }
}

impl StepKinematicsSurfacePair {
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
        let _pair = StepKinematicsSurfacePair::new();
    }
}
