// FILE: step_to_topo_ds_translate_poly_loop_error.rs
// occt: StepToTopoDS_TranslatePolyLoopError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslatePolyLoopError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslatePolyLoopError {
    fn default() -> Self {
        StepToTopoDS_TranslatePolyLoopError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslatePolyLoopError::default(),
            StepToTopoDS_TranslatePolyLoopError::Done
        );
    }
}
