// FILE: b_rep_class_face_passive_classifier.rs
// occt: BRepClass_FacePassiveClassifier

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    On,
    In,
    Out,
}

/// Passive classifier for face classification
pub struct FacePassiveClassifier {
    is_set: bool,
    first_compare: bool,
    first_trans: bool,
    param: f64,
    tolerance: f64,
    closest: i32,
    state: State,
    is_head_or_end: bool,
}

impl FacePassiveClassifier {
    pub fn new() -> Self {
        FacePassiveClassifier {
            is_set: false,
            first_compare: true,
            first_trans: true,
            param: 0.0,
            tolerance: 1e-7,
            closest: 0,
            state: State::Out,
            is_head_or_end: false,
        }
    }

    pub fn reset(&mut self, param: f64, tol: f64) {
        self.is_set = true;
        self.param = param;
        self.tolerance = tol;
        self.first_compare = true;
        self.first_trans = true;
    }

    pub fn compare(&mut self, _orientation: i32) {
        self.first_compare = false;
    }

    pub fn parameter(&self) -> f64 {
        self.param
    }

    pub fn closest_intersection(&self) -> i32 {
        self.closest
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn is_head_or_end(&self) -> bool {
        self.is_head_or_end
    }
}

impl Default for FacePassiveClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_creation() {
        let clf = FacePassiveClassifier::new();
        assert_eq!(clf.state(), State::Out);
        assert_eq!(clf.closest_intersection(), 0);
    }

    #[test]
    fn test_reset() {
        let mut clf = FacePassiveClassifier::new();
        clf.reset(2.5, 0.001);
        assert_eq!(clf.parameter(), 2.5);
    }

    #[test]
    fn test_is_head_or_end() {
        let clf = FacePassiveClassifier::new();
        assert!(!clf.is_head_or_end());
    }
}
