// FILE: top_ope_b_rep_tool_topology.rs
// occt: TopOpeBRepTool_TOPOLOGY

/// TOPOLOGY stub implementation.
pub struct TOPOLOGY;

impl TOPOLOGY {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        TOPOLOGY
    }
}

impl Default for TOPOLOGY {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = TOPOLOGY::new();
    }

    #[test]
    fn test_default() {
        let _ = TOPOLOGY::default();
    }
}
