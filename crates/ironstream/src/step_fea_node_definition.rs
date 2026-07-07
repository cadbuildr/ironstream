// FILE: step_fea_node_definition.rs
// occt: StepFEA_NodeDefinition

/// Representation of STEP entity NodeDefinition
#[derive(Debug, Clone)]
pub struct StepFeaNodeDefinition;

impl StepFeaNodeDefinition {
    /// Creates a new NodeDefinition
    pub fn new() -> Self {
        StepFeaNodeDefinition
    }
}

impl Default for StepFeaNodeDefinition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_definition_creation() {
        let def = StepFeaNodeDefinition::new();
        let _ = def;
    }

    #[test]
    fn test_node_definition_default() {
        let def = StepFeaNodeDefinition::default();
        let _ = def;
    }
}
