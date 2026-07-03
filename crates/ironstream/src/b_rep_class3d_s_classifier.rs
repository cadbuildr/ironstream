// FILE: b_rep_class3d_s_classifier.rs
// occt: BRepClass3d_SClassifier

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClassState {
    Error,
    On,
    In,
    Out,
}

/// Classifier for points in 3D solids
pub struct SolidClassifier {
    state: ClassState,
    is_on_face: bool,
    rejected: bool,
}

impl SolidClassifier {
    pub fn new() -> Self {
        SolidClassifier {
            state: ClassState::Error,
            is_on_face: false,
            rejected: false,
        }
    }

    pub fn rejected(&self) -> bool {
        self.rejected
    }

    pub fn state(&self) -> ClassState {
        self.state
    }

    pub fn is_on_a_face(&self) -> bool {
        self.is_on_face
    }

    pub fn force_in(&mut self) {
        self.state = ClassState::In;
        self.rejected = false;
    }

    pub fn force_out(&mut self) {
        self.state = ClassState::Out;
        self.rejected = false;
    }
}

impl Default for SolidClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_creation() {
        let clf = SolidClassifier::new();
        assert!(!clf.rejected());
        assert_eq!(clf.state(), ClassState::Error);
        assert!(!clf.is_on_a_face());
    }

    #[test]
    fn test_force_in() {
        let mut clf = SolidClassifier::new();
        clf.force_in();
        assert_eq!(clf.state(), ClassState::In);
        assert!(!clf.rejected());
    }

    #[test]
    fn test_force_out() {
        let mut clf = SolidClassifier::new();
        clf.force_out();
        assert_eq!(clf.state(), ClassState::Out);
        assert!(!clf.rejected());
    }
}
