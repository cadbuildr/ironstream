// FILE: geom_adaptor_transformed_curve.rs
// occt: GeomAdaptor_TransformedCurve

/// An adaptor for curves with an applied transformation.
/// Wraps a curve geometry and applies a transformation to all evaluations.
pub struct GeomAdaptorTransformedCurve {
    // In real OCCT, this would wrap a Geom_Curve
    // For this Rust port, we store transformation parameters
    trsf_matrix: [[f64; 4]; 4],
    first_param: f64,
    last_param: f64,
}

impl GeomAdaptorTransformedCurve {
    /// Creates an undefined curve with identity transformation.
    pub fn new() -> Self {
        GeomAdaptorTransformedCurve {
            trsf_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            first_param: 0.0,
            last_param: 1.0,
        }
    }

    /// Creates a curve adaptor with transformation.
    pub fn with_transform(
        trsf_matrix: [[f64; 4]; 4],
        first: f64,
        last: f64,
    ) -> Self {
        GeomAdaptorTransformedCurve {
            trsf_matrix,
            first_param: first,
            last_param: last,
        }
    }

    /// Sets the transformation.
    pub fn set_trsf(&mut self, trsf_matrix: [[f64; 4]; 4]) {
        self.trsf_matrix = trsf_matrix;
    }

    /// Gets the transformation matrix.
    pub fn get_trsf(&self) -> [[f64; 4]; 4] {
        self.trsf_matrix
    }

    /// Applies transformation to a 3D point.
    pub fn transform_point(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let m = &self.trsf_matrix;
        let new_x = m[0][0] * x + m[0][1] * y + m[0][2] * z + m[0][3];
        let new_y = m[1][0] * x + m[1][1] * y + m[1][2] * z + m[1][3];
        let new_z = m[2][0] * x + m[2][1] * y + m[2][2] * z + m[2][3];
        (new_x, new_y, new_z)
    }

    /// Gets parameter range.
    pub fn first(&self) -> f64 {
        self.first_param
    }

    pub fn last(&self) -> f64 {
        self.last_param
    }
}

impl Default for GeomAdaptorTransformedCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_transformed_curve_new() {
        let tc = GeomAdaptorTransformedCurve::new();
        assert!((tc.first() - 0.0).abs() < PRECISION);
        assert!((tc.last() - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_curve_identity() {
        let tc = GeomAdaptorTransformedCurve::new();
        let (x, y, z) = tc.transform_point(1.0, 2.0, 3.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 2.0).abs() < PRECISION);
        assert!((z - 3.0).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_curve_translation() {
        let mut m = [
            [1.0, 0.0, 0.0, 5.0],
            [0.0, 1.0, 0.0, 10.0],
            [0.0, 0.0, 1.0, 15.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let tc = GeomAdaptorTransformedCurve::with_transform(m, 0.0, 1.0);
        let (x, y, z) = tc.transform_point(0.0, 0.0, 0.0);
        assert!((x - 5.0).abs() < PRECISION);
        assert!((y - 10.0).abs() < PRECISION);
        assert!((z - 15.0).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_curve_scale() {
        let m = [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let tc = GeomAdaptorTransformedCurve::with_transform(m, 0.0, 1.0);
        let (x, y, z) = tc.transform_point(1.0, 1.0, 1.0);
        assert!((x - 2.0).abs() < PRECISION);
        assert!((y - 3.0).abs() < PRECISION);
        assert!((z - 4.0).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_curve_get_range() {
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let tc = GeomAdaptorTransformedCurve::with_transform(m, 0.5, 2.5);
        assert!((tc.first() - 0.5).abs() < PRECISION);
        assert!((tc.last() - 2.5).abs() < PRECISION);
    }
}
