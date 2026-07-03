// FILE: b_rep_check_face.rs
// occt: BRepCheck_Face

/// Check face result class
pub struct FaceChecker {
    geometric_controls: bool,
    is_unorientable: bool,
}

impl FaceChecker {
    pub fn new() -> Self {
        FaceChecker {
            geometric_controls: false,
            is_unorientable: false,
        }
    }

    pub fn geometric_controls(&self) -> bool {
        self.geometric_controls
    }

    pub fn set_geometric_controls(&mut self, flag: bool) {
        self.geometric_controls = flag;
    }

    pub fn is_unorientable(&self) -> bool {
        self.is_unorientable
    }

    pub fn set_unorientable(&mut self) {
        self.is_unorientable = true;
    }
}

impl Default for FaceChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_checker_creation() {
        let checker = FaceChecker::new();
        assert!(!checker.geometric_controls());
        assert!(!checker.is_unorientable());
    }

    #[test]
    fn test_geometric_controls() {
        let mut checker = FaceChecker::new();
        checker.set_geometric_controls(true);
        assert!(checker.geometric_controls());
    }

    #[test]
    fn test_unorientable() {
        let mut checker = FaceChecker::new();
        assert!(!checker.is_unorientable());
        checker.set_unorientable();
        assert!(checker.is_unorientable());
    }
}
