// FILE: b_rep_graph_cache_mesh.rs
// occt: BRepGraph_CacheMesh

/// Minimal implementation of BRepGraph_CacheMesh
pub struct BRepGraphCacheMesh {}

impl Default for BRepGraphCacheMesh {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphCacheMesh::default();
    }
}
