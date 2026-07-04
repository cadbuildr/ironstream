// FILE: step_shape_edge_based_wireframe_model.rs
// occt: StepShape_EdgeBasedWireframeModel

//! Representation of STEP entity EdgeBasedWireframeModel

#[derive(Clone, Debug)]
pub struct EdgeBasedWireframeModel {
    name: String,
    ebwm_boundary: Vec<String>, // Placeholder for ConnectedEdgeSet handles
}

impl EdgeBasedWireframeModel {
    /// Empty constructor
    pub fn new() -> Self {
        EdgeBasedWireframeModel {
            name: String::new(),
            ebwm_boundary: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, boundary: Vec<String>) {
        self.name = name;
        self.ebwm_boundary = boundary;
    }

    /// Returns field EbwmBoundary
    pub fn ebwm_boundary(&self) -> &[String] {
        &self.ebwm_boundary
    }

    /// Set field EbwmBoundary
    pub fn set_ebwm_boundary(&mut self, boundary: Vec<String>) {
        self.ebwm_boundary = boundary;
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

impl Default for EdgeBasedWireframeModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let model = EdgeBasedWireframeModel::new();
        assert_eq!(model.name(), "");
        assert_eq!(model.ebwm_boundary().len(), 0);
    }

    #[test]
    fn test_init() {
        let mut model = EdgeBasedWireframeModel::new();
        model.init(
            "Model1".to_string(),
            vec!["edge_set1".to_string(), "edge_set2".to_string()],
        );
        assert_eq!(model.name(), "Model1");
        assert_eq!(model.ebwm_boundary().len(), 2);
    }

    #[test]
    fn test_set_ebwm_boundary() {
        let mut model = EdgeBasedWireframeModel::new();
        model.set_ebwm_boundary(vec!["es1".to_string(), "es2".to_string()]);
        assert_eq!(model.ebwm_boundary().len(), 2);
    }
}
