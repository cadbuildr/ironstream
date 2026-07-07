// FILE: step_shape_edge_loop.rs
// occt: StepShape_EdgeLoop

//! Representation of STEP entity EdgeLoop

#[derive(Clone, Debug)]
pub struct EdgeLoop {
    name: String,
    edge_list: Vec<String>, // Placeholder for OrientedEdge handles
}

impl EdgeLoop {
    /// Returns an EdgeLoop
    pub fn new() -> Self {
        EdgeLoop {
            name: String::new(),
            edge_list: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, edge_list: Vec<String>) {
        self.name = name;
        self.edge_list = edge_list;
    }

    /// Set EdgeList
    pub fn set_edge_list(&mut self, edge_list: Vec<String>) {
        self.edge_list = edge_list;
    }

    /// Returns EdgeList
    pub fn edge_list(&self) -> &[String] {
        &self.edge_list
    }

    /// Returns value at index (1-based)
    pub fn edge_list_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.edge_list.len() {
            Some(&self.edge_list[num - 1])
        } else {
            None
        }
    }

    /// Returns the number of edges
    pub fn nb_edge_list(&self) -> usize {
        self.edge_list.len()
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for EdgeLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loop_obj = EdgeLoop::new();
        assert_eq!(loop_obj.name(), "");
        assert_eq!(loop_obj.nb_edge_list(), 0);
    }

    #[test]
    fn test_init() {
        let mut loop_obj = EdgeLoop::new();
        loop_obj.init(
            "Loop1".to_string(),
            vec!["edge1".to_string(), "edge2".to_string()],
        );
        assert_eq!(loop_obj.name(), "Loop1");
        assert_eq!(loop_obj.nb_edge_list(), 2);
    }

    #[test]
    fn test_edge_list_value() {
        let mut loop_obj = EdgeLoop::new();
        loop_obj.set_edge_list(vec!["e1".to_string(), "e2".to_string(), "e3".to_string()]);
        assert_eq!(loop_obj.edge_list_value(1), Some(&"e1".to_string()));
        assert_eq!(loop_obj.edge_list_value(3), Some(&"e3".to_string()));
        assert_eq!(loop_obj.edge_list_value(4), None);
    }

    #[test]
    fn test_set_edge_list() {
        let mut loop_obj = EdgeLoop::new();
        loop_obj.set_edge_list(vec!["edge1".to_string()]);
        assert_eq!(loop_obj.nb_edge_list(), 1);
    }
}
