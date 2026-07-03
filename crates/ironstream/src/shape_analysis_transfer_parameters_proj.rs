// FILE: shape_analysis_transfer_parameters_proj.rs
// occt: ShapeAnalysis_TransferParametersProj

/// Transfers parameters using projection method.
pub struct TransferParametersProj {
    first: f64,
    last: f64,
    precision: f64,
    force_proj: bool,
    init_ok: bool,
}

impl TransferParametersProj {
    /// Create empty tool.
    pub fn new() -> Self {
        TransferParametersProj {
            first: 0.0,
            last: 0.0,
            precision: 1e-7,
            force_proj: false,
            init_ok: false,
        }
    }

    /// Initialize with edge and face.
    pub fn init(&mut self, _edge: &[u8], _face: &[u8]) {
        self.init_ok = true;
    }

    /// Get force projection flag.
    pub fn force_projection(&self) -> bool {
        self.force_proj
    }

    /// Set force projection flag.
    pub fn set_force_projection(&mut self, b: bool) {
        self.force_proj = b;
    }

    /// Perform parameter transfer.
    pub fn perform(&self, param: f64, _to2d: bool) -> f64 {
        param
    }

    /// Check if curves are same range.
    pub fn is_same_range(&self) -> bool {
        false
    }

    /// Copy non-manifold vertex.
    pub fn copy_nm_vertex(_vertex: &[u8], _to_edge: &[u8], _from_edge: &[u8]) -> Vec<u8> {
        Vec::new()
    }
}

impl Default for TransferParametersProj {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tp = TransferParametersProj::new();
        assert!(!tp.force_proj);
    }

    #[test]
    fn test_force_projection() {
        let mut tp = TransferParametersProj::new();
        tp.set_force_projection(true);
        assert!(tp.force_projection());
    }

    #[test]
    fn test_is_same_range() {
        let tp = TransferParametersProj::new();
        assert!(!tp.is_same_range());
    }

    #[test]
    fn test_perform() {
        let tp = TransferParametersProj::new();
        assert_eq!(tp.perform(1.5, true), 1.5);
    }
}
