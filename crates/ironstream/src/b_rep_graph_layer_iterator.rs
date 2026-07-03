// FILE: b_rep_graph_layer_iterator.rs
// occt: BRepGraph_LayerIterator

/// Minimal implementation of BRepGraph_LayerIterator
pub struct BRepGraphLayerIterator {}

impl Default for BRepGraphLayerIterator {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphLayerIterator::default();
    }
}
