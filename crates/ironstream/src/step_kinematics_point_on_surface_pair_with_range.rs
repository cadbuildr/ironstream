// FILE: step_kinematics_point_on_surface_pair_with_range.rs
// occt: StepKinematics_PointOnSurfacePairWithRange

/// Representation of STEP entity PointOnSurfacePairWithRange.
#[derive(Clone, Debug)]
pub struct StepKinematicsPointOnSurfacePairWithRange {
    range_on_pair_surface: (),
    lower_limit_yaw: Option<f64>,
    upper_limit_yaw: Option<f64>,
    lower_limit_pitch: Option<f64>,
    upper_limit_pitch: Option<f64>,
    lower_limit_roll: Option<f64>,
    upper_limit_roll: Option<f64>,
}

impl Default for StepKinematicsPointOnSurfacePairWithRange {
    fn default() -> Self {
        StepKinematicsPointOnSurfacePairWithRange {
            range_on_pair_surface: (),
            lower_limit_yaw: None,
            upper_limit_yaw: None,
            lower_limit_pitch: None,
            upper_limit_pitch: None,
            lower_limit_roll: None,
            upper_limit_roll: None,
        }
    }
}

impl StepKinematicsPointOnSurfacePairWithRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn range_on_pair_surface(&self) -> () {
        self.range_on_pair_surface
    }

    pub fn set_range_on_pair_surface(&mut self, _surface: ()) {
        self.range_on_pair_surface = ();
    }

    pub fn lower_limit_yaw(&self) -> Option<f64> {
        self.lower_limit_yaw
    }

    pub fn set_lower_limit_yaw(&mut self, value: f64) {
        self.lower_limit_yaw = Some(value);
    }

    pub fn has_lower_limit_yaw(&self) -> bool {
        self.lower_limit_yaw.is_some()
    }

    pub fn upper_limit_yaw(&self) -> Option<f64> {
        self.upper_limit_yaw
    }

    pub fn set_upper_limit_yaw(&mut self, value: f64) {
        self.upper_limit_yaw = Some(value);
    }

    pub fn has_upper_limit_yaw(&self) -> bool {
        self.upper_limit_yaw.is_some()
    }

    pub fn lower_limit_pitch(&self) -> Option<f64> {
        self.lower_limit_pitch
    }

    pub fn set_lower_limit_pitch(&mut self, value: f64) {
        self.lower_limit_pitch = Some(value);
    }

    pub fn has_lower_limit_pitch(&self) -> bool {
        self.lower_limit_pitch.is_some()
    }

    pub fn upper_limit_pitch(&self) -> Option<f64> {
        self.upper_limit_pitch
    }

    pub fn set_upper_limit_pitch(&mut self, value: f64) {
        self.upper_limit_pitch = Some(value);
    }

    pub fn has_upper_limit_pitch(&self) -> bool {
        self.upper_limit_pitch.is_some()
    }

    pub fn lower_limit_roll(&self) -> Option<f64> {
        self.lower_limit_roll
    }

    pub fn set_lower_limit_roll(&mut self, value: f64) {
        self.lower_limit_roll = Some(value);
    }

    pub fn has_lower_limit_roll(&self) -> bool {
        self.lower_limit_roll.is_some()
    }

    pub fn upper_limit_roll(&self) -> Option<f64> {
        self.upper_limit_roll
    }

    pub fn set_upper_limit_roll(&mut self, value: f64) {
        self.upper_limit_roll = Some(value);
    }

    pub fn has_upper_limit_roll(&self) -> bool {
        self.upper_limit_roll.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsPointOnSurfacePairWithRange::new();
        assert!(!pair.has_lower_limit_yaw());
        assert!(!pair.has_upper_limit_yaw());
    }

    #[test]
    fn test_all_limits() {
        let mut pair = StepKinematicsPointOnSurfacePairWithRange::new();
        pair.set_lower_limit_yaw(-1.0);
        pair.set_upper_limit_yaw(1.0);
        pair.set_lower_limit_pitch(-0.5);
        pair.set_upper_limit_pitch(0.5);
        pair.set_lower_limit_roll(-0.8);
        pair.set_upper_limit_roll(0.8);

        assert!(pair.has_lower_limit_yaw());
        assert!(pair.has_upper_limit_yaw());
        assert!(pair.has_lower_limit_pitch());
        assert!(pair.has_upper_limit_pitch());
        assert!(pair.has_lower_limit_roll());
        assert!(pair.has_upper_limit_roll());
    }
}
