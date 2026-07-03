// FILE: int_patch_h_curve2d_tool.rs
// occt: IntPatch_HCurve2dTool

/// Implementation of IntPatch_HCurve2dTool
pub struct IntPatch_HCurve2dTool;

impl IntPatch_HCurve2dTool {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_HCurve2dTool
    }
}

impl Default for IntPatch_HCurve2dTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_HCurve2dTool::new();
    }
}
