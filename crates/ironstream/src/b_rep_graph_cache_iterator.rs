// FILE: b_rep_graph_cache_iterator.rs
// occt: BRepGraph_CacheIterator

/// Minimal implementation of BRepGraph_CacheIterator
pub struct BRepGraphCacheIterator {}

impl Default for BRepGraphCacheIterator {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphCacheIterator::default();
    }
}
