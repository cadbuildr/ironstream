// FILE: int_patch_point_line.rs
// occt: IntPatch_PointLine

/// Implementation of IntPatch_PointLine
pub struct IntPatch_PointLine;

impl IntPatch_PointLine {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_PointLine
    }
}

impl Default for IntPatch_PointLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_PointLine::new();
    }
}
