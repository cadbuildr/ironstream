// FILE: step_to_topo_ds_translate_edge_loop_error.rs
// occt: StepToTopoDS_TranslateEdgeLoopError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslateEdgeLoopError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslateEdgeLoopError {
    fn default() -> Self {
        StepToTopoDS_TranslateEdgeLoopError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslateEdgeLoopError::default(),
            StepToTopoDS_TranslateEdgeLoopError::Done
        );
    }
}
