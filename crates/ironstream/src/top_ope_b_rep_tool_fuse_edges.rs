// FILE: top_ope_b_rep_tool_fuse_edges.rs
// occt: TopOpeBRepTool_FuseEdges

/// FuseEdges stub implementation.
pub struct FuseEdges;

impl FuseEdges {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        FuseEdges
    }
}

impl Default for FuseEdges {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = FuseEdges::new();
    }

    #[test]
    fn test_default() {
        let _ = FuseEdges::default();
    }
}
