// FILE: geom2d_hatch_f_class2d_of_classifier.rs
// occt: Geom2dHatch_FClass2dOfClassifier

/// Classification state for 2D faces in hatching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FClass2dState {
    In,
    Out,
    On,
}

/// 2D face classification helper.
pub struct Geom2dHatchFClass2dOfClassifier {
    state: FClass2dState,
}

impl Geom2dHatchFClass2dOfClassifier {
    pub fn new() -> Self {
        Geom2dHatchFClass2dOfClassifier { state: FClass2dState::Out }
    }

    pub fn set_state(&mut self, state: FClass2dState) {
        self.state = state;
    }

    pub fn state(&self) -> FClass2dState {
        self.state
    }
}

impl Default for Geom2dHatchFClass2dOfClassifier {
    fn default() -> Self {
        Geom2dHatchFClass2dOfClassifier::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fclass2d_creation() {
        let fc = Geom2dHatchFClass2dOfClassifier::new();
        assert_eq!(fc.state(), FClass2dState::Out);
    }
}
