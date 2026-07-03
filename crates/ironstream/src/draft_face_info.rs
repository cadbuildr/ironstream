// FILE: draft_face_info.rs
// occt: Draft_FaceInfo

/// Information container for a face in draft operations.
pub struct DraftFaceInfo {
    new_geometry: bool,
}

impl DraftFaceInfo {
    /// Creates a new empty face info.
    pub fn new() -> Self {
        DraftFaceInfo {
            new_geometry: false,
        }
    }

    /// Creates face info with a geometry flag.
    pub fn with_geometry(has_new_geometry: bool) -> Self {
        DraftFaceInfo {
            new_geometry: has_new_geometry,
        }
    }

    /// Returns whether this face has new geometry.
    pub fn new_geometry(&self) -> bool {
        self.new_geometry
    }

    /// Sets whether this face has new geometry.
    pub fn set_new_geometry(&mut self, has_new: bool) {
        self.new_geometry = has_new;
    }
}

impl Default for DraftFaceInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_face_info_creation() {
        let info = DraftFaceInfo::new();
        assert!(!info.new_geometry());
    }

    #[test]
    fn test_draft_face_info_with_geometry() {
        let info = DraftFaceInfo::with_geometry(true);
        assert!(info.new_geometry());
    }

    #[test]
    fn test_draft_face_info_set_geometry() {
        let mut info = DraftFaceInfo::new();
        info.set_new_geometry(true);
        assert!(info.new_geometry());
        info.set_new_geometry(false);
        assert!(!info.new_geometry());
    }

    #[test]
    fn test_draft_face_info_default() {
        let info = DraftFaceInfo::default();
        assert!(!info.new_geometry());
    }
}
