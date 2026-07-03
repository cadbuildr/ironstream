// FILE: top_ope_b_rep_tool_sc.rs
// occt: TopOpeBRepTool_SC

/// SC stub implementation.
pub struct SC;

impl SC {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        SC
    }
}

impl Default for SC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = SC::new();
    }

    #[test]
    fn test_default() {
        let _ = SC::default();
    }
}
