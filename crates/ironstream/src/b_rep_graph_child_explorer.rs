// FILE: b_rep_graph_child_explorer.rs
// occt: BRepGraph_ChildExplorer

/// Minimal implementation of BRepGraph_ChildExplorer
pub struct BRepGraphChildExplorer {}

impl Default for BRepGraphChildExplorer {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphChildExplorer::default();
    }
}
