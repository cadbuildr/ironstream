// FILE: step_fea_node.rs
// occt: StepFEA_Node

/// Representation of STEP entity Node
#[derive(Debug, Clone)]
pub struct StepFeaNode;

impl StepFeaNode {
    /// Creates a new Node
    pub fn new() -> Self {
        StepFeaNode
    }
}

impl Default for StepFeaNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = StepFeaNode::new();
        let _ = node;
    }

    #[test]
    fn test_node_default() {
        let node = StepFeaNode::default();
        let _ = node;
    }
}
