// FILE: int_patch_poly_arc.rs
// occt: IntPatch_PolyArc

/// Implementation of IntPatch_PolyArc
pub struct IntPatch_PolyArc;

impl IntPatch_PolyArc {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_PolyArc
    }
}

impl Default for IntPatch_PolyArc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_PolyArc::new();
    }
}
