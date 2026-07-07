// FILE: step_kinematics_point_on_surface_pair_value.rs
// occt: StepKinematics_PointOnSurfacePairValue

/// Representation of STEP entity PointOnSurfacePairValue.
#[derive(Clone, Debug)]
pub struct StepKinematicsPointOnSurfacePairValue {
    actual_point_on_surface: (),
    input_orientation: StepKinematicsSpatialRotationRef,
}

#[derive(Clone, Debug)]
pub struct StepKinematicsSpatialRotationRef;

impl Default for StepKinematicsPointOnSurfacePairValue {
    fn default() -> Self {
        StepKinematicsPointOnSurfacePairValue {
            actual_point_on_surface: (),
            input_orientation: StepKinematicsSpatialRotationRef,
        }
    }
}

impl StepKinematicsPointOnSurfacePairValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actual_point_on_surface(&self) -> () {
        self.actual_point_on_surface
    }

    pub fn set_actual_point_on_surface(&mut self, _point: ()) {
        self.actual_point_on_surface = ();
    }

    pub fn input_orientation(&self) -> &StepKinematicsSpatialRotationRef {
        &self.input_orientation
    }

    pub fn set_input_orientation(&mut self, orientation: StepKinematicsSpatialRotationRef) {
        self.input_orientation = orientation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _value = StepKinematicsPointOnSurfacePairValue::new();
    }

    #[test]
    fn test_setters() {
        let mut value = StepKinematicsPointOnSurfacePairValue::new();
        value.set_actual_point_on_surface(());
        value.set_input_orientation(StepKinematicsSpatialRotationRef);

        let _ = value.actual_point_on_surface();
        let _ = value.input_orientation();
    }

    #[test]
    fn test_clone() {
        let value = StepKinematicsPointOnSurfacePairValue::new();
        let _cloned = value.clone();
    }
}
