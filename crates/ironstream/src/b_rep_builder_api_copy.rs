// FILE: b_rep_builder_api_copy.rs
// occt: BRepBuilderAPI_Copy

/// Stub for TopoDS_Shape - represents a topological shape
#[derive(Clone, Debug)]
pub struct TopoDsShape {
    data: Vec<u8>,
}

impl TopoDsShape {
    pub fn new() -> Self {
        TopoDsShape { data: Vec::new() }
    }

    pub fn is_null(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for TopoDsShape {
    fn default() -> Self {
        Self::new()
    }
}

/// BRepBuilderAPI_Copy: Duplication of a shape
pub struct BRepBuilderApiCopy {
    shape: TopoDsShape,
    copy_geom: bool,
    copy_mesh: bool,
}

impl BRepBuilderApiCopy {
    pub fn new() -> Self {
        BRepBuilderApiCopy {
            shape: TopoDsShape::new(),
            copy_geom: true,
            copy_mesh: false,
        }
    }

    pub fn with_shape(s: &TopoDsShape, copy_geom: bool, copy_mesh: bool) -> Self {
        BRepBuilderApiCopy {
            shape: s.clone(),
            copy_geom,
            copy_mesh,
        }
    }

    pub fn perform(&mut self, s: &TopoDsShape, copy_geom: bool, copy_mesh: bool) {
        self.shape = s.clone();
        self.copy_geom = copy_geom;
        self.copy_mesh = copy_mesh;
    }

    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }
}

impl Default for BRepBuilderApiCopy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let copy = BRepBuilderApiCopy::new();
        assert!(copy.shape().is_null());
    }

    #[test]
    fn test_with_shape() {
        let s = TopoDsShape::new();
        let copy = BRepBuilderApiCopy::with_shape(&s, true, false);
        assert_eq!(copy.copy_geom, true);
        assert_eq!(copy.copy_mesh, false);
    }

    #[test]
    fn test_perform() {
        let mut copy = BRepBuilderApiCopy::new();
        let s = TopoDsShape::new();
        copy.perform(&s, false, true);
        assert_eq!(copy.copy_geom, false);
        assert_eq!(copy.copy_mesh, true);
    }

    #[test]
    fn test_topology_shape() {
        let shape = TopoDsShape::new();
        assert!(shape.is_null());
        let shape2 = shape.clone();
        assert!(shape2.is_null());
    }
}
