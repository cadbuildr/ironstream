// FILE: b_rep_graph_compact.rs
// occt: BRepGraph_Compact

/// Minimal implementation of BRepGraph_Compact
pub struct BRepGraphCompact {}

impl Default for BRepGraphCompact {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphCompact::default();
    }
}
