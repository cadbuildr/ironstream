// FILE: topo_ds_to_step_make_edge_error.rs
// occt: TopoDSToStep_MakeEdgeError

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeEdgeError {
    EdgeDone,
    NonManifoldEdge,
    EdgeOther,
}

impl MakeEdgeError {
    pub fn is_success(&self) -> bool {
        matches!(self, MakeEdgeError::EdgeDone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_success() {
        assert!(MakeEdgeError::EdgeDone.is_success());
        assert!(!MakeEdgeError::NonManifoldEdge.is_success());
    }
}
