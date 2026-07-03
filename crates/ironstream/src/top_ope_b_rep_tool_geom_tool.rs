// FILE: top_ope_b_rep_tool_geom_tool.rs
// occt: TopOpeBRepTool_GeomTool

/// GeomTool stub implementation.
pub struct GeomTool;

impl GeomTool {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        GeomTool
    }
}

impl Default for GeomTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = GeomTool::new();
    }

    #[test]
    fn test_default() {
        let _ = GeomTool::default();
    }
}
