// FILE: int_patch_polyhedron_tool.rs
// occt: IntPatch_PolyhedronTool

/// Implementation of IntPatch_PolyhedronTool
pub struct IntPatch_PolyhedronTool;

impl IntPatch_PolyhedronTool {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_PolyhedronTool
    }
}

impl Default for IntPatch_PolyhedronTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_PolyhedronTool::new();
    }
}
