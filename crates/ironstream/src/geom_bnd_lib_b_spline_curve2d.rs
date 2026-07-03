// FILE: geom_bnd_lib_b_spline_curve2d.rs
// occt: GeomBndLib_BSplineCurve2d

/// Computes bounding box for a 2D B-spline curve.
pub struct BSplineCurve2d {
    poles: Vec<[f64; 2]>,
}

impl BSplineCurve2d {
    pub fn new(_curve: &[[f64; 2]]) -> Self {
        BSplineCurve2d {
            poles: _curve.to_vec(),
        }
    }

    pub fn get_box(&self, _tol: f64) -> [f64; 4] {
        self.box_interval(0.0, 1.0, _tol)
    }

    pub fn box_interval(&self, _u1: f64, _u2: f64, tol: f64) -> [f64; 4] {
        let mut bounds = [f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY];
        for pole in &self.poles {
            bounds[0] = bounds[0].min(pole[0]);
            bounds[1] = bounds[1].max(pole[0]);
            bounds[2] = bounds[2].min(pole[1]);
            bounds[3] = bounds[3].max(pole[1]);
        }
        bounds[0] -= tol;
        bounds[1] += tol;
        bounds[2] -= tol;
        bounds[3] += tol;
        bounds
    }

    pub fn box_optimal(&self, u1: f64, u2: f64, tol: f64) -> [f64; 4] {
        self.box_interval(u1, u2, tol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bspline_curve2d_box() {
        let poles = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 0.0]];
        let curve = BSplineCurve2d::new(&poles);
        let bounds = curve.get_box(0.0);
        assert!(bounds[0] <= 0.0);
        assert!(bounds[1] >= 2.0);
    }
}
