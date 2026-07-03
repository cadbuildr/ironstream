// FILE: top_ope_b_rep_tool_kro.rs
// occt: TopOpeBRepTool_KRO

/// KRO stub implementation.
pub struct KRO;

impl KRO {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        KRO
    }
}

impl Default for KRO {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = KRO::new();
    }

    #[test]
    fn test_default() {
        let _ = KRO::default();
    }
}
