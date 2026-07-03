// FILE: b_rep_graph_cache_derived_state.rs
// occt: BRepGraph_CacheDerivedState

/// Minimal implementation of BRepGraph_CacheDerivedState
pub struct BRepGraphCacheDerivedState {}

impl Default for BRepGraphCacheDerivedState {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphCacheDerivedState::default();
    }
}
