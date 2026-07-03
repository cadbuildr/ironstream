// FILE: b_rep_extrema_overlap_tool.rs
// occt: BRepExtrema_OverlapTool

/// Tool for detecting overlapping elements
pub struct OverlapTool;

impl OverlapTool {
    pub fn new() -> Self {
        OverlapTool
    }
}

impl Default for OverlapTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_tool_creation() {
        let _tool = OverlapTool::new();
    }
}
