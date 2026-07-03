// FILE: top_ope_b_rep_build_wire_edge_classifier.rs
// occt: TopOpeBRepBuild_WireEdgeClassifier

#[derive(Debug)]
pub struct TopOpeBRepBuildWireEdgeClassifier;

impl TopOpeBRepBuildWireEdgeClassifier {
    pub fn new() -> Self {
        TopOpeBRepBuildWireEdgeClassifier
    }

    pub fn compare(&self, _loop1_id: i32, _loop2_id: i32) -> u32 {
        3
    }

    pub fn classify_wire(&self, _wire_id: i32, _edge_id: i32) -> u32 {
        3
    }
}

impl Default for TopOpeBRepBuildWireEdgeClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_edge_classifier_new() {
        let _wec = TopOpeBRepBuildWireEdgeClassifier::new();
    }

    #[test]
    fn test_wire_edge_classifier_compare() {
        let wec = TopOpeBRepBuildWireEdgeClassifier::new();
        assert_eq!(wec.compare(1, 2), 3);
    }

    #[test]
    fn test_wire_edge_classifier_classify_wire() {
        let wec = TopOpeBRepBuildWireEdgeClassifier::new();
        assert_eq!(wec.classify_wire(1, 2), 3);
    }

    #[test]
    fn test_wire_edge_classifier_default() {
        let _wec = TopOpeBRepBuildWireEdgeClassifier::default();
    }
}
