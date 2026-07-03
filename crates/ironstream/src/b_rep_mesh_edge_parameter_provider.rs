// FILE: b_rep_mesh_edge_parameter_provider.rs
// occt: BRepMesh_EdgeParameterProvider

/// Provider for edge parameters.
pub struct EdgeParameterProvider {
    parameters: Vec<f64>,
}

impl EdgeParameterProvider {
    /// Constructor.
    pub fn new() -> Self {
        EdgeParameterProvider {
            parameters: Vec::new(),
        }
    }

    /// Gets parameter at index.
    pub fn parameter(&self, index: usize) -> Option<f64> {
        self.parameters.get(index).copied()
    }

    /// Adds a parameter.
    pub fn add_parameter(&mut self, param: f64) {
        self.parameters.push(param);
    }

    /// Returns the number of parameters.
    pub fn size(&self) -> usize {
        self.parameters.len()
    }
}

impl Default for EdgeParameterProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_parameter_provider_new() {
        let provider = EdgeParameterProvider::new();
        assert_eq!(provider.size(), 0);
    }

    #[test]
    fn test_edge_parameter_provider_add() {
        let mut provider = EdgeParameterProvider::new();
        provider.add_parameter(0.5);
        assert_eq!(provider.size(), 1);
        assert_eq!(provider.parameter(0), Some(0.5));
    }
}
