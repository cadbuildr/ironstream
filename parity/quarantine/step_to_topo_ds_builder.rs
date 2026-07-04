// FILE: step_to_topo_ds_builder.rs
// occt: StepToTopoDS_Builder

use std::sync::Arc;

/// Error states for the Builder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderError {
    NotDone = 0,
    ManifoldSolidBrepError = 1,
    BrepWithVoidsError = 2,
    FacetedBrepError = 3,
    FacetedBrepAndBrepWithVoidsError = 4,
    ShellBasedSurfaceModelError = 5,
    GeometricSetError = 6,
}

/// Placeholder for TopoDS_Shape (result of topology building)
#[derive(Clone)]
pub struct TopoDS_Shape {
    id: usize,
}

impl TopoDS_Shape {
    pub fn new(id: usize) -> Self {
        TopoDS_Shape { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Builder for converting STEP shape representations to OpenCascade topology.
/// This builder translates various STEP shape representations into TopoDS shapes.
pub struct Builder {
    error: BuilderError,
    result: Option<Arc<TopoDS_Shape>>,
}

impl Builder {
    /// Create a new Builder
    pub fn new() -> Self {
        Builder {
            error: BuilderError::NotDone,
            result: None,
        }
    }

    /// Get the error status
    pub fn error(&self) -> BuilderError {
        self.error
    }

    /// Set the error status
    pub fn set_error(&mut self, error: BuilderError) {
        self.error = error;
    }

    /// Get the result shape
    pub fn value(&self) -> Option<&Arc<TopoDS_Shape>> {
        self.result.as_ref()
    }

    /// Set the result shape
    pub fn set_value(&mut self, shape: Arc<TopoDS_Shape>) {
        self.result = Some(shape);
    }

    /// Check if the builder has successfully produced a result
    pub fn is_done(&self) -> bool {
        self.error == BuilderError::NotDone && self.result.is_some()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = Builder::new();
        assert_eq!(builder.error(), BuilderError::NotDone);
        assert!(builder.value().is_none());
    }

    #[test]
    fn test_set_error() {
        let mut builder = Builder::new();
        builder.set_error(BuilderError::ManifoldSolidBrepError);
        assert_eq!(builder.error(), BuilderError::ManifoldSolidBrepError);
    }

    #[test]
    fn test_set_value() {
        let mut builder = Builder::new();
        let shape = Arc::new(TopoDS_Shape::new(42));

        builder.set_value(shape.clone());

        assert!(builder.value().is_some());
        assert_eq!(builder.value().unwrap().id(), 42);
    }

    #[test]
    fn test_is_done_no_result() {
        let builder = Builder::new();
        // Not done because there's no result
        assert!(!builder.is_done());
    }

    #[test]
    fn test_is_done_with_result() {
        let mut builder = Builder::new();
        let shape = Arc::new(TopoDS_Shape::new(1));

        builder.set_value(shape);
        // Done because error is NotDone and result exists
        assert!(builder.is_done());
    }

    #[test]
    fn test_is_done_with_error_and_result() {
        let mut builder = Builder::new();
        let shape = Arc::new(TopoDS_Shape::new(1));

        builder.set_value(shape);
        builder.set_error(BuilderError::FacetedBrepError);

        // Not done because there's an error
        assert!(!builder.is_done());
    }

    #[test]
    fn test_default() {
        let builder = Builder::default();
        assert_eq!(builder.error(), BuilderError::NotDone);
    }
}
