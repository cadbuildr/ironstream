// FILE: b_rep_graph_layer_history.rs
// occt: BRepGraph_LayerHistory

/// Minimal implementation of BRepGraph_LayerHistory
pub struct BRepGraphLayerHistory {}

impl Default for BRepGraphLayerHistory {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphLayerHistory::default();
    }
}
