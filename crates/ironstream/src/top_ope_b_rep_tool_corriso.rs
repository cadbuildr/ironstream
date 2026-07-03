// FILE: top_ope_b_rep_tool_corriso.rs
// occt: TopOpeBRepTool_CORRISO

/// CORRISO stub implementation.
pub struct CORRISO;

impl CORRISO {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        CORRISO
    }
}

impl Default for CORRISO {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = CORRISO::new();
    }

    #[test]
    fn test_default() {
        let _ = CORRISO::default();
    }
}
