// FILE: contap_the_path_point_of_the_search.rs
// occt: Contap_ThePathPointOfTheSearch

/// Represents a point on a path during surface search.
/// Stores either a new interior point on an arc or an existing vertex.
#[derive(Clone)]
pub struct ContapThePathPointOfTheSearch {
    point: [f64; 3],        // 3D point coordinates
    tolerance: f64,
    is_new: bool,           // True if point is new (not on vertex)
    vertex_index: Option<i32>,  // Vertex reference if not new
    arc_index: Option<i32>,     // Arc/curve reference
    parameter: f64,         // Parameter on the arc
}

impl ContapThePathPointOfTheSearch {
    /// Create a default (empty) path point.
    pub fn new() -> Self {
        ContapThePathPointOfTheSearch {
            point: [0.0; 3],
            tolerance: 0.0,
            is_new: true,
            vertex_index: None,
            arc_index: None,
            parameter: 0.0,
        }
    }

    /// Create a path point on a vertex.
    pub fn from_vertex(
        px: f64,
        py: f64,
        pz: f64,
        tol: f64,
        vertex_idx: i32,
        arc_idx: i32,
        param: f64,
    ) -> Self {
        ContapThePathPointOfTheSearch {
            point: [px, py, pz],
            tolerance: tol,
            is_new: false,
            vertex_index: Some(vertex_idx),
            arc_index: Some(arc_idx),
            parameter: param,
        }
    }

    /// Create a path point on an arc (interior point).
    pub fn from_arc(
        px: f64,
        py: f64,
        pz: f64,
        tol: f64,
        arc_idx: i32,
        param: f64,
    ) -> Self {
        ContapThePathPointOfTheSearch {
            point: [px, py, pz],
            tolerance: tol,
            is_new: true,
            vertex_index: None,
            arc_index: Some(arc_idx),
            parameter: param,
        }
    }

    /// Set value with vertex reference.
    pub fn set_value_vertex(
        &mut self,
        px: f64,
        py: f64,
        pz: f64,
        tol: f64,
        vertex_idx: i32,
        arc_idx: i32,
        param: f64,
    ) {
        self.point = [px, py, pz];
        self.tolerance = tol;
        self.is_new = false;
        self.vertex_index = Some(vertex_idx);
        self.arc_index = Some(arc_idx);
        self.parameter = param;
    }

    /// Set value for interior arc point.
    pub fn set_value_arc(&mut self, px: f64, py: f64, pz: f64, tol: f64, arc_idx: i32, param: f64) {
        self.point = [px, py, pz];
        self.tolerance = tol;
        self.is_new = true;
        self.vertex_index = None;
        self.arc_index = Some(arc_idx);
        self.parameter = param;
    }

    /// Get the 3D point.
    pub fn value(&self) -> [f64; 3] {
        self.point
    }

    /// Get the tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Check if this is a new point (not on an existing vertex).
    pub fn is_new(&self) -> bool {
        self.is_new
    }

    /// Get vertex index (panics if is_new is true).
    pub fn vertex(&self) -> Option<i32> {
        if self.is_new {
            None
        } else {
            self.vertex_index
        }
    }

    /// Get arc index.
    pub fn arc(&self) -> Option<i32> {
        self.arc_index
    }

    /// Get parameter on arc.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }
}

impl Default for ContapThePathPointOfTheSearch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_default() {
        let pt = ContapThePathPointOfTheSearch::new();
        assert!(pt.is_new());
        assert_eq!(pt.tolerance(), 0.0);
        assert_eq!(pt.value(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_from_vertex() {
        let pt = ContapThePathPointOfTheSearch::from_vertex(1.0, 2.0, 3.0, 1e-5, 5, 10, 0.5);
        assert!(!pt.is_new());
        assert_eq!(pt.tolerance(), 1e-5);
        assert_eq!(pt.value(), [1.0, 2.0, 3.0]);
        assert_eq!(pt.vertex(), Some(5));
        assert_eq!(pt.arc(), Some(10));
        assert_eq!(pt.parameter(), 0.5);
    }

    #[test]
    fn test_from_arc() {
        let pt = ContapThePathPointOfTheSearch::from_arc(1.5, 2.5, 3.5, 1e-6, 7, 0.25);
        assert!(pt.is_new());
        assert_eq!(pt.tolerance(), 1e-6);
        assert_eq!(pt.arc(), Some(7));
    }

    #[test]
    fn test_set_value_vertex() {
        let mut pt = ContapThePathPointOfTheSearch::new();
        pt.set_value_vertex(0.1, 0.2, 0.3, 0.001, 1, 2, 0.75);
        assert!(!pt.is_new());
        assert_eq!(pt.value(), [0.1, 0.2, 0.3]);
    }
}
