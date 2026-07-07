// FILE: topo_ds_to_step_make_vertex_error.rs
// occt: TopoDSToStep_MakeVertexError

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeVertexError {
    VertexDone,
    VertexOther,
}

impl MakeVertexError {
    pub fn is_success(&self) -> bool {
        matches!(self, MakeVertexError::VertexDone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_success() {
        assert!(MakeVertexError::VertexDone.is_success());
        assert!(!MakeVertexError::VertexOther.is_success());
    }
}
