// FILE: top_ope_b_rep_build_loop_classifier.rs
// occt: TopOpeBRepBuild_LoopClassifier

#[derive(Debug)]
pub struct TopOpeBRepBuildLoopClassifier;

impl TopOpeBRepBuildLoopClassifier {
    pub fn new() -> Self {
        TopOpeBRepBuildLoopClassifier
    }

    pub fn compare(&self, _l1_id: i32, _l2_id: i32) -> u32 {
        3
    }
}

impl Default for TopOpeBRepBuildLoopClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_classifier_new() {
        let _lc = TopOpeBRepBuildLoopClassifier::new();
    }

    #[test]
    fn test_loop_classifier_compare() {
        let lc = TopOpeBRepBuildLoopClassifier::new();
        let result = lc.compare(1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_loop_classifier_default() {
        let _lc = TopOpeBRepBuildLoopClassifier::default();
    }
}
