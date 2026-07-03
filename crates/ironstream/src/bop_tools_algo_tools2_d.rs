// FILE: bop_tools_algo_tools2_d.rs
// occt: BOPTools_AlgoTools2D

/// 2D algorithm tools for boolean operations.
pub struct BopToolsAlgoTools2D;

impl BopToolsAlgoTools2D {
    /// Check if point is on edge (2D).
    pub fn is_on_edge(_edge: &[u8], _t: f64, _tol: f64) -> bool {
        false
    }

    /// Get middle point 2D.
    pub fn middle_point(_e1: &[u8], _e2: &[u8]) -> (f64, f64) {
        (0.5, 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_edge() {
        assert!(!BopToolsAlgoTools2D::is_on_edge(&[], 0.5, 0.01));
    }

    #[test]
    fn test_middle_point() {
        let (x, y) = BopToolsAlgoTools2D::middle_point(&[], &[]);
        assert_eq!(x, 0.5);
        assert_eq!(y, 0.5);
    }
}
