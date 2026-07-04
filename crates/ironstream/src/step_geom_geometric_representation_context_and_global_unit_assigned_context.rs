// FILE: step_geom_geometric_representation_context_and_global_unit_assigned_context.rs
// occt: StepGeom_GeometricRepresentationContextAndGlobalUnitAssignedContext

use std::sync::Arc;

#[derive(Clone)]
pub struct GeometricRepresentationContextAndGlobalUnitAssignedContext {
    context_identifier: Option<Arc<String>>,
    context_type: Option<Arc<String>>,
    coordinate_space_dimension: i32,
}

impl GeometricRepresentationContextAndGlobalUnitAssignedContext {
    pub fn new() -> Self {
        Self {
            context_identifier: None,
            context_type: None,
            coordinate_space_dimension: 0,
        }
    }

    pub fn init(
        &mut self,
        context_identifier: Option<String>,
        context_type: Option<String>,
        coordinate_space_dimension: i32,
    ) {
        self.context_identifier = context_identifier.map(|c| Arc::new(c));
        self.context_type = context_type.map(|c| Arc::new(c));
        self.coordinate_space_dimension = coordinate_space_dimension;
    }

    pub fn set_coordinate_space_dimension(&mut self, dim: i32) {
        self.coordinate_space_dimension = dim;
    }

    pub fn coordinate_space_dimension(&self) -> i32 {
        self.coordinate_space_dimension
    }

    pub fn context_identifier(&self) -> Option<String> {
        self.context_identifier.as_ref().map(|c| c.as_ref().clone())
    }

    pub fn context_type(&self) -> Option<String> {
        self.context_type.as_ref().map(|c| c.as_ref().clone())
    }
}

impl Default for GeometricRepresentationContextAndGlobalUnitAssignedContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ctx = GeometricRepresentationContextAndGlobalUnitAssignedContext::new();
        assert_eq!(ctx.coordinate_space_dimension(), 0);
    }
}
