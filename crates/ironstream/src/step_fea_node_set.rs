// FILE: step_fea_node_set.rs
// occt: StepFEA_NodeSet

/// Representation of STEP entity NodeSet
#[derive(Debug, Clone)]
pub struct StepFeaNodeSet {
    name: String,
    nodes: Vec<i32>,
}

impl StepFeaNodeSet {
    /// Creates a new empty NodeSet
    pub fn new() -> Self {
        StepFeaNodeSet {
            name: String::new(),
            nodes: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, nodes: Vec<i32>) {
        self.name = name;
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
}

impl Default for StepFeaNodeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_set_creation() {
        let set = StepFeaNodeSet::new();
        assert_eq!(set.name(), "");
        assert_eq!(set.nodes().len(), 0);
    }

    #[test]
    fn test_node_set_init() {
        let mut set = StepFeaNodeSet::new();
        set.init("NodeSet".to_string(), vec![10, 11, 12]);

        assert_eq!(set.name(), "NodeSet");
        assert_eq!(set.nodes(), &[10, 11, 12]);
    }

    #[test]
    fn test_node_set_setters() {
        let mut set = StepFeaNodeSet::new();
        set.set_name("Test".to_string());
        set.set_nodes(vec![20, 21]);

        assert_eq!(set.name(), "Test");
        assert_eq!(set.nodes(), &[20, 21]);
    }
}
