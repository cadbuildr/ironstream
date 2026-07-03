// FILE: b_rep_builder_api_make_shape_on_mesh.rs
// occt: BRepBuilderAPI_MakeShapeOnMesh

pub struct BrepbuilderapiMakeshapeonmesh;

impl BrepbuilderapiMakeshapeonmesh {
    pub fn new() -> Self {
        BrepbuilderapiMakeshapeonmesh
    }
}

impl Default for BrepbuilderapiMakeshapeonmesh {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiMakeshapeonmesh::new();
    }
}
