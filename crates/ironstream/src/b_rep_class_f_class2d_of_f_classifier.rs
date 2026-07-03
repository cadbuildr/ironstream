// FILE: b_rep_class_f_class2d_of_f_classifier.rs
// occt: BRepClass_FClass2dOfFClassifier

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    On,
    In,
    Out,
}

/// 2D face classifier for face point classification
pub struct FClass2dOfFClassifier {
    is_set: bool,
    first_compare: bool,
    first_trans: bool,
    param: f64,
    tolerance: f64,
    closest: i32,
    state: State,
    is_head_or_end: bool,
}

impl FClass2dOfFClassifier {
    pub fn new() -> Self {
        FClass2dOfFClassifier {
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
        self.closest = 0;
        self.state = State::Out;
        self.is_head_or_end = false;
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

impl Default for FClass2dOfFClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_creation() {
        let clf = FClass2dOfFClassifier::new();
        assert_eq!(clf.state(), State::Out);
        assert_eq!(clf.closest_intersection(), 0);
    }

    #[test]
    fn test_reset() {
        let mut clf = FClass2dOfFClassifier::new();
        clf.reset(3.5, 0.01);
        assert_eq!(clf.parameter(), 3.5);
    }

    #[test]
    fn test_compare() {
        let mut clf = FClass2dOfFClassifier::new();
        clf.reset(1.0, 0.001);
        clf.compare(0);
        assert!(!clf.is_head_or_end());
    }
}
