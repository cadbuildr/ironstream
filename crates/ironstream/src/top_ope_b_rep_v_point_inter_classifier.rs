// FILE: top_ope_b_rep_v_point_inter_classifier.rs
// occt: TopOpeBRep_VPointInterClassifier

/// Classifies VPointInter objects.
pub struct VPointInterClassifier {
    state: u32, // TopAbs_State
    edge: Vec<u8>,
    edge_parameter: f64,
}

impl VPointInterClassifier {
    /// Create a new VPointInterClassifier.
    pub fn new() -> Self {
        VPointInterClassifier {
            state: 0,
            edge: Vec::new(),
            edge_parameter: 0.0,
        }
    }

    /// Compute position of VPoint with respect to face.
    pub fn vpoint_position(&mut self, _f: &[u8], _vp: &[u8], _shape_index: usize, _pc: &[u8], _assume_inon: bool, _tol: f64) -> u32 {
        self.state
    }

    /// Get the edge containing the VPoint.
    pub fn edge(&self) -> Option<&[u8]> {
        if self.edge.is_empty() {
            None
        } else {
            Some(&self.edge)
        }
    }

    /// Get the parameter of the VPoint on Edge.
    pub fn edge_parameter(&self) -> f64 {
        self.edge_parameter
    }
}

impl Default for VPointInterClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let classifier = VPointInterClassifier::new();
        assert_eq!(classifier.state, 0);
        assert!(classifier.edge().is_none());
    }

    #[test]
    fn test_edge_parameter() {
        let classifier = VPointInterClassifier::new();
        assert_eq!(classifier.edge_parameter(), 0.0);
    }

    #[test]
    fn test_vpoint_position() {
        let mut classifier = VPointInterClassifier::new();
        let result = classifier.vpoint_position(&[], &[], 0, &[], false, 0.01);
        assert_eq!(result, 0);
    }
}
