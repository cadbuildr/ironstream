// FILE: geom_plate_plate_g0_criterion.rs
// occt: GeomPlate_PlateG0Criterion

/// G0 criterion for plate surface approximation.
/// Evaluates whether geometric continuity (G0) is satisfied.
pub struct GeomPlatePlateG0Criterion {
    data_xy: Vec<[f64; 2]>,
    data_xyz: Vec<[f64; 3]>,
    maximum: f64,
}

impl GeomPlatePlateG0Criterion {
    /// Create a G0 criterion with data points and maximum tolerance
    pub fn new(data_xy: Vec<[f64; 2]>, data_xyz: Vec<[f64; 3]>, maximum: f64) -> Self {
        GeomPlatePlateG0Criterion {
            data_xy,
            data_xyz,
            maximum,
        }
    }

    /// Get the XY data points
    pub fn data_xy(&self) -> &[[f64; 2]] {
        &self.data_xy
    }

    /// Get the XYZ data points
    pub fn data_xyz(&self) -> &[[f64; 3]] {
        &self.data_xyz
    }

    /// Get the maximum tolerance value
    pub fn maximum(&self) -> f64 {
        self.maximum
    }

    /// Check if criterion is satisfied (stub - depends on patch context)
    pub fn is_satisfied(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let xy_data = vec![[0.0, 0.0], [1.0, 1.0]];
        let xyz_data = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        let max_val = 0.01;

        let criterion = GeomPlatePlateG0Criterion::new(xy_data.clone(), xyz_data.clone(), max_val);

        assert_eq!(criterion.data_xy().len(), 2);
        assert_eq!(criterion.data_xyz().len(), 2);
        assert_eq!(criterion.maximum(), 0.01);
    }

    #[test]
    fn test_data_accessors() {
        let xy_data = vec![[0.5, 0.5]];
        let xyz_data = vec![[0.5, 0.5, 0.5]];

        let criterion = GeomPlatePlateG0Criterion::new(xy_data, xyz_data, 0.001);

        assert_eq!(criterion.data_xy()[0], [0.5, 0.5]);
        assert_eq!(criterion.data_xyz()[0], [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_is_satisfied() {
        let criterion = GeomPlatePlateG0Criterion::new(vec![], vec![], 0.01);
        assert!(criterion.is_satisfied());
    }
}
