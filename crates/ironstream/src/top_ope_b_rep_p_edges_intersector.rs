// FILE: top_ope_b_rep_p_edges_intersector.rs
// occt: TopOpeBRep_PEdgesIntersector

/// Pointer type alias for EdgesIntersector.
pub type PEdgesIntersector = Option<Box<EdgesIntersector>>;

/// Edges intersector structure.
pub struct EdgesIntersector {
    data: Vec<u8>,
}

impl EdgesIntersector {
    /// Create a new EdgesIntersector.
    pub fn new() -> Self {
        EdgesIntersector { data: Vec::new() }
    }
}

impl Default for EdgesIntersector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let intersector = EdgesIntersector::new();
        assert_eq!(intersector.data.len(), 0);
    }

    #[test]
    fn test_pointer_type() {
        let p: PEdgesIntersector = Some(Box::new(EdgesIntersector::new()));
        assert!(p.is_some());
    }

    #[test]
    fn test_null_pointer() {
        let p: PEdgesIntersector = None;
        assert!(p.is_none());
    }
}
