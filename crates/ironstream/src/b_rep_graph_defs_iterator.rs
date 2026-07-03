// FILE: b_rep_graph_defs_iterator.rs
// occt: BRepGraph_DefsIterator

/// Minimal implementation of BRepGraph_DefsIterator
pub struct BRepGraphDefsIterator {}

impl Default for BRepGraphDefsIterator {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphDefsIterator::default();
    }
}
