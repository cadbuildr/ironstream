// FILE: b_rep_prim_face_builder.rs
// occt: BRepPrim_FaceBuilder

/// Builds a BRep Face from a Geom Surface.
#[derive(Debug, Clone)]
pub struct BRepPrimFaceBuilder;

impl BRepPrimFaceBuilder {
    pub fn new() -> Self {
        BRepPrimFaceBuilder
    }
}

impl Default for BRepPrimFaceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_builder() {
        let _fb = BRepPrimFaceBuilder::new();
    }
}
