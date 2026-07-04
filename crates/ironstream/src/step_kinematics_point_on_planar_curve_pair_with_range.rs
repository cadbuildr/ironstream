// FILE: step_kinematics_point_on_planar_curve_pair_with_range.rs
// occt: StepKinematics_PointOnPlanarCurvePairWithRange

/// Representation of STEP entity PointOnPlanarCurvePairWithRange.
/// Extends PointOnPlanarCurvePair with optional range limits on yaw, pitch, and roll angles.
#[derive(Clone, Debug)]
pub struct StepKinematicsPointOnPlanarCurvePairWithRange {
    range_on_pair_curve: (),
    lower_limit_yaw: Option<f64>,
    upper_limit_yaw: Option<f64>,
    lower_limit_pitch: Option<f64>,
    upper_limit_pitch: Option<f64>,
    lower_limit_roll: Option<f64>,
    upper_limit_roll: Option<f64>,
}

impl Default for StepKinematicsPointOnPlanarCurvePairWithRange {
    fn default() -> Self {
        StepKinematicsPointOnPlanarCurvePairWithRange {
            range_on_pair_curve: (),
            lower_limit_yaw: None,
            upper_limit_yaw: None,
            lower_limit_pitch: None,
            upper_limit_pitch: None,
            lower_limit_roll: None,
            upper_limit_roll: None,
        }
    }
}

impl StepKinematicsPointOnPlanarCurvePairWithRange {
    /// Create a new PointOnPlanarCurvePairWithRange.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the range on pair curve (placeholder for handle).
    pub fn range_on_pair_curve(&self) -> () {
        self.range_on_pair_curve
    }

    /// Sets the range on pair curve (placeholder for handle).
    pub fn set_range_on_pair_curve(&mut self, _curve: ()) {
        self.range_on_pair_curve = ();
    }

    /// Returns the lower yaw limit if defined.
    pub fn lower_limit_yaw(&self) -> Option<f64> {
        self.lower_limit_yaw
    }

    /// Sets the lower yaw limit.
    pub fn set_lower_limit_yaw(&mut self, value: f64) {
        self.lower_limit_yaw = Some(value);
    }

    /// Returns whether the lower yaw limit is defined.
    pub fn has_lower_limit_yaw(&self) -> bool {
        self.lower_limit_yaw.is_some()
    }

    /// Returns the upper yaw limit if defined.
    pub fn upper_limit_yaw(&self) -> Option<f64> {
        self.upper_limit_yaw
    }

    /// Sets the upper yaw limit.
    pub fn set_upper_limit_yaw(&mut self, value: f64) {
        self.upper_limit_yaw = Some(value);
    }

    /// Returns whether the upper yaw limit is defined.
    pub fn has_upper_limit_yaw(&self) -> bool {
        self.upper_limit_yaw.is_some()
    }

    /// Returns the lower pitch limit if defined.
    pub fn lower_limit_pitch(&self) -> Option<f64> {
        self.lower_limit_pitch
    }

    /// Sets the lower pitch limit.
    pub fn set_lower_limit_pitch(&mut self, value: f64) {
        self.lower_limit_pitch = Some(value);
    }

    /// Returns whether the lower pitch limit is defined.
    pub fn has_lower_limit_pitch(&self) -> bool {
        self.lower_limit_pitch.is_some()
    }

    /// Returns the upper pitch limit if defined.
    pub fn upper_limit_pitch(&self) -> Option<f64> {
        self.upper_limit_pitch
    }

    /// Sets the upper pitch limit.
    pub fn set_upper_limit_pitch(&mut self, value: f64) {
        self.upper_limit_pitch = Some(value);
    }

    /// Returns whether the upper pitch limit is defined.
    pub fn has_upper_limit_pitch(&self) -> bool {
        self.upper_limit_pitch.is_some()
    }

    /// Returns the lower roll limit if defined.
    pub fn lower_limit_roll(&self) -> Option<f64> {
        self.lower_limit_roll
    }

    /// Sets the lower roll limit.
    pub fn set_lower_limit_roll(&mut self, value: f64) {
        self.lower_limit_roll = Some(value);
    }

    /// Returns whether the lower roll limit is defined.
    pub fn has_lower_limit_roll(&self) -> bool {
        self.lower_limit_roll.is_some()
    }

    /// Returns the upper roll limit if defined.
    pub fn upper_limit_roll(&self) -> Option<f64> {
        self.upper_limit_roll
    }

    /// Sets the upper roll limit.
    pub fn set_upper_limit_roll(&mut self, value: f64) {
        self.upper_limit_roll = Some(value);
    }

    /// Returns whether the upper roll limit is defined.
    pub fn has_upper_limit_roll(&self) -> bool {
        self.upper_limit_roll.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let pair = StepKinematicsPointOnPlanarCurvePairWithRange::new();
        assert!(!pair.has_lower_limit_yaw());
        assert!(!pair.has_upper_limit_yaw());
        assert!(!pair.has_lower_limit_pitch());
        assert!(!pair.has_upper_limit_pitch());
        assert!(!pair.has_lower_limit_roll());
        assert!(!pair.has_upper_limit_roll());
    }

    #[test]
    fn test_yaw_limits() {
        let mut pair = StepKinematicsPointOnPlanarCurvePairWithRange::new();
        pair.set_lower_limit_yaw(-1.5);
        pair.set_upper_limit_yaw(1.5);

        assert_eq!(pair.lower_limit_yaw(), Some(-1.5));
        assert_eq!(pair.upper_limit_yaw(), Some(1.5));
        assert!(pair.has_lower_limit_yaw());
        assert!(pair.has_upper_limit_yaw());
    }

    #[test]
    fn test_pitch_limits() {
        let mut pair = StepKinematicsPointOnPlanarCurvePairWithRange::new();
        pair.set_lower_limit_pitch(-0.5);
        pair.set_upper_limit_pitch(0.5);

        assert_eq!(pair.lower_limit_pitch(), Some(-0.5));
        assert_eq!(pair.upper_limit_pitch(), Some(0.5));
    }

    #[test]
    fn test_roll_limits() {
        let mut pair = StepKinematicsPointOnPlanarCurvePairWithRange::new();
        pair.set_lower_limit_roll(-2.0);
        pair.set_upper_limit_roll(2.0);

        assert_eq!(pair.lower_limit_roll(), Some(-2.0));
        assert_eq!(pair.upper_limit_roll(), Some(2.0));
    }

    #[test]
    fn test_all_limits() {
        let mut pair = StepKinematicsPointOnPlanarCurvePairWithRange::new();
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
