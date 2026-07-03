// FILE: b_rep_graph_layer_parametric.rs
// occt: BRepGraph_LayerParametric

use std::collections::HashMap;

/// Parametric evaluation layer for graph nodes.
pub struct BRepGraphLayerParametric {
    parameters: HashMap<u32, Vec<f64>>,
    constraints: HashMap<u32, String>,
}

impl BRepGraphLayerParametric {
    /// Create a new parametric layer.
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    /// Get parameters for a node.
    pub fn get_parameters(&self, node_id: u32) -> Option<&[f64]> {
        self.parameters.get(&node_id).map(|v| v.as_slice())
    }

    /// Set parameters for a node.
    pub fn set_parameters(&mut self, node_id: u32, params: Vec<f64>) {
        self.parameters.insert(node_id, params);
    }

    /// Get constraint for a node.
    pub fn get_constraint(&self, node_id: u32) -> Option<&str> {
        self.constraints.get(&node_id).map(|s| s.as_str())
    }

    /// Set constraint for a node.
    pub fn set_constraint(&mut self, node_id: u32, constraint: String) {
        self.constraints.insert(node_id, constraint);
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.parameters.clear();
        self.constraints.clear();
    }
}

impl Default for BRepGraphLayerParametric {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let layer = BRepGraphLayerParametric::new();
        assert_eq!(layer.parameters.len(), 0);
    }

    #[test]
    fn test_parameters() {
        let mut layer = BRepGraphLayerParametric::new();
        layer.set_parameters(1, vec![1.0, 2.0, 3.0]);

        assert_eq!(layer.get_parameters(1), Some(&[1.0, 2.0, 3.0][..]));
        assert_eq!(layer.get_parameters(2), None);
    }

    #[test]
    fn test_constraint() {
        let mut layer = BRepGraphLayerParametric::new();
        layer.set_constraint(1, "x > 0".to_string());

        assert_eq!(layer.get_constraint(1), Some("x > 0"));
        assert_eq!(layer.get_constraint(2), None);
    }
}
