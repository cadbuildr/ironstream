// FILE: top_ope_b_rep_tool_purge.rs
// occt: TopOpeBRepTool_PURGE

/// PURGE stub implementation.
pub struct PURGE;

impl PURGE {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        PURGE
    }
}

impl Default for PURGE {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = PURGE::new();
    }

    #[test]
    fn test_default() {
        let _ = PURGE::default();
    }
}
