// FILE: top_ope_b_rep_ff_transition_tool.rs
// occt: TopOpeBRep_FFTransitionTool

/// FFTransitionTool stub implementation.
pub struct FFTransitionTool;

impl FFTransitionTool {
    /// Create new instance.
    pub fn new() -> Self {
        FFTransitionTool
    }
}

impl Default for FFTransitionTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = FFTransitionTool::new();
    }

    #[test]
    fn test_default() {
        let _ = FFTransitionTool::default();
    }
}
