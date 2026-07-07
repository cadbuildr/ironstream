// FILE: topo_ds_t_vertex.rs
// occt: TopoDS_TVertex

//! Topology shape implementation for vertex.

/// Internal topology structure for vertex
#[derive(Clone)]
pub struct TopoDS_TVertex {
    id: usize,
}

impl TopoDS_TVertex {
    /// Creates new vertex topology
    pub fn new(id: usize) -> Self {
        TopoDS_TVertex { id }
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
    fn test_t_vertex_new() {
        let tshape = TopoDS_TVertex::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
