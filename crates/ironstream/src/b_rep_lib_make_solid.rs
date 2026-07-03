// FILE: b_rep_lib_make_solid.rs
// occt: BRepLib_MakeSolid

use std::collections::HashMap;

/// Makes a solid from compsolid or shells.
#[derive(Debug, Clone)]
pub struct BRepLibMakeSolid {
    deleted_faces: Vec<()>,
}

impl BRepLibMakeSolid {
    /// Create an empty solid covering whole space.
    pub fn new() -> Self {
        BRepLibMakeSolid {
            deleted_faces: Vec::new(),
        }
    }

    /// Get deleted faces.
    pub fn deleted_faces(&self) -> &[()] {
        &self.deleted_faces
    }
}

impl Default for BRepLibMakeSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_solid_creation() {
        let ms = BRepLibMakeSolid::new();
        assert_eq!(ms.deleted_faces().len(), 0);
    }
}
