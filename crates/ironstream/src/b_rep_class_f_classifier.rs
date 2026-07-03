// FILE: b_rep_class_f_classifier.rs
// occt: BRepClass_FClassifier

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    On,
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Before,
    Middle,
    After,
    OnCC,
}

/// Classifier for points on a face
pub struct FaceClassifier {
    edge_parameter: f64,
    position: Position,
    rejected: bool,
    nowires: bool,
}

impl FaceClassifier {
    pub fn new() -> Self {
        FaceClassifier {
            edge_parameter: 0.0,
            position: Position::Middle,
            rejected: false,
            nowires: false,
        }
    }

    pub fn rejected(&self) -> bool {
        self.rejected
    }

    pub fn nowires(&self) -> bool {
        self.nowires
    }

    pub fn edge_parameter(&self) -> f64 {
        self.edge_parameter
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn state(&self) -> State {
        State::Out
    }
}

impl Default for FaceClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_creation() {
        let clf = FaceClassifier::new();
        assert!(!clf.rejected());
        assert!(!clf.nowires());
        assert_eq!(clf.state(), State::Out);
    }

    #[test]
    fn test_position() {
        let clf = FaceClassifier::new();
        assert_eq!(clf.position(), Position::Middle);
    }
}
