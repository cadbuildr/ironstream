// FILE: i_mesh_tools_context.rs
// occt: IMeshTools_Context

/// Context of BRepMesh algorithm.
/// Caches discrete model and instances of tools for its processing.
pub struct IMeshToolsContext {
    shape_data: Option<String>,
    model: Option<String>,
    parameters: IMeshToolsParameters,
    model_builder: Option<Box<dyn ModelBuilder>>,
    edge_discret: Option<Box<dyn ModelAlgo>>,
    model_healer: Option<Box<dyn ModelAlgo>>,
    pre_processor: Option<Box<dyn ModelAlgo>>,
    face_discret: Option<Box<dyn ModelAlgo>>,
    post_processor: Option<Box<dyn ModelAlgo>>,
}

/// Parameters for meshing.
pub struct IMeshToolsParameters {
    pub clean_model: bool,
}

impl Default for IMeshToolsParameters {
    fn default() -> Self {
        IMeshToolsParameters { clean_model: false }
    }
}

/// Interface for model builder.
pub trait ModelBuilder {
    fn perform(&self, shape: &str, params: &IMeshToolsParameters) -> Option<String>;
}

/// Interface for model algorithm.
pub trait ModelAlgo {
    fn perform(&self, model: &str, params: &IMeshToolsParameters) -> bool;
}

impl IMeshToolsContext {
    /// Create new context.
    pub fn new() -> Self {
        IMeshToolsContext {
            shape_data: None,
            model: None,
            parameters: IMeshToolsParameters::default(),
            model_builder: None,
            edge_discret: None,
            model_healer: None,
            pre_processor: None,
            face_discret: None,
            post_processor: None,
        }
    }

    /// Set shape.
    pub fn set_shape(&mut self, shape: String) {
        self.shape_data = Some(shape);
    }

    /// Get shape.
    pub fn get_shape(&self) -> Option<&String> {
        self.shape_data.as_ref()
    }

    /// Build model using assigned model builder.
    pub fn build_model(&mut self) -> bool {
        match (&self.model_builder, &self.shape_data) {
            (Some(builder), Some(shape)) => {
                if let Some(model) = builder.perform(shape, &self.parameters) {
                    self.model = Some(model);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Discretize edges.
    pub fn discretize_edges(&self) -> bool {
        match (&self.model, &self.edge_discret) {
            (Some(model), Some(algo)) => algo.perform(model, &self.parameters),
            _ => false,
        }
    }

    /// Heal model.
    pub fn heal_model(&self) -> bool {
        match &self.model {
            Some(model) => match &self.model_healer {
                Some(healer) => healer.perform(model, &self.parameters),
                None => true,
            },
            None => false,
        }
    }

    /// Pre-process model.
    pub fn pre_process_model(&self) -> bool {
        match &self.model {
            Some(model) => match &self.pre_processor {
                Some(proc) => proc.perform(model, &self.parameters),
                None => true,
            },
            None => false,
        }
    }

    /// Discretize faces.
    pub fn discretize_faces(&self) -> bool {
        match (&self.model, &self.face_discret) {
            (Some(model), Some(algo)) => algo.perform(model, &self.parameters),
            _ => false,
        }
    }

    /// Post-process model.
    pub fn post_process_model(&self) -> bool {
        match &self.model {
            Some(model) => match &self.post_processor {
                Some(proc) => proc.perform(model, &self.parameters),
                None => true,
            },
            None => false,
        }
    }

    /// Clean temporary context data.
    pub fn clean(&mut self) {
        if self.parameters.clean_model {
            self.model = None;
        }
    }

    /// Get model.
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Get parameters.
    pub fn get_parameters(&self) -> &IMeshToolsParameters {
        &self.parameters
    }

    /// Change parameters.
    pub fn change_parameters(&mut self) -> &mut IMeshToolsParameters {
        &mut self.parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ctx = IMeshToolsContext::new();
        assert_eq!(ctx.get_shape(), None);
        assert_eq!(ctx.get_model(), None);
    }

    #[test]
    fn test_set_shape() {
        let mut ctx = IMeshToolsContext::new();
        ctx.set_shape("shape".to_string());
        assert_eq!(ctx.get_shape(), Some(&"shape".to_string()));
    }

    #[test]
    fn test_build_model_fails_without_builder() {
        let mut ctx = IMeshToolsContext::new();
        ctx.set_shape("shape".to_string());
        assert!(!ctx.build_model());
    }

    #[test]
    fn test_clean() {
        let mut ctx = IMeshToolsContext::new();
        ctx.model = Some("model".to_string());
        ctx.parameters.clean_model = true;
        ctx.clean();
        assert_eq!(ctx.get_model(), None);
    }
}
