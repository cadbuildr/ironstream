// FILE: geom2d_hatch_classifier.rs
// occt: Geom2dHatch_Classifier

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopAbsState {
    In,
    Out,
    On,
}

/// Classifier for hatching.
pub struct Geom2dHatchClassifier {
    state: TopAbsState,
}

impl Geom2dHatchClassifier {
    pub fn new() -> Self {
        Geom2dHatchClassifier { state: TopAbsState::Out }
    }

    pub fn set_state(&mut self, state: TopAbsState) {
        self.state = state;
    }

    pub fn state(&self) -> TopAbsState {
        self.state
    }

    pub fn is_in(&self) -> bool {
        self.state == TopAbsState::In
    }

    pub fn is_out(&self) -> bool {
        self.state == TopAbsState::Out
    }

    pub fn is_on(&self) -> bool {
        self.state == TopAbsState::On
    }
}

impl Default for Geom2dHatchClassifier {
    fn default() -> Self {
        Geom2dHatchClassifier::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_default() {
        let clf = Geom2dHatchClassifier::new();
        assert!(clf.is_out());
    }

    #[test]
    fn test_set_state() {
        let mut clf = Geom2dHatchClassifier::new();
        clf.set_state(TopAbsState::In);
        assert!(clf.is_in());
        assert!(!clf.is_out());
    }
}
