// FILE: b_rep_graph_tool.rs
// occt: BRepGraph_Tool

/// Minimal implementation of BRepGraph_Tool
pub struct BRepGraphTool {}

impl Default for BRepGraphTool {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphTool::default();
    }
}
