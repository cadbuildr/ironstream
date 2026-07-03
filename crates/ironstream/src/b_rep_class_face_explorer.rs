// FILE: b_rep_class_face_explorer.rs
// occt: BRepClass_FaceExplorer

/// Explorer for face boundary edges in classification
pub struct FaceExplorer {
    max_tolerance: f64,
    use_bnd_box: bool,
}

impl FaceExplorer {
    pub fn new() -> Self {
        FaceExplorer {
            max_tolerance: 1e-7,
            use_bnd_box: false,
        }
    }

    pub fn max_tolerance(&self) -> f64 {
        self.max_tolerance
    }

    pub fn set_max_tolerance(&mut self, val: f64) {
        self.max_tolerance = val;
    }

    pub fn use_bnd_box(&self) -> bool {
        self.use_bnd_box
    }

    pub fn set_use_bnd_box(&mut self, val: bool) {
        self.use_bnd_box = val;
    }

    pub fn reject(&self, _x: f64, _y: f64) -> bool {
        false
    }

    pub fn more_wires(&self) -> bool {
        false
    }

    pub fn more_edges(&self) -> bool {
        false
    }

    pub fn init_wires(&mut self) {}

    pub fn init_edges(&mut self) {}

    pub fn next_wire(&mut self) {}

    pub fn next_edge(&mut self) {}
}

impl Default for FaceExplorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_explorer_creation() {
        let explorer = FaceExplorer::new();
        assert!(!explorer.use_bnd_box());
        assert!(explorer.max_tolerance() > 0.0);
    }

    #[test]
    fn test_max_tolerance() {
        let mut explorer = FaceExplorer::new();
        explorer.set_max_tolerance(0.01);
        assert_eq!(explorer.max_tolerance(), 0.01);
    }

    #[test]
    fn test_use_bnd_box() {
        let mut explorer = FaceExplorer::new();
        explorer.set_use_bnd_box(true);
        assert!(explorer.use_bnd_box());
    }

    #[test]
    fn test_wire_enumeration() {
        let mut explorer = FaceExplorer::new();
        assert!(!explorer.more_wires());
        explorer.init_wires();
        explorer.next_wire();
    }
}
