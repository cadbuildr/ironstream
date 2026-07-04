// FILE: step_kinematics_planar_curve_pair.rs
// occt: StepKinematics_PlanarCurvePair

pub struct PlanarCurvePair {
    curve1: Option<Box<dyn std::any::Any>>,
    curve2: Option<Box<dyn std::any::Any>>,
    orientation: bool,
}

impl PlanarCurvePair {
    pub fn new() -> Self {
        PlanarCurvePair {
            curve1: None,
            curve2: None,
            orientation: false,
        }
    }

    pub fn init(
        &mut self,
        curve1: Option<Box<dyn std::any::Any>>,
        curve2: Option<Box<dyn std::any::Any>>,
        orientation: bool,
    ) {
        self.curve1 = curve1;
        self.curve2 = curve2;
        self.orientation = orientation;
    }

    pub fn curve1(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.curve1
    }

    pub fn set_curve1(&mut self, curve: Option<Box<dyn std::any::Any>>) {
        self.curve1 = curve;
    }

    pub fn curve2(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.curve2
    }

    pub fn set_curve2(&mut self, curve: Option<Box<dyn std::any::Any>>) {
        self.curve2 = curve;
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
    fn test_planar_curve_pair_creation() {
        let pair = PlanarCurvePair::new();
        assert!(pair.curve1().is_none());
        assert!(pair.curve2().is_none());
        assert_eq!(pair.orientation(), false);
    }

    #[test]
    fn test_init() {
        let mut pair = PlanarCurvePair::new();
        pair.init(None, None, true);
        assert!(pair.curve1().is_none());
        assert!(pair.curve2().is_none());
        assert_eq!(pair.orientation(), true);
    }

    #[test]
    fn test_setters() {
        let mut pair = PlanarCurvePair::new();
        pair.set_curve1(None);
        pair.set_curve2(None);
        pair.set_orientation(true);
        assert_eq!(pair.orientation(), true);
    }
}
