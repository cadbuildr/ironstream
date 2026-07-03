// FILE: top_ope_b_rep_tool_mk_tondg_e.rs
// occt: TopOpeBRepTool_mkTondgE

#[allow(dead_code)]
/// mkTondgE stub implementation.
pub struct mkTondgE;

impl mkTondgE {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        mkTondgE
    }
}

impl Default for mkTondgE {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = mkTondgE::new();
    }

    #[test]
    fn test_default() {
        let _ = mkTondgE::default();
    }
}
