// FILE: step_fea_node_with_vector.rs
// occt: StepFEA_NodeWithVector

/// Representation of STEP entity NodeWithVector
#[derive(Debug, Clone)]
pub struct StepFeaNodeWithVector;

impl StepFeaNodeWithVector {
    /// Creates a new NodeWithVector
    pub fn new() -> Self {
        StepFeaNodeWithVector
    }
}

impl Default for StepFeaNodeWithVector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_with_vector_creation() {
        let node = StepFeaNodeWithVector::new();
        let _ = node;
    }

    #[test]
    fn test_node_with_vector_default() {
        let node = StepFeaNodeWithVector::default();
        let _ = node;
    }
}
