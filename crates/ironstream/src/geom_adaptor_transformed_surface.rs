// FILE: geom_adaptor_transformed_surface.rs
// occt: GeomAdaptor_TransformedSurface

/// An adaptor for surfaces with an applied transformation.
/// Wraps surface geometry and applies a transformation to evaluations.
pub struct GeomAdaptorTransformedSurface {
    trsf_matrix: [[f64; 4]; 4],
    u_first: f64,
    u_last: f64,
    v_first: f64,
    v_last: f64,
    tol_u: f64,
    tol_v: f64,
}

impl GeomAdaptorTransformedSurface {
    /// Creates an undefined surface with identity transformation.
    pub fn new() -> Self {
        GeomAdaptorTransformedSurface {
            trsf_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            u_first: 0.0,
            u_last: 1.0,
            v_first: 0.0,
            v_last: 1.0,
            tol_u: 0.0,
            tol_v: 0.0,
        }
    }

    /// Creates a surface adaptor with transformation and parameter bounds.
    pub fn with_transform(
        trsf_matrix: [[f64; 4]; 4],
        u_first: f64,
        u_last: f64,
        v_first: f64,
        v_last: f64,
        tol_u: f64,
        tol_v: f64,
    ) -> Self {
        GeomAdaptorTransformedSurface {
            trsf_matrix,
            u_first,
            u_last,
            v_first,
            v_last,
            tol_u,
            tol_v,
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

    /// Gets U parameter range.
    pub fn u_first(&self) -> f64 {
        self.u_first
    }

    pub fn u_last(&self) -> f64 {
        self.u_last
    }

    /// Gets V parameter range.
    pub fn v_first(&self) -> f64 {
        self.v_first
    }

    pub fn v_last(&self) -> f64 {
        self.v_last
    }

    /// Gets tolerances.
    pub fn tol_u(&self) -> f64 {
        self.tol_u
    }

    pub fn tol_v(&self) -> f64 {
        self.tol_v
    }
}

impl Default for GeomAdaptorTransformedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_transformed_surface_new() {
        let ts = GeomAdaptorTransformedSurface::new();
        assert!((ts.u_first() - 0.0).abs() < PRECISION);
        assert!((ts.u_last() - 1.0).abs() < PRECISION);
        assert!((ts.v_first() - 0.0).abs() < PRECISION);
        assert!((ts.v_last() - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_surface_identity() {
        let ts = GeomAdaptorTransformedSurface::new();
        let (x, y, z) = ts.transform_point(1.0, 2.0, 3.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 2.0).abs() < PRECISION);
        assert!((z - 3.0).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_surface_translation() {
        let m = [
            [1.0, 0.0, 0.0, 2.0],
            [0.0, 1.0, 0.0, 3.0],
            [0.0, 0.0, 1.0, 4.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let ts = GeomAdaptorTransformedSurface::with_transform(m, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0);
        let (x, y, z) = ts.transform_point(0.0, 0.0, 0.0);
        assert!((x - 2.0).abs() < PRECISION);
        assert!((y - 3.0).abs() < PRECISION);
        assert!((z - 4.0).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_surface_parameter_bounds() {
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let ts = GeomAdaptorTransformedSurface::with_transform(
            m, 0.1, 0.9, 0.2, 0.8, 0.01, 0.02,
        );
        assert!((ts.u_first() - 0.1).abs() < PRECISION);
        assert!((ts.u_last() - 0.9).abs() < PRECISION);
        assert!((ts.v_first() - 0.2).abs() < PRECISION);
        assert!((ts.v_last() - 0.8).abs() < PRECISION);
        assert!((ts.tol_u() - 0.01).abs() < PRECISION);
        assert!((ts.tol_v() - 0.02).abs() < PRECISION);
    }

    #[test]
    fn test_transformed_surface_scale() {
        let m = [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let ts = GeomAdaptorTransformedSurface::with_transform(m, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0);
        let (x, y, z) = ts.transform_point(1.0, 1.0, 1.0);
        assert!((x - 2.0).abs() < PRECISION);
        assert!((y - 3.0).abs() < PRECISION);
        assert!((z - 4.0).abs() < PRECISION);
    }
}
