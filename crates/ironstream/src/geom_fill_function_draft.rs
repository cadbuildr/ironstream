// FILE: geom_fill_function_draft.rs
// occt: GeomFill_FunctionDraft

/// Function for draft surface generation
pub struct FunctionDraft;

impl FunctionDraft {
    pub fn new() -> Self {
        FunctionDraft
    }
}

impl Default for FunctionDraft {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _func = FunctionDraft::new();
    }

    #[test]
    fn test_default() {
        let _func = FunctionDraft::default();
    }
}
