// FILE: top_ope_b_rep_tool_p_so_classif.rs
// occt: TopOpeBRepTool_PSoClassif

/// PSoClassif stub implementation.
pub struct PSoClassif;

impl PSoClassif {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        PSoClassif
    }
}

impl Default for PSoClassif {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = PSoClassif::new();
    }

    #[test]
    fn test_default() {
        let _ = PSoClassif::default();
    }
}
