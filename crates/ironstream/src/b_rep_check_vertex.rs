// FILE: b_rep_check_vertex.rs
// occt: BRepCheck_Vertex

/// Check vertex result class
pub struct VertexChecker;

impl VertexChecker {
    pub fn new() -> Self {
        VertexChecker
    }

    pub fn tolerance() -> f64 {
        1e-7
    }
}

impl Default for VertexChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_checker_creation() {
        let _checker = VertexChecker::new();
    }

    #[test]
    fn test_tolerance() {
        assert!(VertexChecker::tolerance() > 0.0);
    }
}
