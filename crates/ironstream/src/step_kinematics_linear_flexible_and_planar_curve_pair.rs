// FILE: step_kinematics_linear_flexible_and_planar_curve_pair.rs
// occt: StepKinematics_LinearFlexibleAndPlanarCurvePair

pub struct LinearFlexibleAndPlanarCurvePair {
    pair_curve: Option<Box<dyn std::any::Any>>,
    orientation: bool,
}

impl LinearFlexibleAndPlanarCurvePair {
    pub fn new() -> Self {
        LinearFlexibleAndPlanarCurvePair {
            pair_curve: None,
            orientation: false,
        }
    }

    pub fn init(&mut self, pair_curve: Option<Box<dyn std::any::Any>>, orientation: bool) {
        self.pair_curve = pair_curve;
        self.orientation = orientation;
    }

    pub fn pair_curve(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.pair_curve
    }

    pub fn set_pair_curve(&mut self, curve: Option<Box<dyn std::any::Any>>) {
        self.pair_curve = curve;
    }

    pub fn orientation(&self) -> bool {
        self.orientation
    }

    pub fn set_orientation(&mut self, value: bool) {
        self.orientation = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_flexible_and_planar_curve_pair_creation() {
        let pair = LinearFlexibleAndPlanarCurvePair::new();
        assert!(pair.pair_curve().is_none());
        assert_eq!(pair.orientation(), false);
    }

    #[test]
    fn test_init() {
        let mut pair = LinearFlexibleAndPlanarCurvePair::new();
        pair.init(None, true);
        assert!(pair.pair_curve().is_none());
        assert_eq!(pair.orientation(), true);
    }

    #[test]
    fn test_setters() {
        let mut pair = LinearFlexibleAndPlanarCurvePair::new();
        pair.set_pair_curve(None);
        pair.set_orientation(true);
        assert!(pair.pair_curve().is_none());
        assert_eq!(pair.orientation(), true);
    }
}
