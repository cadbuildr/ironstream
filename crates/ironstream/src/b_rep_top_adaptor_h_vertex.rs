// FILE: b_rep_top_adaptor_h_vertex.rs
// occt: BRepTopAdaptor_HVertex

/// Handle to a topological vertex adapter.
#[derive(Debug, Clone)]
pub struct BRepTopAdaptorHVertex;

impl BRepTopAdaptorHVertex {
    pub fn new() -> Self {
        BRepTopAdaptorHVertex
    }
}

impl Default for BRepTopAdaptorHVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_vertex() {
        let _hv = BRepTopAdaptorHVertex::new();
    }
}
