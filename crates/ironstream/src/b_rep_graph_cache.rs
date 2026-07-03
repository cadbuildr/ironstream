// FILE: b_rep_graph_cache.rs
// occt: BRepGraph_Cache

/// Minimal implementation of BRepGraph_Cache
pub struct BRepGraphCache {}

impl Default for BRepGraphCache {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphCache::default();
    }
}
