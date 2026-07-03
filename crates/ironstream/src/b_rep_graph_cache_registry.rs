// FILE: b_rep_graph_cache_registry.rs
// occt: BRepGraph_CacheRegistry

/// Minimal implementation of BRepGraph_CacheRegistry
pub struct BRepGraphCacheRegistry {}

impl Default for BRepGraphCacheRegistry {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphCacheRegistry::default();
    }
}
