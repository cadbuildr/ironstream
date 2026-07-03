// FILE: top_ope_b_rep_edges_intersector.rs
// occt: TopOpeBRep_EdgesIntersector

/// EdgesIntersector stub implementation.
pub struct EdgesIntersector;

impl EdgesIntersector {
    /// Create new instance.
    pub fn new() -> Self {
        EdgesIntersector
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
        let _ = EdgesIntersector::new();
    }

    #[test]
    fn test_default() {
        let _ = EdgesIntersector::default();
    }
}
