// FILE: top_ope_b_rep_tool_shape_explorer.rs
// occt: TopOpeBRepTool_ShapeExplorer

/// ShapeExplorer stub implementation.
pub struct ShapeExplorer;

impl ShapeExplorer {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        ShapeExplorer
    }
}

impl Default for ShapeExplorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ShapeExplorer::new();
    }

    #[test]
    fn test_default() {
        let _ = ShapeExplorer::default();
    }
}
