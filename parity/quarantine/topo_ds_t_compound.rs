// FILE: topo_ds_t_compound.rs
// occt: TopoDS_TCompound

//! Topology shape implementation for compound.

/// Internal topology structure for compound
#[derive(Clone)]
pub struct TopoDS_TCompound {
    id: usize,
}

impl TopoDS_TCompound {
    /// Creates new compound topology
    pub fn new(id: usize) -> Self {
        TopoDS_TCompound { id }
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
    fn test_t_compound_new() {
        let tshape = TopoDS_TCompound::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
