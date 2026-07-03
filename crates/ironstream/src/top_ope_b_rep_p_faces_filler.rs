// FILE: top_ope_b_rep_p_faces_filler.rs
// occt: TopOpeBRep_PFacesFiller

/// Pointer type alias for FacesFiller.
pub type PFacesFiller = Option<Box<FacesFiller>>;

/// Faces filler structure.
pub struct FacesFiller {
    data: Vec<u8>,
}

impl FacesFiller {
    /// Create a new FacesFiller.
    pub fn new() -> Self {
        FacesFiller { data: Vec::new() }
    }
}

impl Default for FacesFiller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let filler = FacesFiller::new();
        assert_eq!(filler.data.len(), 0);
    }

    #[test]
    fn test_pointer_type() {
        let p: PFacesFiller = Some(Box::new(FacesFiller::new()));
        assert!(p.is_some());
    }

    #[test]
    fn test_null_pointer() {
        let p: PFacesFiller = None;
        assert!(p.is_none());
    }
}
