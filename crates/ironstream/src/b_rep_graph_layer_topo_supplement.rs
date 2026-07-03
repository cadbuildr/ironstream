// FILE: b_rep_graph_layer_topo_supplement.rs
// occt: BRepGraph_LayerTopoSupplement

/// Minimal implementation of BRepGraph_LayerTopoSupplement
pub struct BRepGraphLayerTopoSupplement {}

impl Default for BRepGraphLayerTopoSupplement {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphLayerTopoSupplement::default();
    }
}
