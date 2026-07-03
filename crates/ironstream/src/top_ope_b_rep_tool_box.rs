// FILE: top_ope_b_rep_tool_box.rs
// occt: TopOpeBRepTool_box

/// Box utility stub implementation.
pub struct BoxUtil;

impl BoxUtil {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        BoxUtil
    }
}

impl Default for BoxUtil {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BoxUtil::new();
    }

    #[test]
    fn test_default() {
        let _ = BoxUtil::default();
    }
}
