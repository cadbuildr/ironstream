// FILE: step_fea_geometric_node.rs
// occt: StepFEA_GeometricNode

/// Representation of STEP entity GeometricNode
#[derive(Debug, Clone)]
pub struct StepFeaGeometricNode;

impl StepFeaGeometricNode {
    /// Creates a new GeometricNode
    pub fn new() -> Self {
        StepFeaGeometricNode
    }
}

impl Default for StepFeaGeometricNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometric_node_creation() {
        let node = StepFeaGeometricNode::new();
        let _ = node;
    }

    #[test]
    fn test_geometric_node_default() {
        let node = StepFeaGeometricNode::default();
        let _ = node;
    }
}
