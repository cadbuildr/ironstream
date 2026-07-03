// FILE: int_patch_w_line.rs
// occt: IntPatch_WLine

/// Implementation of IntPatch_WLine
pub struct IntPatch_WLine;

impl IntPatch_WLine {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_WLine
    }
}

impl Default for IntPatch_WLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_WLine::new();
    }
}
