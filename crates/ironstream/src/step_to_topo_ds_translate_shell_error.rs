// FILE: step_to_topo_ds_translate_shell_error.rs
// occt: StepToTopoDS_TranslateShellError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslateShellError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslateShellError {
    fn default() -> Self {
        StepToTopoDS_TranslateShellError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslateShellError::default(),
            StepToTopoDS_TranslateShellError::Done
        );
    }
}
