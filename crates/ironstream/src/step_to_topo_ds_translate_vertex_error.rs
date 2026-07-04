// FILE: step_to_topo_ds_translate_vertex_error.rs
// occt: StepToTopoDS_TranslateVertexError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslateVertexError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslateVertexError {
    fn default() -> Self {
        StepToTopoDS_TranslateVertexError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslateVertexError::default(),
            StepToTopoDS_TranslateVertexError::Done
        );
    }
}
