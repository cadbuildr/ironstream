// FILE: top_ope_b_rep_build_fuse_face.rs
// occt: TopOpeBRepBuild_FuseFace

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildFuseFace {
    internal_faces: Vec<i32>,
    external_faces: Vec<i32>,
    fused_faces: Vec<i32>,
    internal_edges: Vec<i32>,
    external_edges: Vec<i32>,
    modified_edges: Vec<i32>,
    is_done: bool,
    is_modified: bool,
}

impl TopOpeBRepBuildFuseFace {
    pub fn new() -> Self {
        TopOpeBRepBuildFuseFace {
            internal_faces: Vec::new(),
            external_faces: Vec::new(),
            fused_faces: Vec::new(),
            internal_edges: Vec::new(),
            external_edges: Vec::new(),
            modified_edges: Vec::new(),
            is_done: false,
            is_modified: false,
        }
    }

    pub fn with_faces(internal: Vec<i32>, external: Vec<i32>) -> Self {
        TopOpeBRepBuildFuseFace {
            internal_faces: internal,
            external_faces: external,
            fused_faces: Vec::new(),
            internal_edges: Vec::new(),
            external_edges: Vec::new(),
            modified_edges: Vec::new(),
            is_done: false,
            is_modified: false,
        }
    }

    pub fn perform_face(&mut self) {
        self.is_done = true;
    }

    pub fn perform_edge(&mut self) {
        self.is_modified = true;
    }

    pub fn clear_edge(&mut self) {
        self.internal_edges.clear();
        self.external_edges.clear();
        self.modified_edges.clear();
    }

    pub fn clear_vertex(&mut self) {}

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    pub fn fused_faces(&self) -> &[i32] {
        &self.fused_faces
    }

    pub fn internal_edges(&self) -> &[i32] {
        &self.internal_edges
    }

    pub fn external_edges(&self) -> &[i32] {
        &self.external_edges
    }

    pub fn modified_edges(&self) -> &[i32] {
        &self.modified_edges
    }
}

impl Default for TopOpeBRepBuildFuseFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuse_face_new() {
        let ff = TopOpeBRepBuildFuseFace::new();
        assert!(!ff.is_done());
        assert!(!ff.is_modified());
    }

    #[test]
    fn test_fuse_face_with_faces() {
        let internal = vec![1, 2];
        let external = vec![3, 4];
        let ff = TopOpeBRepBuildFuseFace::with_faces(internal, external);
        assert!(!ff.is_done());
    }

    #[test]
    fn test_fuse_face_perform() {
        let mut ff = TopOpeBRepBuildFuseFace::new();
        assert!(!ff.is_done());
        ff.perform_face();
        assert!(ff.is_done());
    }

    #[test]
    fn test_fuse_face_perform_edge() {
        let mut ff = TopOpeBRepBuildFuseFace::new();
        assert!(!ff.is_modified());
        ff.perform_edge();
        assert!(ff.is_modified());
    }

    #[test]
    fn test_fuse_face_clear_edge() {
        let mut ff = TopOpeBRepBuildFuseFace::new();
        ff.clear_edge();
        assert!(ff.internal_edges().is_empty());
    }

    #[test]
    fn test_fuse_face_default() {
        let ff = TopOpeBRepBuildFuseFace::default();
        assert!(!ff.is_done());
    }
}
