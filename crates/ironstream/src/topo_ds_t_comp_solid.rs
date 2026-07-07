// FILE: topo_ds_t_comp_solid.rs
// occt: TopoDS_TCompSolid

//! Topology shape implementation for compound solid.

/// Internal topology structure for compound solid
#[derive(Clone)]
pub struct TopoDS_TCompSolid {
    id: usize,
}

impl TopoDS_TCompSolid {
    /// Creates new compound solid topology
    pub fn new(id: usize) -> Self {
        TopoDS_TCompSolid { id }
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
    fn test_t_comp_solid_new() {
        let tshape = TopoDS_TCompSolid::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
