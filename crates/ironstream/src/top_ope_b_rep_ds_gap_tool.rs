// FILE: top_ope_b_rep_ds_gap_tool.rs
// occt: TopOpeBRepDS_GapTool

/// Tool for managing gaps in edges
#[derive(Debug, Clone)]
pub struct GapTool {
    /// DS reference
    ds_id: Option<usize>,
}

impl GapTool {
    /// Create new GapTool
    pub fn new() -> Self {
        GapTool { ds_id: None }
    }

    /// Create with data structure
    pub fn with_ds(ds_id: usize) -> Self {
        GapTool {
            ds_id: Some(ds_id),
        }
    }

    /// Get DS reference
    pub fn hds(&self) -> Option<usize> {
        self.ds_id
    }
}

impl Default for GapTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_tool_new() {
        let gt = GapTool::new();
        assert!(gt.hds().is_none());
    }

    #[test]
    fn test_gap_tool_with_ds() {
        let gt = GapTool::with_ds(42);
        assert_eq!(gt.hds(), Some(42));
    }
}
