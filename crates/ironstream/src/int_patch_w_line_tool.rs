// FILE: int_patch_w_line_tool.rs
// occt: IntPatch_WLineTool

/// Implementation of IntPatch_WLineTool
pub struct IntPatch_WLineTool;

impl IntPatch_WLineTool {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_WLineTool
    }
}

impl Default for IntPatch_WLineTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_WLineTool::new();
    }
}
