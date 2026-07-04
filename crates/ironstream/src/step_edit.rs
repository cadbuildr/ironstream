// FILE: step_edit.rs
// occt: STEPEdit

/// STEP editing utilities
pub struct STEPEdit;

impl STEPEdit {
    pub fn new() -> Self {
        STEPEdit
    }
}

impl Default for STEPEdit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _edit = STEPEdit::new();
    }
}
