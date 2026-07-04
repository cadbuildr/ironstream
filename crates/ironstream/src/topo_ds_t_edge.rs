// FILE: topo_ds_t_edge.rs
// occt: TopoDS_TEdge

//! Topology shape implementation for edge.

/// Internal topology structure for edge
#[derive(Clone)]
pub struct TopoDS_TEdge {
    id: usize,
}

impl TopoDS_TEdge {
    /// Creates new edge topology
    pub fn new(id: usize) -> Self {
        TopoDS_TEdge { id }
    }

    /// Returns topology ID
    pub fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_edge_new() {
        let tshape = TopoDS_TEdge::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
