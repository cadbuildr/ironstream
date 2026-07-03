// FILE: top_ope_b_rep_tool_reguw.rs
// occt: TopOpeBRepTool_REGUW

/// REGUW stub implementation.
pub struct REGUW;

impl REGUW {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        REGUW
    }
}

impl Default for REGUW {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = REGUW::new();
    }

    #[test]
    fn test_default() {
        let _ = REGUW::default();
    }
}
