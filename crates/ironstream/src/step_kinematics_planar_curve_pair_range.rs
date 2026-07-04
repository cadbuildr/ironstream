// FILE: step_kinematics_planar_curve_pair_range.rs
// occt: StepKinematics_PlanarCurvePairRange

pub struct PlanarCurvePairRange {
    range_on_curve1: Option<Box<dyn std::any::Any>>,
    range_on_curve2: Option<Box<dyn std::any::Any>>,
}

impl PlanarCurvePairRange {
    pub fn new() -> Self {
        PlanarCurvePairRange {
            range_on_curve1: None,
            range_on_curve2: None,
        }
    }

    pub fn init(
        &mut self,
        range_on_curve1: Option<Box<dyn std::any::Any>>,
        range_on_curve2: Option<Box<dyn std::any::Any>>,
    ) {
        self.range_on_curve1 = range_on_curve1;
        self.range_on_curve2 = range_on_curve2;
    }

    pub fn range_on_curve1(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.range_on_curve1
    }

    pub fn set_range_on_curve1(&mut self, curve: Option<Box<dyn std::any::Any>>) {
        self.range_on_curve1 = curve;
    }

    pub fn range_on_curve2(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.range_on_curve2
    }

    pub fn set_range_on_curve2(&mut self, curve: Option<Box<dyn std::any::Any>>) {
        self.range_on_curve2 = curve;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planar_curve_pair_range_creation() {
        let range = PlanarCurvePairRange::new();
        assert!(range.range_on_curve1().is_none());
        assert!(range.range_on_curve2().is_none());
    }

    #[test]
    fn test_init() {
        let mut range = PlanarCurvePairRange::new();
        range.init(None, None);
        assert!(range.range_on_curve1().is_none());
        assert!(range.range_on_curve2().is_none());
    }
}
