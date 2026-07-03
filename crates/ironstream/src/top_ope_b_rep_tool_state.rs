// FILE: top_ope_b_rep_tool_state.rs
// occt: TopOpeBRepTool_STATE

/// STATE stub implementation.
pub struct STATE;

impl STATE {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        STATE
    }
}

impl Default for STATE {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = STATE::new();
    }

    #[test]
    fn test_default() {
        let _ = STATE::default();
    }
}
