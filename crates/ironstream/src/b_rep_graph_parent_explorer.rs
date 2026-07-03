// FILE: b_rep_graph_parent_explorer.rs
// occt: BRepGraph_ParentExplorer

/// Minimal implementation of BRepGraph_ParentExplorer
pub struct BRepGraphParentExplorer {}

impl Default for BRepGraphParentExplorer {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphParentExplorer::default();
    }
}
