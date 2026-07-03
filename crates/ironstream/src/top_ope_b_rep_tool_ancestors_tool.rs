// FILE: top_ope_b_rep_tool_ancestors_tool.rs
// occt: TopOpeBRepTool_AncestorsTool

/// AncestorsTool stub implementation.
pub struct AncestorsTool;

impl AncestorsTool {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        AncestorsTool
    }
}

impl Default for AncestorsTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = AncestorsTool::new();
    }

    #[test]
    fn test_default() {
        let _ = AncestorsTool::default();
    }
}
