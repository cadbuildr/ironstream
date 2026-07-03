// FILE: top_ope_b_rep_tool_curve_tool.rs
// occt: TopOpeBRepTool_CurveTool

/// CurveTool stub implementation.
pub struct CurveTool;

impl CurveTool {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        CurveTool
    }
}

impl Default for CurveTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = CurveTool::new();
    }

    #[test]
    fn test_default() {
        let _ = CurveTool::default();
    }
}
