// FILE: top_ope_b_rep_build_pave_classifier.rs
// occt: TopOpeBRepBuild_PaveClassifier

#[derive(Debug)]
pub struct TopOpeBRepBuildPaveClassifier;

impl TopOpeBRepBuildPaveClassifier {
    pub fn new() -> Self {
        TopOpeBRepBuildPaveClassifier
    }

    pub fn compare(&self, _pave1_id: i32, _pave2_id: i32) -> u32 {
        3
    }
}

impl Default for TopOpeBRepBuildPaveClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pave_classifier_new() {
        let _pc = TopOpeBRepBuildPaveClassifier::new();
    }

    #[test]
    fn test_pave_classifier_compare() {
        let pc = TopOpeBRepBuildPaveClassifier::new();
        assert_eq!(pc.compare(1, 2), 3);
    }

    #[test]
    fn test_pave_classifier_default() {
        let _pc = TopOpeBRepBuildPaveClassifier::default();
    }
}
