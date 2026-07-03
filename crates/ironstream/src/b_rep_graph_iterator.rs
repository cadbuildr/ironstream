// FILE: b_rep_graph_iterator.rs
// occt: BRepGraph_Iterator

/// Minimal implementation of BRepGraph_Iterator
pub struct BRepGraphIterator {}

impl Default for BRepGraphIterator {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIterator::default();
    }
}
