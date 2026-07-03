// FILE: i_mesh_tools_model_builder.rs
// occt: IMeshTools_ModelBuilder

/// Interface class for tool building discrete model.
pub trait ModelBuilder {
    /// Creates discrete model for the given shape.
    /// Returns model data or None in case of failure.
    fn perform_internal(&self, shape: &str, params: &str) -> Option<String>;

    /// Exceptions protected method to create discrete model.
    fn perform(&self, shape: &str, params: &str) -> Option<String> {
        // In Rust, we simply delegate to internal implementation
        self.perform_internal(shape, params)
    }
}

/// Default implementation for testing.
pub struct DefaultModelBuilder {
    success: bool,
}

impl DefaultModelBuilder {
    /// Create builder that will succeed or fail.
    pub fn new(success: bool) -> Self {
        DefaultModelBuilder { success }
    }
}

impl ModelBuilder for DefaultModelBuilder {
    fn perform_internal(&self, shape: &str, _params: &str) -> Option<String> {
        if self.success {
            Some(format!("model_for_{}", shape))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_success() {
        let builder = DefaultModelBuilder::new(true);
        let result = builder.perform("shape1", "params");
        assert_eq!(result, Some("model_for_shape1".to_string()));
    }

    #[test]
    fn test_default_failure() {
        let builder = DefaultModelBuilder::new(false);
        let result = builder.perform("shape1", "params");
        assert_eq!(result, None);
    }
}
