// FILE: step_kinematics_planar_pair_with_range.rs
// occt: StepKinematics_PlanarPairWithRange

/// Representation of STEP entity PlanarPairWithRange.
/// This extends PlanarPair with optional range constraints on rotation and translation axes.
#[derive(Clone, Debug)]
pub struct StepKinematicsPlanarPairWithRange {
    lower_limit_actual_rotation: Option<f64>,
    upper_limit_actual_rotation: Option<f64>,
    lower_limit_actual_translation_x: Option<f64>,
    upper_limit_actual_translation_x: Option<f64>,
    lower_limit_actual_translation_y: Option<f64>,
    upper_limit_actual_translation_y: Option<f64>,
}

impl Default for StepKinematicsPlanarPairWithRange {
    fn default() -> Self {
        StepKinematicsPlanarPairWithRange {
            lower_limit_actual_rotation: None,
            upper_limit_actual_rotation: None,
            lower_limit_actual_translation_x: None,
            upper_limit_actual_translation_x: None,
            lower_limit_actual_translation_y: None,
            upper_limit_actual_translation_y: None,
        }
    }
}

impl StepKinematicsPlanarPairWithRange {
    /// Create a new PlanarPairWithRange with all fields initially unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the lower rotation limit if defined.
    pub fn lower_limit_actual_rotation(&self) -> Option<f64> {
        self.lower_limit_actual_rotation
    }

    /// Sets the lower rotation limit.
    pub fn set_lower_limit_actual_rotation(&mut self, value: f64) {
        self.lower_limit_actual_rotation = Some(value);
    }

    /// Returns whether the lower rotation limit is defined.
    pub fn has_lower_limit_actual_rotation(&self) -> bool {
        self.lower_limit_actual_rotation.is_some()
    }

    /// Returns the upper rotation limit if defined.
    pub fn upper_limit_actual_rotation(&self) -> Option<f64> {
        self.upper_limit_actual_rotation
    }

    /// Sets the upper rotation limit.
    pub fn set_upper_limit_actual_rotation(&mut self, value: f64) {
        self.upper_limit_actual_rotation = Some(value);
    }

    /// Returns whether the upper rotation limit is defined.
    pub fn has_upper_limit_actual_rotation(&self) -> bool {
        self.upper_limit_actual_rotation.is_some()
    }

    /// Returns the lower X translation limit if defined.
    pub fn lower_limit_actual_translation_x(&self) -> Option<f64> {
        self.lower_limit_actual_translation_x
    }

    /// Sets the lower X translation limit.
    pub fn set_lower_limit_actual_translation_x(&mut self, value: f64) {
        self.lower_limit_actual_translation_x = Some(value);
    }

    /// Returns whether the lower X translation limit is defined.
    pub fn has_lower_limit_actual_translation_x(&self) -> bool {
        self.lower_limit_actual_translation_x.is_some()
    }

    /// Returns the upper X translation limit if defined.
    pub fn upper_limit_actual_translation_x(&self) -> Option<f64> {
        self.upper_limit_actual_translation_x
    }

    /// Sets the upper X translation limit.
    pub fn set_upper_limit_actual_translation_x(&mut self, value: f64) {
        self.upper_limit_actual_translation_x = Some(value);
    }

    /// Returns whether the upper X translation limit is defined.
    pub fn has_upper_limit_actual_translation_x(&self) -> bool {
        self.upper_limit_actual_translation_x.is_some()
    }

    /// Returns the lower Y translation limit if defined.
    pub fn lower_limit_actual_translation_y(&self) -> Option<f64> {
        self.lower_limit_actual_translation_y
    }

    /// Sets the lower Y translation limit.
    pub fn set_lower_limit_actual_translation_y(&mut self, value: f64) {
        self.lower_limit_actual_translation_y = Some(value);
    }

    /// Returns whether the lower Y translation limit is defined.
    pub fn has_lower_limit_actual_translation_y(&self) -> bool {
        self.lower_limit_actual_translation_y.is_some()
    }

    /// Returns the upper Y translation limit if defined.
    pub fn upper_limit_actual_translation_y(&self) -> Option<f64> {
        self.upper_limit_actual_translation_y
    }

    /// Sets the upper Y translation limit.
    pub fn set_upper_limit_actual_translation_y(&mut self, value: f64) {
        self.upper_limit_actual_translation_y = Some(value);
    }

    /// Returns whether the upper Y translation limit is defined.
    pub fn has_upper_limit_actual_translation_y(&self) -> bool {
        self.upper_limit_actual_translation_y.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planar_pair_with_range_creation() {
        let pair = StepKinematicsPlanarPairWithRange::new();
        assert!(!pair.has_lower_limit_actual_rotation());
        assert!(!pair.has_upper_limit_actual_rotation());
        assert!(!pair.has_lower_limit_actual_translation_x());
        assert!(!pair.has_upper_limit_actual_translation_x());
        assert!(!pair.has_lower_limit_actual_translation_y());
        assert!(!pair.has_upper_limit_actual_translation_y());
    }

    #[test]
    fn test_planar_pair_with_range_rotation_limits() {
        let mut pair = StepKinematicsPlanarPairWithRange::new();
        pair.set_lower_limit_actual_rotation(-3.14);
        pair.set_upper_limit_actual_rotation(3.14);

        assert!(pair.has_lower_limit_actual_rotation());
        assert!(pair.has_upper_limit_actual_rotation());
        assert_eq!(pair.lower_limit_actual_rotation(), Some(-3.14));
        assert_eq!(pair.upper_limit_actual_rotation(), Some(3.14));
    }

    #[test]
    fn test_planar_pair_with_range_translation_limits() {
        let mut pair = StepKinematicsPlanarPairWithRange::new();
        pair.set_lower_limit_actual_translation_x(-10.0);
        pair.set_upper_limit_actual_translation_x(10.0);
        pair.set_lower_limit_actual_translation_y(-5.0);
        pair.set_upper_limit_actual_translation_y(5.0);

        assert_eq!(pair.lower_limit_actual_translation_x(), Some(-10.0));
        assert_eq!(pair.upper_limit_actual_translation_x(), Some(10.0));
        assert_eq!(pair.lower_limit_actual_translation_y(), Some(-5.0));
        assert_eq!(pair.upper_limit_actual_translation_y(), Some(5.0));
    }

    #[test]
    fn test_planar_pair_with_range_all_fields() {
        let mut pair = StepKinematicsPlanarPairWithRange::new();
        pair.set_lower_limit_actual_rotation(-1.0);
        pair.set_upper_limit_actual_rotation(1.0);
        pair.set_lower_limit_actual_translation_x(-2.0);
        pair.set_upper_limit_actual_translation_x(2.0);
        pair.set_lower_limit_actual_translation_y(-3.0);
        pair.set_upper_limit_actual_translation_y(3.0);

        assert!(pair.has_lower_limit_actual_rotation());
        assert!(pair.has_upper_limit_actual_rotation());
        assert!(pair.has_lower_limit_actual_translation_x());
        assert!(pair.has_upper_limit_actual_translation_x());
        assert!(pair.has_lower_limit_actual_translation_y());
        assert!(pair.has_upper_limit_actual_translation_y());
    }
}
