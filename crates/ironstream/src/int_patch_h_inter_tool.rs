// FILE: int_patch_h_inter_tool.rs
// occt: IntPatch_HInterTool

/// Implementation of IntPatch_HInterTool
pub struct IntPatch_HInterTool;

impl IntPatch_HInterTool {
    /// Creates a new instance.
    pub fn new() -> Self {
        IntPatch_HInterTool
    }
}

impl Default for IntPatch_HInterTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IntPatch_HInterTool::new();
    }
}
