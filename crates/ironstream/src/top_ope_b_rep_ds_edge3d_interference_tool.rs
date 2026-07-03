// FILE: top_ope_b_rep_ds_edge3d_interference_tool.rs
// occt: TopOpeBRepDS_Edge3dInterferenceTool

/// Tool for 3D edge interferences
#[derive(Debug, Clone)]
pub struct Edge3dInterferenceTool {
    /// DS reference
    ds_id: Option<usize>,
}

impl Edge3dInterferenceTool {
    /// Create new Edge3dInterferenceTool
    pub fn new() -> Self {
        Edge3dInterferenceTool { ds_id: None }
    }

    /// Create with data structure
    pub fn with_ds(ds_id: usize) -> Self {
        Edge3dInterferenceTool {
            ds_id: Some(ds_id),
        }
    }

    /// Get DS reference
    pub fn hds(&self) -> Option<usize> {
        self.ds_id
    }
}

impl Default for Edge3dInterferenceTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge3d_interference_tool_new() {
        let e3dit = Edge3dInterferenceTool::new();
        assert!(e3dit.hds().is_none());
    }

    #[test]
    fn test_edge3d_interference_tool_with_ds() {
        let e3dit = Edge3dInterferenceTool::with_ds(42);
        assert_eq!(e3dit.hds(), Some(42));
    }
}
