// FILE: b_rep_graph_deduplicate.rs
// occt: BRepGraph_Deduplicate

/// Minimal implementation of BRepGraph_Deduplicate
pub struct BRepGraphDeduplicate {}

impl Default for BRepGraphDeduplicate {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphDeduplicate::default();
    }
}
