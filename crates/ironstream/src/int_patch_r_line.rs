// FILE: int_patch_r_line.rs
// occt: IntPatch_RLine

/// Implementation of IntPatch_RLine
pub struct IntPatch_RLine;

impl IntPatch_RLine {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_RLine
    }
}

impl Default for IntPatch_RLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_RLine::new();
    }
}
