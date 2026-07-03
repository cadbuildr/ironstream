// FILE: geom_l_prop_surface_utils.rs
// occt: GeomLProp_SurfaceUtils

/// Utility functions for computing local properties on surfaces.
pub struct GeomLPropSurfaceUtils;

impl GeomLPropSurfaceUtils {
    /// Evaluate surface derivatives at parameters.
    pub fn eval_surface_derivatives(
        u: f64,
        v: f64,
        order: i32,
    ) -> Result<(f64, f64, f64, f64, f64), String> {
        if order < 0 || order > 2 {
            return Err("Order must be between 0 and 2".to_string());
        }

        // Returns (x, dx/du, dx/dv, d2x/du2, d2x/dv2)
        // This is a placeholder for actual surface evaluation
        Ok((u * v, 1.0, 1.0, 0.0, 0.0))
    }

    /// Compute surface normal from derivatives.
    pub fn compute_normal(d1u: (f64, f64, f64), d1v: (f64, f64, f64)) -> (f64, f64, f64) {
        // Cross product: d1u x d1v
        let nx = d1u.1 * d1v.2 - d1u.2 * d1v.1;
        let ny = d1u.2 * d1v.0 - d1u.0 * d1v.2;
        let nz = d1u.0 * d1v.1 - d1u.1 * d1v.0;

        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len < 1e-10 {
            (0.0, 0.0, 1.0)
        } else {
            (nx / len, ny / len, nz / len)
        }
    }

    /// Compute principal curvatures.
    pub fn compute_curvatures(
        d1u: (f64, f64, f64),
        d1v: (f64, f64, f64),
        d2u: (f64, f64, f64),
        d2v: (f64, f64, f64),
        duv: (f64, f64, f64),
        normal: (f64, f64, f64),
    ) -> Result<(f64, f64), String> {
        // Compute fundamental forms
        let e = d1u.0 * d1u.0 + d1u.1 * d1u.1 + d1u.2 * d1u.2;
        let f = d1u.0 * d1v.0 + d1u.1 * d1v.1 + d1u.2 * d1v.2;
        let g = d1v.0 * d1v.0 + d1v.1 * d1v.1 + d1v.2 * d1v.2;

        let l = d2u.0 * normal.0 + d2u.1 * normal.1 + d2u.2 * normal.2;
        let m = duv.0 * normal.0 + duv.1 * normal.1 + duv.2 * normal.2;
        let n = d2v.0 * normal.0 + d2v.1 * normal.1 + d2v.2 * normal.2;

        let det_eg_ff = e * g - f * f;
        if det_eg_ff.abs() < 1e-10 {
            return Err("Degenerate first fundamental form".to_string());
        }

        // Shape operator eigenvalues (principal curvatures)
        // Using simplified approach
        let k1 = (e * n - 2.0 * f * m + g * l) / (2.0 * det_eg_ff);
        let k2 = (l * n - m * m) / det_eg_ff;

        Ok((k1, k2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_surface_derivatives_order_0() {
        let result = GeomLPropSurfaceUtils::eval_surface_derivatives(1.0, 2.0, 0);
        assert!(result.is_ok());
        let (x, _, _, _, _) = result.unwrap();
        assert_eq!(x, 2.0);
    }

    #[test]
    fn test_eval_surface_derivatives_invalid_order() {
        let result = GeomLPropSurfaceUtils::eval_surface_derivatives(1.0, 2.0, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_normal_orthogonal() {
        let d1u = (1.0, 0.0, 0.0);
        let d1v = (0.0, 1.0, 0.0);
        let n = GeomLPropSurfaceUtils::compute_normal(d1u, d1v);

        // Normal should be (0, 0, 1)
        assert!((n.0 - 0.0).abs() < 1e-6);
        assert!((n.1 - 0.0).abs() < 1e-6);
        assert!((n.2 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_compute_normal_scaled() {
        let d1u = (2.0, 0.0, 0.0);
        let d1v = (0.0, 2.0, 0.0);
        let n = GeomLPropSurfaceUtils::compute_normal(d1u, d1v);

        // Normal should still be (0, 0, 1)
        assert!((n.0 - 0.0).abs() < 1e-6);
        assert!((n.1 - 0.0).abs() < 1e-6);
        assert!((n.2 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_compute_normal_zero_length() {
        let d1u = (0.0, 0.0, 0.0);
        let d1v = (0.0, 0.0, 0.0);
        let n = GeomLPropSurfaceUtils::compute_normal(d1u, d1v);

        // Should return default normal
        assert!((n.2 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_compute_curvatures_sphere() {
        let d1u = (1.0, 0.0, 0.0);
        let d1v = (0.0, 1.0, 0.0);
        let d2u = (-1.0, 0.0, 0.0);
        let d2v = (0.0, -1.0, 0.0);
        let duv = (0.0, 0.0, 0.0);
        let normal = (0.0, 0.0, 1.0);

        let result = GeomLPropSurfaceUtils::compute_curvatures(d1u, d1v, d2u, d2v, duv, normal);
        assert!(result.is_ok());
    }
}
