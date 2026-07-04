// FILE: step_select_file_modifier.rs
// occt: StepSelect_FileModifier

/// Represents a file modifier for STEP files
pub trait FileModifier {
    /// Perform the file modification
    fn perform(&self);
}

/// Default implementation of FileModifier
pub struct DefaultFileModifier {
    keep_graph: bool,
}

impl DefaultFileModifier {
    /// Create a new DefaultFileModifier
    pub fn new() -> Self {
        DefaultFileModifier { keep_graph: true }
    }

    /// Get the keep_graph flag
    pub fn keep_graph(&self) -> bool {
        self.keep_graph
    }

    /// Set the keep_graph flag
    pub fn set_keep_graph(&mut self, keep_graph: bool) {
        self.keep_graph = keep_graph;
    }
}

impl Default for DefaultFileModifier {
    fn default() -> Self {
        Self::new()
    }
}

impl FileModifier for DefaultFileModifier {
    fn perform(&self) {
        // Default implementation does nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let modifier = DefaultFileModifier::new();
        assert!(modifier.keep_graph());
    }

    #[test]
    fn test_set_keep_graph() {
        let mut modifier = DefaultFileModifier::new();
        modifier.set_keep_graph(false);
        assert!(!modifier.keep_graph());
    }

    #[test]
    fn test_trait_implementation() {
        let modifier = DefaultFileModifier::new();
        let _: &dyn FileModifier = &modifier;
        modifier.perform(); // Just verify it can be called
    }
}
