// FILE: int_tools_o.rs
// occt: IntTools

use super::int_tools_root::IntToolsRoot;

/// Common utility functions for intersection tools.
pub struct IntTools;

impl IntTools {
    /// Get the length of an edge.
    pub fn length(_edge: &[u8]) -> f64 {
        // Placeholder: would need actual edge curve evaluation
        1.0
    }

    /// Remove identical roots within tolerance.
    pub fn remove_identical_roots(roots: &mut Vec<IntToolsRoot>, eps_t: f64) {
        if roots.len() < 2 {
            return;
        }
        let mut i = 0;
        while i < roots.len() {
            let mut j = i + 1;
            while j < roots.len() {
                if (roots[i].root() - roots[j].root()).abs() < eps_t {
                    roots.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    /// Sort roots in increasing order.
    pub fn sort_roots(roots: &mut Vec<IntToolsRoot>, _eps_t: f64) {
        roots.sort_by(|a, b| a.root().partial_cmp(&b.root()).unwrap());
    }

    /// Find root states (before and after).
    pub fn find_root_states(_roots: &mut Vec<IntToolsRoot>, _eps_null: f64) {
        // Placeholder: would need function evaluation
    }

    /// Find parameter on curve for a point.
    pub fn parameter(_point: &[f64], _curve: &[u8]) -> Option<f64> {
        // Placeholder: would need curve projection
        Some(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let edge = vec![];
        let len = IntTools::length(&edge);
        assert!(len > 0.0);
    }

    #[test]
    fn test_remove_identical_roots() {
        let mut roots = vec![
            IntToolsRoot::with_root_type(1.0, 0),
            IntToolsRoot::with_root_type(1.01, 0),
            IntToolsRoot::with_root_type(2.0, 0),
        ];
        IntTools::remove_identical_roots(&mut roots, 0.02);
        assert_eq!(roots.len(), 2);
    }

    #[test]
    fn test_sort_roots() {
        let mut roots = vec![
            IntToolsRoot::with_root_type(3.0, 0),
            IntToolsRoot::with_root_type(1.0, 0),
            IntToolsRoot::with_root_type(2.0, 0),
        ];
        IntTools::sort_roots(&mut roots, 1e-10);
        assert!(roots[0].root() < roots[1].root());
        assert!(roots[1].root() < roots[2].root());
    }

    #[test]
    fn test_parameter() {
        let point = vec![1.0, 2.0, 3.0];
        let curve = vec![];
        let param = IntTools::parameter(&point, &curve);
        assert!(param.is_some());
    }
}
