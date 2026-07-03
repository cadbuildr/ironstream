// FILE: int_patch_interference_polyhedron.rs
// occt: IntPatch_InterferencePolyhedron

/// Implementation of IntPatch_InterferencePolyhedron
pub struct IntPatch_InterferencePolyhedron;

impl IntPatch_InterferencePolyhedron {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_InterferencePolyhedron
    }
}

impl Default for IntPatch_InterferencePolyhedron {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_InterferencePolyhedron::new();
    }
}
