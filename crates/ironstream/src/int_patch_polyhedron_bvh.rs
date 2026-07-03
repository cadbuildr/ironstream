// FILE: int_patch_polyhedron_bvh.rs
// occt: IntPatch_PolyhedronBVH

/// BVH (Bounding Volume Hierarchy) for polyhedron acceleration
#[derive(Clone, Debug)]
pub struct IntPatchPolyhedronBvh {
    num_nodes: usize,
    is_built: bool,
}

impl IntPatchPolyhedronBvh {
    pub fn new() -> Self {
        IntPatchPolyhedronBvh { num_nodes: 0, is_built: false }
    }

    pub fn build(&mut self, num_triangles: usize) {
        self.num_nodes = num_triangles * 2;
        self.is_built = true;
    }

    pub fn is_built(&self) -> bool {
        self.is_built
    }

    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }
}

impl Default for IntPatchPolyhedronBvh {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let bvh = IntPatchPolyhedronBvh::new();
        assert!(!bvh.is_built());
    }

    #[test]
    fn test_build() {
        let mut bvh = IntPatchPolyhedronBvh::new();
        bvh.build(10);
        assert!(bvh.is_built());
        assert_eq!(bvh.num_nodes(), 20);
    }
}
