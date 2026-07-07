// FILE: step_fea_node_group.rs
// occt: StepFEA_NodeGroup

/// Representation of STEP entity NodeGroup
#[derive(Debug, Clone)]
pub struct StepFeaNodeGroup {
    name: String,
    description: String,
    model_ref: Option<i32>,
    nodes: Vec<i32>,
}

impl StepFeaNodeGroup {
    /// Creates a new empty NodeGroup
    pub fn new() -> Self {
        StepFeaNodeGroup {
            name: String::new(),
            description: String::new(),
            model_ref: None,
            nodes: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        model_ref: Option<i32>,
        nodes: Vec<i32>,
    ) {
        self.name = name;
        self.description = description;
        self.model_ref = model_ref;
        self.nodes = nodes;
    }

    /// Returns field Nodes
    pub fn nodes(&self) -> &[i32] {
        &self.nodes
    }

    /// Set field Nodes
    pub fn set_nodes(&mut self, nodes: Vec<i32>) {
        self.nodes = nodes;
    }

    /// Returns field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns field description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set field description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns field ModelRef
    pub fn model_ref(&self) -> Option<i32> {
        self.model_ref
    }

    /// Set field ModelRef
    pub fn set_model_ref(&mut self, model_ref: Option<i32>) {
        self.model_ref = model_ref;
    }
}

impl Default for StepFeaNodeGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_group_creation() {
        let group = StepFeaNodeGroup::new();
        assert_eq!(group.name(), "");
        assert_eq!(group.description(), "");
        assert_eq!(group.model_ref(), None);
        assert_eq!(group.nodes().len(), 0);
    }

    #[test]
    fn test_node_group_init() {
        let mut group = StepFeaNodeGroup::new();
        group.init(
            "Group".to_string(),
            "Description".to_string(),
            Some(1),
            vec![1, 2, 3],
        );

        assert_eq!(group.name(), "Group");
        assert_eq!(group.description(), "Description");
        assert_eq!(group.model_ref(), Some(1));
        assert_eq!(group.nodes(), &[1, 2, 3]);
    }

    #[test]
    fn test_node_group_setters() {
        let mut group = StepFeaNodeGroup::new();
        group.set_name("Test".to_string());
        group.set_description("Test Desc".to_string());
        group.set_model_ref(Some(2));
        group.set_nodes(vec![4, 5]);

        assert_eq!(group.name(), "Test");
        assert_eq!(group.description(), "Test Desc");
        assert_eq!(group.model_ref(), Some(2));
        assert_eq!(group.nodes(), &[4, 5]);
    }
}
