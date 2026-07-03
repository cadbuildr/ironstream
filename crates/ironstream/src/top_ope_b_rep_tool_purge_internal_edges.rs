// FILE: top_ope_b_rep_tool_purge_internal_edges.rs
// occt: TopOpeBRepTool_PurgeInternalEdges

/// PurgeInternalEdges stub implementation.
pub struct PurgeInternalEdges;

impl PurgeInternalEdges {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        PurgeInternalEdges
    }
}

impl Default for PurgeInternalEdges {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = PurgeInternalEdges::new();
    }

    #[test]
    fn test_default() {
        let _ = PurgeInternalEdges::default();
    }
}
