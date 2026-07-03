// FILE: top_ope_b_rep_tool_tool.rs
// occt: TopOpeBRepTool_TOOL

/// TOOL stub implementation.
pub struct TOOL;

impl TOOL {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        TOOL
    }
}

impl Default for TOOL {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = TOOL::new();
    }

    #[test]
    fn test_default() {
        let _ = TOOL::default();
    }
}
