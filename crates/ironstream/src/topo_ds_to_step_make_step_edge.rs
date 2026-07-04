// FILE: topo_ds_to_step_make_step_edge.rs
// occt: TopoDSToStep_MakeStepEdge

use super::topo_ds_to_step_make_edge_error::MakeEdgeError;

pub struct MakeStepEdge {
    result: Option<TopologicalRepresentationItem>,
    error: MakeEdgeError,
}

pub struct TopologicalRepresentationItem;

impl MakeStepEdge {
    pub fn new() -> Self {
        MakeStepEdge {
            result: None,
            error: MakeEdgeError::EdgeDone,
        }
    }

    pub fn value(&self) -> Option<&TopologicalRepresentationItem> {
        self.result.as_ref()
    }

    pub fn error(&self) -> MakeEdgeError {
        self.error
    }
}

impl Default for MakeStepEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeStepEdge::new();
        assert!(maker.value().is_none());
        assert_eq!(maker.error(), MakeEdgeError::EdgeDone);
    }
}
