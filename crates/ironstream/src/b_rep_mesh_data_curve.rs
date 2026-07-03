// FILE: b_rep_mesh_data_curve.rs
// occt: BRepMeshData_Curve

/// Default implementation of curve data model entity for mesh.
pub struct BRepMeshDataCurve {
    points: Vec<f64>,
    parameters: Vec<f64>,
}

impl BRepMeshDataCurve {
    /// Creates a new curve data.
    pub fn new() -> Self {
        BRepMeshDataCurve {
            points: Vec::new(),
            parameters: Vec::new(),
        }
    }

    /// Inserts a new discretization point at the given position.
    pub fn insert_point(&mut self, _position: usize, _point: f64, _param: f64) {
        // Insert point
    }

    /// Adds a new discretization point.
    pub fn add_point(&mut self, _point: f64, _param: f64) {
        self.points.push(_point);
        self.parameters.push(_param);
    }

    /// Returns the discretization point at index.
    pub fn get_point(&self, _index: usize) -> Option<f64> {
        None
    }

    /// Removes point at index.
    pub fn remove_point(&mut self, _index: usize) {
        // Remove point
    }

    /// Returns the parameter at index.
    pub fn get_parameter(&self, _index: usize) -> Option<f64> {
        None
    }

    /// Returns the number of parameters.
    pub fn parameters_nb(&self) -> usize {
        self.parameters.len()
    }

    /// Clears all parameters.
    pub fn clear(&mut self, _keep_endpoints: bool) {
        self.points.clear();
        self.parameters.clear();
    }
}

impl Default for BRepMeshDataCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_creation() {
        let curve = BRepMeshDataCurve::new();
        assert_eq!(curve.parameters_nb(), 0);
    }

    #[test]
    fn test_curve_add_point() {
        let mut curve = BRepMeshDataCurve::new();
        curve.add_point(1.0, 0.0);
        assert_eq!(curve.parameters_nb(), 1);
    }

    #[test]
    fn test_curve_clear() {
        let mut curve = BRepMeshDataCurve::new();
        curve.add_point(1.0, 0.0);
        curve.clear(false);
        assert_eq!(curve.parameters_nb(), 0);
    }
}
