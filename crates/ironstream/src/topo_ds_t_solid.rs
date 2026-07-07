// FILE: topo_ds_t_solid.rs
// occt: TopoDS_TSolid

//! Topology shape implementation for solid.

/// Internal topology structure for solid
#[derive(Clone)]
pub struct TopoDS_TSolid {
    id: usize,
}

impl TopoDS_TSolid {
    /// Creates new solid topology
    pub fn new(id: usize) -> Self {
        TopoDS_TSolid { id }
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
    fn test_t_solid_new() {
        let tshape = TopoDS_TSolid::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
