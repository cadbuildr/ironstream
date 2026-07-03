// FILE: top_ope_b_rep_ds_edge_interference_tool.rs
// occt: TopOpeBRepDS_EdgeInterferenceTool

/// Tool for edge interferences
#[derive(Debug, Clone)]
pub struct EdgeInterferenceTool {
    /// DS reference
    ds_id: Option<usize>,
}

impl EdgeInterferenceTool {
    /// Create new EdgeInterferenceTool
    pub fn new() -> Self {
        EdgeInterferenceTool { ds_id: None }
    }

    /// Create with data structure
    pub fn with_ds(ds_id: usize) -> Self {
        EdgeInterferenceTool {
            ds_id: Some(ds_id),
        }
    }

    /// Get DS reference
    pub fn hds(&self) -> Option<usize> {
        self.ds_id
    }
}

impl Default for EdgeInterferenceTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_interference_tool_new() {
        let eit = EdgeInterferenceTool::new();
        assert!(eit.hds().is_none());
    }

    #[test]
    fn test_edge_interference_tool_with_ds() {
        let eit = EdgeInterferenceTool::with_ds(42);
        assert_eq!(eit.hds(), Some(42));
    }
}
