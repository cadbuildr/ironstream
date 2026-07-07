// FILE: topo_ds_to_step_make_step_vertex.rs
// occt: TopoDSToStep_MakeStepVertex

use super::topo_ds_to_step_make_vertex_error::MakeVertexError;

pub struct MakeStepVertex {
    result: Option<TopologicalRepresentationItem>,
    error: MakeVertexError,
}

pub struct TopologicalRepresentationItem;

impl MakeStepVertex {
    pub fn new() -> Self {
        MakeStepVertex {
            result: None,
            error: MakeVertexError::VertexDone,
        }
    }

    pub fn value(&self) -> Option<&TopologicalRepresentationItem> {
        self.result.as_ref()
    }

    pub fn error(&self) -> MakeVertexError {
        self.error
    }
}

impl Default for MakeStepVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeStepVertex::new();
        assert!(maker.value().is_none());
    }
}
