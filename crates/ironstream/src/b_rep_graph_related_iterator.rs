// FILE: b_rep_graph_related_iterator.rs
// occt: BRepGraph_RelatedIterator

/// Minimal implementation of BRepGraph_RelatedIterator
pub struct BRepGraphRelatedIterator {}

impl Default for BRepGraphRelatedIterator {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphRelatedIterator::default();
    }
}
