// FILE: b_rep_class3d_solid_passive_classifier.rs
// occt: BRepClass3d_SolidPassiveClassifier

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    On,
    In,
    Out,
}

/// Passive classifier for solid points
pub struct SolidPassiveClassifier {
    is_set: bool,
    param: f64,
    tolerance: f64,
    state: State,
    has_intersect: bool,
}

impl SolidPassiveClassifier {
    pub fn new() -> Self {
        SolidPassiveClassifier {
            is_set: false,
            param: 0.0,
            tolerance: 1e-7,
            state: State::Out,
            has_intersect: false,
        }
    }

    pub fn reset(&mut self, param: f64, tol: f64) {
        self.is_set = true;
        self.param = param;
        self.tolerance = tol;
        self.has_intersect = false;
    }

    pub fn compare(&mut self, _orientation: i32) {
        self.has_intersect = true;
    }

    pub fn parameter(&self) -> f64 {
        self.param
    }

    pub fn has_intersection(&self) -> bool {
        self.has_intersect
    }

    pub fn state(&self) -> State {
        self.state
    }
}

impl Default for SolidPassiveClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_creation() {
        let clf = SolidPassiveClassifier::new();
        assert!(!clf.has_intersection());
        assert_eq!(clf.state(), State::Out);
    }

    #[test]
    fn test_reset() {
        let mut clf = SolidPassiveClassifier::new();
        clf.reset(5.0, 0.01);
        assert_eq!(clf.parameter(), 5.0);
    }

    #[test]
    fn test_compare() {
        let mut clf = SolidPassiveClassifier::new();
        assert!(!clf.has_intersection());
        clf.compare(0);
        assert!(clf.has_intersection());
    }
}
