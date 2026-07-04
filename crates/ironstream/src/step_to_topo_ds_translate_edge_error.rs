// FILE: step_to_topo_ds_translate_edge_error.rs
// occt: StepToTopoDS_TranslateEdgeError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslateEdgeError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslateEdgeError {
    fn default() -> Self {
        StepToTopoDS_TranslateEdgeError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        let done = StepToTopoDS_TranslateEdgeError::Done;
        let other = StepToTopoDS_TranslateEdgeError::Other;
        assert_eq!(done, StepToTopoDS_TranslateEdgeError::Done);
        assert_eq!(other, StepToTopoDS_TranslateEdgeError::Other);
        assert_ne!(done, other);
    }

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslateEdgeError::default(),
            StepToTopoDS_TranslateEdgeError::Done
        );
    }

    #[test]
    fn test_copy() {
        let err = StepToTopoDS_TranslateEdgeError::Other;
        let err2 = err;
        assert_eq!(err, err2);
    }
}
