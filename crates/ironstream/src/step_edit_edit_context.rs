// FILE: step_edit_edit_context.rs
// occt: STEPEdit_EditContext

/// Context for editing STEP
pub struct STEPEdit_EditContext;

impl STEPEdit_EditContext {
    pub fn new() -> Self {
        STEPEdit_EditContext
    }
}

impl Default for STEPEdit_EditContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ctx = STEPEdit_EditContext::new();
    }
}
