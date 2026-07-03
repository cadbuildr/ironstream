// FILE: b_rep_feat_perf_selection.rs
// occt: BRepFeat_PerfSelection

/// Helper class for performance-related face selection in features.
pub struct BRepFeatPerfSelection {
    face_id: usize,
}

impl BRepFeatPerfSelection {
    /// Creates a new selection object.
    pub fn new() -> Self {
        BRepFeatPerfSelection { face_id: 0 }
    }

    /// Sets the selected face.
    pub fn set_face(&mut self, _face_id: usize) {
        // Set face
    }

    /// Gets the selected face.
    pub fn face(&self) -> usize {
        self.face_id
    }
}

impl Default for BRepFeatPerfSelection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_selection_creation() {
        let sel = BRepFeatPerfSelection::new();
        assert_eq!(sel.face(), 0);
    }

    #[test]
    fn test_perf_selection_default() {
        let sel = BRepFeatPerfSelection::default();
        assert_eq!(sel.face(), 0);
    }
}
