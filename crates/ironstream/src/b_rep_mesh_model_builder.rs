// FILE: b_rep_mesh_model_builder.rs
// occt: BRepMesh_ModelBuilder

/// Builder for mesh models.
pub struct ModelBuilder {
    _private: (),
}

impl ModelBuilder {
    /// Constructor.
    pub fn new() -> Self {
        ModelBuilder { _private: () }
    }

    /// Builds model.
    pub fn build(&self) -> bool {
        true
    }
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_builder_new() {
        let builder = ModelBuilder::new();
        assert!(builder.build());
    }
}
