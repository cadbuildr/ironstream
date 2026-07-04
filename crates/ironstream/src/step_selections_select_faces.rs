// FILE: step_selections_select_faces.rs
// occt: STEPSelections_SelectFaces

/// Selector for faces in STEP
pub struct STEPSelections_SelectFaces;

impl STEPSelections_SelectFaces {
    pub fn new() -> Self {
        STEPSelections_SelectFaces
    }
}

impl Default for STEPSelections_SelectFaces {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = STEPSelections_SelectFaces::new();
    }
}
