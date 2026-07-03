// FILE: top_ope_b_rep_tool_out_curve_type.rs
// occt: TopOpeBRepTool_OutCurveType

/// OutCurveType stub implementation.
pub struct OutCurveType;

impl OutCurveType {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        OutCurveType
    }
}

impl Default for OutCurveType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = OutCurveType::new();
    }

    #[test]
    fn test_default() {
        let _ = OutCurveType::default();
    }
}
