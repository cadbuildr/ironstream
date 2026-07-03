// FILE: int_patch_g_line.rs
// occt: IntPatch_GLine

/// Implementation of IntPatch_GLine
pub struct IntPatch_GLine;

impl IntPatch_GLine {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_GLine
    }
}

impl Default for IntPatch_GLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_GLine::new();
    }
}
