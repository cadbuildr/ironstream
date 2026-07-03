// FILE: b_rep_offset_api_make_evolved.rs
// occt: BRepOffsetAPI_MakeEvolved

/// Describes functions to build evolved shapes.
/// An evolved shape is built from a planar spine (face or wire) and a profile (wire).
/// The evolved shape is the unlooped sweep (pipe) of the profile along the spine.
#[derive(Debug, Clone)]
pub struct BRepOffsetApiMakeEvolved {
    /// Whether the operation was successful
    is_done: bool,
}

impl BRepOffsetApiMakeEvolved {
    /// Create a new MakeEvolved builder
    pub fn new() -> Self {
        Self { is_done: false }
    }

    /// Check if the operation succeeded
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark operation as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }
}

impl Default for BRepOffsetApiMakeEvolved {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let evolved = BRepOffsetApiMakeEvolved::new();
        assert!(!evolved.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut evolved = BRepOffsetApiMakeEvolved::new();
        evolved.set_done();
        assert!(evolved.is_done());
    }

    #[test]
    fn test_default() {
        let evolved = BRepOffsetApiMakeEvolved::default();
        assert!(!evolved.is_done());
    }
}
