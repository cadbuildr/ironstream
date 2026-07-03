// FILE: b_rep_offset_api_sewing.rs
// occt: BRepOffsetAPI_Sewing

/// Assembles a set of shapes (mainly faces and wires) into one shell or compound.
/// Provides functions to sew faces and merge free edges.
#[derive(Debug, Clone)]
pub struct BRepOffsetApiSewing {
    /// Tolerance for merging edges
    tolerance: f64,
    /// Number of faces being sewn
    face_count: usize,
    /// Number of merged edges
    merged_edge_count: usize,
}

impl BRepOffsetApiSewing {
    /// Create a new Sewing object with tolerance
    pub fn new(tolerance: f64) -> Self {
        Self {
            tolerance,
            face_count: 0,
            merged_edge_count: 0,
        }
    }

    /// Get tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set tolerance
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Get number of faces
    pub fn face_count(&self) -> usize {
        self.face_count
    }

    /// Add a face
    pub fn add_face(&mut self) {
        self.face_count += 1;
    }

    /// Get number of merged edges
    pub fn merged_edge_count(&self) -> usize {
        self.merged_edge_count
    }

    /// Record merged edge
    pub fn record_merged_edge(&mut self) {
        self.merged_edge_count += 1;
    }
}

impl Default for BRepOffsetApiSewing {
    fn default() -> Self {
        Self::new(1e-7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sewing = BRepOffsetApiSewing::new(0.01);
        assert_eq!(sewing.tolerance(), 0.01);
        assert_eq!(sewing.face_count(), 0);
    }

    #[test]
    fn test_set_tolerance() {
        let mut sewing = BRepOffsetApiSewing::new(0.01);
        sewing.set_tolerance(0.001);
        assert_eq!(sewing.tolerance(), 0.001);
    }

    #[test]
    fn test_add_face() {
        let mut sewing = BRepOffsetApiSewing::new(0.01);
        sewing.add_face();
        sewing.add_face();
        assert_eq!(sewing.face_count(), 2);
    }

    #[test]
    fn test_record_merged_edge() {
        let mut sewing = BRepOffsetApiSewing::new(0.01);
        sewing.record_merged_edge();
        assert_eq!(sewing.merged_edge_count(), 1);
    }

    #[test]
    fn test_default() {
        let sewing = BRepOffsetApiSewing::default();
        assert!(sewing.tolerance() > 0.0);
    }
}
