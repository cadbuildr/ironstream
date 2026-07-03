// FILE: ch_fi_ds_common_point.rs
// occt: ChFiDS_CommonPoint

/// A point that is common to two adjacent fillets or to a fillet and an edge.
/// This represents start/end of fillet common to 2 adjacent fillets
/// and to an edge on one of 2 faces participating in the construction of the fillet.
#[derive(Debug, Clone)]
pub struct ChFiDsCommonPoint {
    /// The 3D point
    point: [f64; 3],
    /// Output 3D vector if present
    vector: Option<[f64; 3]>,
    /// Tolerance/fuzziness on the point
    tolerance: f64,
    /// Whether this is a vertex on the restriction
    is_vertex: bool,
    /// Whether this is on an arc of restriction
    is_on_arc: bool,
    /// Parameter on arc if on_arc is true
    param_on_arc: f64,
    /// Parameter on spine
    param_on_spine: f64,
}

impl ChFiDsCommonPoint {
    /// Create a new empty CommonPoint
    pub fn new() -> Self {
        Self {
            point: [0.0; 3],
            vector: None,
            tolerance: 0.0,
            is_vertex: false,
            is_on_arc: false,
            param_on_arc: 0.0,
            param_on_spine: 0.0,
        }
    }

    /// Reset to default values
    pub fn reset(&mut self) {
        self.point = [0.0; 3];
        self.vector = None;
        self.tolerance = 0.0;
        self.is_vertex = false;
        self.is_on_arc = false;
        self.param_on_arc = 0.0;
        self.param_on_spine = 0.0;
    }

    /// Set the point as a vertex on the initial restriction facet
    pub fn set_vertex(&mut self) {
        self.is_vertex = true;
    }

    /// Set the arc parameters
    pub fn set_arc(&mut self, param: f64, tolerance: f64) {
        self.is_on_arc = true;
        self.param_on_arc = param;
        if tolerance > self.tolerance {
            self.tolerance = tolerance;
        }
    }

    /// Set the parameter on the spine
    pub fn set_parameter(&mut self, param: f64) {
        self.param_on_spine = param;
    }

    /// Set the 3D point
    pub fn set_point(&mut self, x: f64, y: f64, z: f64) {
        self.point = [x, y, z];
    }

    /// Set the output 3D vector
    pub fn set_vector(&mut self, vx: f64, vy: f64, vz: f64) {
        self.vector = Some([vx, vy, vz]);
    }

    /// Set the tolerance/fuzziness on the point
    pub fn set_tolerance(&mut self, tol: f64) {
        if tol > self.tolerance {
            self.tolerance = tol;
        }
    }

    /// Get tolerance/fuzziness on the point
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Check if this point is a vertex on the initial restriction
    pub fn is_vertex(&self) -> bool {
        self.is_vertex
    }

    /// Check if this point is on an arc of the restriction
    pub fn is_on_arc(&self) -> bool {
        self.is_on_arc
    }

    /// Get the parameter on the arc (if on_arc)
    pub fn parameter_on_arc(&self) -> f64 {
        self.param_on_arc
    }

    /// Get the parameter on the spine
    pub fn parameter(&self) -> f64 {
        self.param_on_spine
    }

    /// Get the 3D point
    pub fn point(&self) -> (f64, f64, f64) {
        (self.point[0], self.point[1], self.point[2])
    }

    /// Check if output vector is stored
    pub fn has_vector(&self) -> bool {
        self.vector.is_some()
    }

    /// Get the output 3D vector (if present)
    pub fn vector(&self) -> Option<(f64, f64, f64)> {
        self.vector.map(|v| (v[0], v[1], v[2]))
    }
}

impl Default for ChFiDsCommonPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cp = ChFiDsCommonPoint::new();
        assert!(!cp.is_vertex());
        assert!(!cp.is_on_arc());
        assert!(!cp.has_vector());
        assert_eq!(cp.tolerance(), 0.0);
        assert_eq!(cp.parameter(), 0.0);
    }

    #[test]
    fn test_set_point() {
        let mut cp = ChFiDsCommonPoint::new();
        cp.set_point(1.5, 2.5, 3.5);
        let (x, y, z) = cp.point();
        assert_eq!(x, 1.5);
        assert_eq!(y, 2.5);
        assert_eq!(z, 3.5);
    }

    #[test]
    fn test_set_vector() {
        let mut cp = ChFiDsCommonPoint::new();
        cp.set_vector(0.5, 0.6, 0.7);
        assert!(cp.has_vector());
        let (vx, vy, vz) = cp.vector().unwrap();
        assert_eq!(vx, 0.5);
        assert_eq!(vy, 0.6);
        assert_eq!(vz, 0.7);
    }

    #[test]
    fn test_set_vertex() {
        let mut cp = ChFiDsCommonPoint::new();
        cp.set_vertex();
        assert!(cp.is_vertex());
    }

    #[test]
    fn test_set_arc() {
        let mut cp = ChFiDsCommonPoint::new();
        cp.set_arc(2.5, 0.01);
        assert!(cp.is_on_arc());
        assert_eq!(cp.parameter_on_arc(), 2.5);
        assert_eq!(cp.tolerance(), 0.01);
    }

    #[test]
    fn test_set_parameter() {
        let mut cp = ChFiDsCommonPoint::new();
        cp.set_parameter(4.5);
        assert_eq!(cp.parameter(), 4.5);
    }

    #[test]
    fn test_reset() {
        let mut cp = ChFiDsCommonPoint::new();
        cp.set_point(1.0, 2.0, 3.0);
        cp.set_vector(1.0, 0.0, 0.0);
        cp.set_vertex();
        cp.reset();
        assert!(!cp.is_vertex());
        assert!(!cp.has_vector());
    }

    #[test]
    fn test_default() {
        let cp = ChFiDsCommonPoint::default();
        assert!(!cp.is_vertex());
    }
}
