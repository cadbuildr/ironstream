// FILE: top_ope_b_rep_tool_regus.rs
// occt: TopOpeBRepTool_REGUS

/// REGUS stub implementation.
pub struct REGUS;

impl REGUS {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        REGUS
    }
}

impl Default for REGUS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = REGUS::new();
    }

    #[test]
    fn test_default() {
        let _ = REGUS::default();
    }
}
