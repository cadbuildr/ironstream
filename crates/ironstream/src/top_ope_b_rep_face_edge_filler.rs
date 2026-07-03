// FILE: top_ope_b_rep_face_edge_filler.rs
// occt: TopOpeBRep_FaceEdgeFiller

/// FaceEdgeFiller stub implementation.
pub struct FaceEdgeFiller;

impl FaceEdgeFiller {
    /// Create new instance.
    pub fn new() -> Self {
        FaceEdgeFiller
    }
}

impl Default for FaceEdgeFiller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = FaceEdgeFiller::new();
    }

    #[test]
    fn test_default() {
        let _ = FaceEdgeFiller::default();
    }
}
