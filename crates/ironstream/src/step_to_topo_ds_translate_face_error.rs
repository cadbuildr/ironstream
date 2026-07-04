// FILE: step_to_topo_ds_translate_face_error.rs
// occt: StepToTopoDS_TranslateFaceError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslateFaceError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslateFaceError {
    fn default() -> Self {
        StepToTopoDS_TranslateFaceError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslateFaceError::default(),
            StepToTopoDS_TranslateFaceError::Done
        );
    }
}
