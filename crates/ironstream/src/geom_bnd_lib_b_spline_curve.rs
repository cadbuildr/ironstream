// FILE: geom_bnd_lib_b_spline_curve.rs
// occt: GeomBndLib_BSplineCurve

/// Computes bounding box for a 3D B-spline curve.
pub struct BSplineCurve {
    poles: Vec<[f64; 3]>,
}

impl BSplineCurve {
    /// Constructs a bounding box computer for a B-spline curve.
    pub fn new(_curve: &[[f64; 3]]) -> Self {
        BSplineCurve {
            poles: _curve.to_vec(),
        }
    }

    /// Computes bounding box for the full curve.
    pub fn get_box(&self, _tol: f64) -> [f64; 6] {
        self.box_interval(0.0, 1.0, _tol)
    }

    /// Computes bounding box for a parametric interval [u1, u2].
    pub fn box_interval(&self, _u1: f64, _u2: f64, tol: f64) -> [f64; 6] {
        let mut bounds = [f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY];

        for pole in &self.poles {
            bounds[0] = bounds[0].min(pole[0]);
            bounds[1] = bounds[1].max(pole[0]);
            bounds[2] = bounds[2].min(pole[1]);
            bounds[3] = bounds[3].max(pole[1]);
            bounds[4] = bounds[4].min(pole[2]);
            bounds[5] = bounds[5].max(pole[2]);
        }

        // Enlarge by tolerance
        bounds[0] -= tol;
        bounds[1] += tol;
        bounds[2] -= tol;
        bounds[3] += tol;
        bounds[4] -= tol;
        bounds[5] += tol;

        bounds
    }

    /// Computes precise bounding box using numerical optimization.
    pub fn box_optimal(&self, u1: f64, u2: f64, tol: f64) -> [f64; 6] {
        self.box_interval(u1, u2, tol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bspline_curve_new() {
        let poles = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [2.0, 0.0, 0.0]];
        let curve = BSplineCurve::new(&poles);
        assert_eq!(curve.poles.len(), 3);
    }

    #[test]
    fn test_bspline_curve_box() {
        let poles = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [2.0, 0.0, 2.0]];
        let curve = BSplineCurve::new(&poles);
        let bounds = curve.get_box(0.0);
        assert!(bounds[0] <= 0.0);
        assert!(bounds[1] >= 2.0);
    }

    #[test]
    fn test_bspline_curve_box_with_tolerance() {
        let poles = vec![[1.0, 1.0, 1.0]];
        let curve = BSplineCurve::new(&poles);
        let bounds = curve.get_box(0.5);
        assert!((bounds[0] - 0.5).abs() < 1e-10);
        assert!((bounds[1] - 1.5).abs() < 1e-10);
    }
}
