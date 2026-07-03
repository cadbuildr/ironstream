// FILE: top_ope_b_rep_ds_interference_tool.rs
// occt: TopOpeBRepDS_InterferenceTool

/// Tool for managing interferences
#[derive(Debug, Clone)]
pub struct InterferenceTool {
    /// DS reference
    ds_id: Option<usize>,
}

impl InterferenceTool {
    /// Create new InterferenceTool
    pub fn new() -> Self {
        InterferenceTool { ds_id: None }
    }

    /// Create with data structure
    pub fn with_ds(ds_id: usize) -> Self {
        InterferenceTool {
            ds_id: Some(ds_id),
        }
    }

    /// Get DS reference
    pub fn hds(&self) -> Option<usize> {
        self.ds_id
    }
}

impl Default for InterferenceTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_tool_new() {
        let it = InterferenceTool::new();
        assert!(it.hds().is_none());
    }

    #[test]
    fn test_interference_tool_with_ds() {
        let it = InterferenceTool::with_ds(42);
        assert_eq!(it.hds(), Some(42));
    }
}
