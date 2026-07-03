// FILE: top_ope_b_rep_face_edge_intersector.rs
// occt: TopOpeBRep_FaceEdgeIntersector

/// FaceEdgeIntersector stub implementation.
pub struct FaceEdgeIntersector;

impl FaceEdgeIntersector {
    /// Create new instance.
    pub fn new() -> Self {
        FaceEdgeIntersector
    }
}

impl Default for FaceEdgeIntersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = FaceEdgeIntersector::new();
    }

    #[test]
    fn test_default() {
        let _ = FaceEdgeIntersector::default();
    }
}
