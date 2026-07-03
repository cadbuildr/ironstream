// FILE: top_ope_b_rep_tool_shape_classifier.rs
// occt: TopOpeBRepTool_ShapeClassifier

/// ShapeClassifier stub implementation.
pub struct ShapeClassifier;

impl ShapeClassifier {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        ShapeClassifier
    }
}

impl Default for ShapeClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ShapeClassifier::new();
    }

    #[test]
    fn test_default() {
        let _ = ShapeClassifier::default();
    }
}
