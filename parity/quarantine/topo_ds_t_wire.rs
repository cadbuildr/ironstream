// FILE: topo_ds_t_wire.rs
// occt: TopoDS_TWire

//! Topology shape implementation for wire.

/// Internal topology structure for wire
#[derive(Clone)]
pub struct TopoDS_TWire {
    id: usize,
}

impl TopoDS_TWire {
    /// Creates new wire topology
    pub fn new(id: usize) -> Self {
        TopoDS_TWire { id }
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
    fn test_t_wire_new() {
        let tshape = TopoDS_TWire::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
