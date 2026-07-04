// FILE: topo_ds_t_face.rs
// occt: TopoDS_TFace

//! Topology shape implementation for face.

/// Internal topology structure for face
#[derive(Clone)]
pub struct TopoDS_TFace {
    id: usize,
}

impl TopoDS_TFace {
    /// Creates new face topology
    pub fn new(id: usize) -> Self {
        TopoDS_TFace { id }
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
    fn test_t_face_new() {
        let tshape = TopoDS_TFace::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
