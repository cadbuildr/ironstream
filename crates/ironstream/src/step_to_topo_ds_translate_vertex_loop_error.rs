// FILE: step_to_topo_ds_translate_vertex_loop_error.rs
// occt: StepToTopoDS_TranslateVertexLoopError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslateVertexLoopError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslateVertexLoopError {
    fn default() -> Self {
        StepToTopoDS_TranslateVertexLoopError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslateVertexLoopError::default(),
            StepToTopoDS_TranslateVertexLoopError::Done
        );
    }
}
