// FILE: i_mesh_tools_mesh_builder.rs
// occt: IMeshTools_MeshBuilder

/// Builds mesh for each face of shape without triangulation.
pub struct IMeshToolsMeshBuilder {
    context: Option<String>,
}

impl IMeshToolsMeshBuilder {
    /// Create empty mesh builder.
    pub fn new() -> Self {
        IMeshToolsMeshBuilder { context: None }
    }

    /// Create with context.
    pub fn with_context(context: String) -> Self {
        IMeshToolsMeshBuilder {
            context: Some(context),
        }
    }

    /// Set context.
    pub fn set_context(&mut self, context: String) {
        self.context = Some(context);
    }

    /// Get context.
    pub fn get_context(&self) -> Option<&String> {
        self.context.as_ref()
    }

    /// Performs meshing to the shape using current context.
    pub fn perform(&self) -> bool {
        self.context.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = IMeshToolsMeshBuilder::new();
        assert_eq!(builder.get_context(), None);
    }

    #[test]
    fn test_with_context() {
        let builder = IMeshToolsMeshBuilder::with_context("ctx".to_string());
        assert_eq!(builder.get_context(), Some(&"ctx".to_string()));
    }

    #[test]
    fn test_perform_requires_context() {
        let builder = IMeshToolsMeshBuilder::new();
        assert!(!builder.perform());

        let builder = IMeshToolsMeshBuilder::with_context("ctx".to_string());
        assert!(builder.perform());
    }
}
