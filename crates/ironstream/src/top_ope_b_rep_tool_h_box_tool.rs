// FILE: top_ope_b_rep_tool_h_box_tool.rs
// occt: TopOpeBRepTool_HBoxTool

/// HBoxTool stub implementation.
pub struct HBoxTool;

impl HBoxTool {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        HBoxTool
    }
}

impl Default for HBoxTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = HBoxTool::new();
    }

    #[test]
    fn test_default() {
        let _ = HBoxTool::default();
    }
}
