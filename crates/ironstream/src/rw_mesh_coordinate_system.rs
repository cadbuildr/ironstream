// FILE: rw_mesh_coordinate_system.rs
// occt: RWMesh_CoordinateSystem

/// Defines a coordinate system for mesh data
#[derive(Clone, Debug)]
pub struct RwMeshCoordinateSystem {
    origin: [f64; 3],
    axis_x: [f64; 3],
    axis_y: [f64; 3],
    axis_z: [f64; 3],
}

impl RwMeshCoordinateSystem {
    /// Creates a default coordinate system (identity)
    pub fn new() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            axis_x: [1.0, 0.0, 0.0],
            axis_y: [0.0, 1.0, 0.0],
            axis_z: [0.0, 0.0, 1.0],
        }
    }

    /// Returns the origin
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    /// Returns X axis
    pub fn axis_x(&self) -> [f64; 3] {
        self.axis_x
    }

    /// Returns Y axis
    pub fn axis_y(&self) -> [f64; 3] {
        self.axis_y
    }

    /// Returns Z axis
    pub fn axis_z(&self) -> [f64; 3] {
        self.axis_z
    }
}

impl Default for RwMeshCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_identity() {
        let sys = RwMeshCoordinateSystem::new();
        assert_eq!(sys.origin(), [0.0, 0.0, 0.0]);
        assert_eq!(sys.axis_x(), [1.0, 0.0, 0.0]);
        assert_eq!(sys.axis_y(), [0.0, 1.0, 0.0]);
        assert_eq!(sys.axis_z(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_axes_orthogonal() {
        let sys = RwMeshCoordinateSystem::new();
        let x = sys.axis_x();
        let y = sys.axis_y();
        let z = sys.axis_z();

        let xy_dot = x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
        let xz_dot = x[0] * z[0] + x[1] * z[1] + x[2] * z[2];
        let yz_dot = y[0] * z[0] + y[1] * z[1] + y[2] * z[2];

        assert!((xy_dot).abs() < 1e-10);
        assert!((xz_dot).abs() < 1e-10);
        assert!((yz_dot).abs() < 1e-10);
    }

    #[test]
    fn test_clone() {
        let sys = RwMeshCoordinateSystem::new();
        let cloned = sys.clone();
        assert_eq!(cloned.origin(), sys.origin());
    }
}
