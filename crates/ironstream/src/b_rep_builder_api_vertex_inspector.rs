// FILE: b_rep_builder_api_vertex_inspector.rs
// occt: BRepBuilderAPI_VertexInspector

use std::collections::VecDeque;

/// Inspector for cell filter algorithm working with 3D XYZ points.
/// Used in search of coincident points with a certain tolerance.
///
/// Mirrors OCCT's BRepBuilderAPI_VertexInspector.
pub struct BRepBuilderAPIVertexInspector {
    /// Squared tolerance for distance comparisons
    tol_squared: f64,
    /// Result indices of adjacent points
    res_ind: VecDeque<i32>,
    /// Points used for comparison
    points: Vec<[f64; 3]>,
    /// Current point to search for coincidence
    current: [f64; 3],
}

impl BRepBuilderAPIVertexInspector {
    /// Constructor; remembers the tolerance
    pub fn new(tolerance: f64) -> Self {
        BRepBuilderAPIVertexInspector {
            tol_squared: tolerance * tolerance,
            res_ind: VecDeque::new(),
            points: Vec::new(),
            current: [0.0; 3],
        }
    }

    /// Keep the points used for comparison
    pub fn add(&mut self, point: [f64; 3]) {
        self.points.push(point);
    }

    /// Clear the list of adjacent point indices
    pub fn clear_result_indices(&mut self) {
        self.res_ind.clear();
    }

    /// Set current point to search for coincidence
    pub fn set_current(&mut self, point: [f64; 3]) {
        self.current = point;
    }

    /// Get list of indices of points adjacent with the current point
    pub fn result_indices(&self) -> Vec<i32> {
        self.res_ind.iter().copied().collect()
    }

    /// Implementation of inspection method.
    /// Returns true if the point at the given index is within tolerance of current point.
    pub fn inspect(&mut self, target: usize) -> bool {
        if target >= self.points.len() {
            return false;
        }

        let p = self.points[target];
        let dx = self.current[0] - p[0];
        let dy = self.current[1] - p[1];
        let dz = self.current[2] - p[2];

        let dist_squared = dx * dx + dy * dy + dz * dz;

        if dist_squared <= self.tol_squared {
            self.res_ind.push_back(target as i32);
            true
        } else {
            false
        }
    }

    /// Static method to get coordinate from a point
    pub const fn dimension() -> usize {
        3
    }

    /// Get coordinate at index (0=x, 1=y, 2=z)
    pub fn coord(index: usize, point: &[f64; 3]) -> f64 {
        if index < 3 {
            point[index]
        } else {
            0.0
        }
    }

    /// Shift point by tolerance
    pub fn shift(point: &[f64; 3], tolerance: f64) -> [f64; 3] {
        [
            point[0] + tolerance,
            point[1] + tolerance,
            point[2] + tolerance,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let inspector = BRepBuilderAPIVertexInspector::new(0.1);
        // OCCT stores myTol = theTol * theTol (BRepBuilderAPI_VertexInspector.hxx);
        // 0.1 * 0.1 is not bit-exactly 0.01 in IEEE 754, so compare against the
        // exact value OCCT semantics produce.
        assert_eq!(inspector.tol_squared, 0.1 * 0.1);
    }

    #[test]
    fn test_add_and_set_current() {
        let mut inspector = BRepBuilderAPIVertexInspector::new(0.1);
        inspector.add([1.0, 2.0, 3.0]);
        inspector.add([1.05, 2.05, 3.05]);
        inspector.set_current([1.0, 2.0, 3.0]);
        assert_eq!(inspector.points.len(), 2);
    }

    #[test]
    fn test_inspect_within_tolerance() {
        let mut inspector = BRepBuilderAPIVertexInspector::new(0.1);
        inspector.add([1.0, 2.0, 3.0]);
        inspector.set_current([1.05, 2.05, 3.05]);
        // Distance squared = 0.05^2 + 0.05^2 + 0.05^2 = 0.0075 < 0.01
        let found = inspector.inspect(0);
        assert!(found);
        assert_eq!(inspector.result_indices().len(), 1);
        assert_eq!(inspector.result_indices()[0], 0);
    }

    #[test]
    fn test_inspect_outside_tolerance() {
        let mut inspector = BRepBuilderAPIVertexInspector::new(0.1);
        inspector.add([1.0, 2.0, 3.0]);
        inspector.set_current([1.2, 2.0, 3.0]);
        // Distance squared = 0.2^2 = 0.04 > 0.01
        let found = inspector.inspect(0);
        assert!(!found);
        assert_eq!(inspector.result_indices().len(), 0);
    }

    #[test]
    fn test_clear_result_indices() {
        let mut inspector = BRepBuilderAPIVertexInspector::new(0.1);
        inspector.add([1.0, 2.0, 3.0]);
        inspector.set_current([1.0, 2.0, 3.0]);
        inspector.inspect(0);
        assert_eq!(inspector.result_indices().len(), 1);
        inspector.clear_result_indices();
        assert_eq!(inspector.result_indices().len(), 0);
    }

    #[test]
    fn test_shift() {
        let point = [1.0, 2.0, 3.0];
        let shifted = BRepBuilderAPIVertexInspector::shift(&point, 0.1);
        assert_eq!(shifted, [1.1, 2.1, 3.1]);
    }

    #[test]
    fn test_coord() {
        let point = [1.0, 2.0, 3.0];
        assert_eq!(BRepBuilderAPIVertexInspector::coord(0, &point), 1.0);
        assert_eq!(BRepBuilderAPIVertexInspector::coord(1, &point), 2.0);
        assert_eq!(BRepBuilderAPIVertexInspector::coord(2, &point), 3.0);
        assert_eq!(BRepBuilderAPIVertexInspector::coord(3, &point), 0.0);
    }
}
