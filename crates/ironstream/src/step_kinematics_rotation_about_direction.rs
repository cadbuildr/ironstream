// FILE: step_kinematics_rotation_about_direction.rs
// occt: StepKinematics_RotationAboutDirection

/// Representation of STEP entity RotationAboutDirection.
#[derive(Clone, Debug)]
pub struct StepKinematicsRotationAboutDirection {
    direction: (),
    rotation_angle: f64,
}

impl Default for StepKinematicsRotationAboutDirection {
    fn default() -> Self {
        StepKinematicsRotationAboutDirection {
            direction: (),
            rotation_angle: 0.0,
        }
    }
}

impl StepKinematicsRotationAboutDirection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn direction(&self) -> () {
        self.direction
    }

    pub fn set_direction(&mut self, _d: ()) {
        self.direction = ();
    }

    pub fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }

    pub fn set_rotation_angle(&mut self, angle: f64) {
        self.rotation_angle = angle;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let rot = StepKinematicsRotationAboutDirection::new();
        assert_eq!(rot.rotation_angle(), 0.0);
    }

    #[test]
    fn test_setter() {
        let mut rot = StepKinematicsRotationAboutDirection::new();
        rot.set_rotation_angle(1.57);
        assert_eq!(rot.rotation_angle(), 1.57);
    }
}
