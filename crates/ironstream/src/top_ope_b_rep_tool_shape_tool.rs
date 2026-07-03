// FILE: top_ope_b_rep_tool_shape_tool.rs
// occt: TopOpeBRepTool_ShapeTool

/// ShapeTool stub implementation.
pub struct ShapeTool;

impl ShapeTool {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        ShapeTool
    }
}

impl Default for ShapeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ShapeTool::new();
    }

    #[test]
    fn test_default() {
        let _ = ShapeTool::default();
    }
}
