// FILE: step_kinematics_sliding_surface_pair_value.rs
// occt: StepKinematics_SlidingSurfacePairValue

/// Representation of STEP entity SlidingSurfacePairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsSlidingSurfacePairValue {
    actual_point_on_surface1: (),
    actual_point_on_surface2: (),
}

impl Default for StepKinematicsSlidingSurfacePairValue {
    fn default() -> Self {
        StepKinematicsSlidingSurfacePairValue {
            actual_point_on_surface1: (),
            actual_point_on_surface2: (),
        }
    }
}

impl StepKinematicsSlidingSurfacePairValue {
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
        let _value = StepKinematicsSlidingSurfacePairValue::new();
    }
}
