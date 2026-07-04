// FILE: step_to_topo_ds_translate_solid_error.rs
// occt: StepToTopoDS_TranslateSolidError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepToTopoDS_TranslateSolidError {
    Done,
    Other,
}

impl Default for StepToTopoDS_TranslateSolidError {
    fn default() -> Self {
        StepToTopoDS_TranslateSolidError::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            StepToTopoDS_TranslateSolidError::default(),
            StepToTopoDS_TranslateSolidError::Done
        );
    }
}
