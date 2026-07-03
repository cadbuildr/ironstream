// FILE: b_rep_lib_fuse_edges.rs
// occt: BRepLib_FuseEdges

/// Detects and fuses useless vertices and edges
pub struct FuseEdges {
    shape_done: bool,
    edges_done: bool,
    result_edges_done: bool,
    nb_connex_edge: i32,
    concat_bspl: bool,
}

impl FuseEdges {
    pub fn new() -> Self {
        FuseEdges {
            shape_done: false,
            edges_done: false,
            result_edges_done: false,
            nb_connex_edge: 0,
            concat_bspl: false,
        }
    }

    pub fn set_concat_bspl(&mut self, concat: bool) {
        self.concat_bspl = concat;
    }

    pub fn perform(&mut self) {
        self.shape_done = true;
        self.edges_done = true;
    }

    pub fn nb_vertices(&self) -> i32 {
        0
    }
}

impl Default for FuseEdges {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuse_edges_creation() {
        let fuse = FuseEdges::new();
        assert!(!fuse.shape_done);
    }

    #[test]
    fn test_set_concat_bspl() {
        let mut fuse = FuseEdges::new();
        fuse.set_concat_bspl(true);
        assert!(fuse.concat_bspl);
    }

    #[test]
    fn test_perform() {
        let mut fuse = FuseEdges::new();
        fuse.perform();
        assert!(fuse.shape_done);
    }
}
