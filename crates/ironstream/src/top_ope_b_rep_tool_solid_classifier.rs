// FILE: top_ope_b_rep_tool_solid_classifier.rs
// occt: TopOpeBRepTool_SolidClassifier

/// SolidClassifier stub implementation.
pub struct SolidClassifier;

impl SolidClassifier {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        SolidClassifier
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
    fn test_new() {
        let _ = SolidClassifier::new();
    }

    #[test]
    fn test_default() {
        let _ = SolidClassifier::default();
    }
}
