// FILE: step_geom_geometric_representation_context_and_parametric_representation_context.rs
// occt: StepGeom_GeometricRepresentationContextAndParametricRepresentationContext

use std::sync::Arc;

#[derive(Clone)]
pub struct GeometricRepresentationContextAndParametricRepresentationContext {
    context_identifier: Option<Arc<String>>,
    context_type: Option<Arc<String>>,
    coordinate_space_dimension: i32,
}

impl GeometricRepresentationContextAndParametricRepresentationContext {
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
}

impl Default for GeometricRepresentationContextAndParametricRepresentationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ctx = GeometricRepresentationContextAndParametricRepresentationContext::new();
        assert_eq!(ctx.coordinate_space_dimension(), 0);
    }
}
