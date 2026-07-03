// FILE: b_rep_graph_refs_iterator.rs
// occt: BRepGraph_RefsIterator

/// Minimal implementation of BRepGraph_RefsIterator
pub struct BRepGraphRefsIterator {}

impl Default for BRepGraphRefsIterator {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphRefsIterator::default();
    }
}
