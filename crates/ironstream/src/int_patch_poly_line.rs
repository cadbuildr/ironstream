// FILE: int_patch_poly_line.rs
// occt: IntPatch_PolyLine

/// Implementation of IntPatch_PolyLine
pub struct IntPatch_PolyLine;

impl IntPatch_PolyLine {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_PolyLine
    }
}

impl Default for IntPatch_PolyLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_PolyLine::new();
    }
}
