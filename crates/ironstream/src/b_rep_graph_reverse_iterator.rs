// FILE: b_rep_graph_reverse_iterator.rs
// occt: BRepGraph_ReverseIterator

/// Minimal implementation of BRepGraph_ReverseIterator
pub struct BRepGraphReverseIterator {}

impl Default for BRepGraphReverseIterator {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphReverseIterator::default();
    }
}
