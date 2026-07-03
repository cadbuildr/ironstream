// FILE: b_rep_mesh_data_p_curve.rs
// occt: BRepMeshData_PCurve

/// Parametric curve data for mesh operations on faces.
pub struct BRepMeshDataPCurve {
    curve_id: usize,
    points: Vec<f64>,
}

impl BRepMeshDataPCurve {
    /// Creates a new parametric curve data.
    pub fn new(_curve_id: usize) -> Self {
        BRepMeshDataPCurve {
            curve_id: _curve_id,
            points: Vec::new(),
        }
    }

    /// Adds a point to the parametric curve.
    pub fn add_point(&mut self, _point_id: usize) {
        self.points.push(0.0);
    }

    /// Returns the number of points.
    pub fn points_nb(&self) -> usize {
        self.points.len()
    }

    /// Gets the point at index.
    pub fn point(&self, _index: usize) -> Option<f64> {
        if _index < self.points.len() {
            Some(self.points[_index])
        } else {
            None
        }
    }

    /// Clears all points.
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcurve_creation() {
        let pcurve = BRepMeshDataPCurve::new(0);
        assert_eq!(pcurve.curve_id, 0);
        assert_eq!(pcurve.points_nb(), 0);
    }

    #[test]
    fn test_pcurve_add_point() {
        let mut pcurve = BRepMeshDataPCurve::new(0);
        pcurve.add_point(1);
        assert_eq!(pcurve.points_nb(), 1);
    }

    #[test]
    fn test_pcurve_clear() {
        let mut pcurve = BRepMeshDataPCurve::new(0);
        pcurve.add_point(1);
        pcurve.clear();
        assert_eq!(pcurve.points_nb(), 0);
    }
}
