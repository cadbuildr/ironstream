// FILE: l_prop_curve_utils.rs
// occt: LProp_CurveUtils

/// Utility functions for computing curve local properties.
pub struct LPropCurveUtils;

impl LPropCurveUtils {
    /// Compute curvature from derivatives.
    pub fn curvature(d1: (f64, f64, f64), d2: (f64, f64, f64)) -> f64 {
        // curvature = |d1 x d2| / |d1|^3
        let cross_x = d1.1 * d2.2 - d1.2 * d2.1;
        let cross_y = d1.2 * d2.0 - d1.0 * d2.2;
        let cross_z = d1.0 * d2.1 - d1.1 * d2.0;

        let cross_mag = (cross_x * cross_x + cross_y * cross_y + cross_z * cross_z).sqrt();
        let d1_mag = (d1.0 * d1.0 + d1.1 * d1.1 + d1.2 * d1.2).sqrt();

        if d1_mag < 1e-10 {
            0.0
        } else {
            cross_mag / (d1_mag * d1_mag * d1_mag)
        }
    }

    /// Compute torsion from derivatives.
    pub fn torsion(
        d1: (f64, f64, f64),
        d2: (f64, f64, f64),
        d3: (f64, f64, f64),
    ) -> f64 {
        // torsion = (d1 x d2) . d3 / |d1 x d2|^2
        let cross_x = d1.1 * d2.2 - d1.2 * d2.1;
        let cross_y = d1.2 * d2.0 - d1.0 * d2.2;
        let cross_z = d1.0 * d2.1 - d1.1 * d2.0;

        let cross_mag_sq = cross_x * cross_x + cross_y * cross_y + cross_z * cross_z;
        let dot = cross_x * d3.0 + cross_y * d3.1 + cross_z * d3.2;

        if cross_mag_sq < 1e-20 {
            0.0
        } else {
            dot / cross_mag_sq
        }
    }

    /// Compute tangent direction from first derivative.
    pub fn tangent(d1: (f64, f64, f64)) -> Option<(f64, f64, f64)> {
        let mag = (d1.0 * d1.0 + d1.1 * d1.1 + d1.2 * d1.2).sqrt();
        if mag < 1e-10 {
            None
        } else {
            Some((d1.0 / mag, d1.1 / mag, d1.2 / mag))
        }
    }

    /// Compute normal direction from first and second derivatives.
    pub fn normal(
        d1: (f64, f64, f64),
        d2: (f64, f64, f64),
    ) -> Option<(f64, f64, f64)> {
        let d1_mag = (d1.0 * d1.0 + d1.1 * d1.1 + d1.2 * d1.2).sqrt();
        if d1_mag < 1e-10 {
            return None;
        }

        let cross_x = d1.1 * d2.2 - d1.2 * d2.1;
        let cross_y = d1.2 * d2.0 - d1.0 * d2.2;
        let cross_z = d1.0 * d2.1 - d1.1 * d2.0;

        let cross_mag = (cross_x * cross_x + cross_y * cross_y + cross_z * cross_z).sqrt();
        if cross_mag < 1e-10 {
            return None;
        }

        // Normal = (T x (T x T'')) / |T x (T x T'')|
        let t_x = d1.0 / d1_mag;
        let t_y = d1.1 / d1_mag;
        let t_z = d1.2 / d1_mag;

        let tx_d2_x = t_y * d2.2 - t_z * d2.1;
        let tx_d2_y = t_z * d2.0 - t_x * d2.2;
        let tx_d2_z = t_x * d2.1 - t_y * d2.0;

        let n_x = t_y * tx_d2_z - t_z * tx_d2_y;
        let n_y = t_z * tx_d2_x - t_x * tx_d2_z;
        let n_z = t_x * tx_d2_y - t_y * tx_d2_x;

        let n_mag = (n_x * n_x + n_y * n_y + n_z * n_z).sqrt();
        if n_mag < 1e-10 {
            None
        } else {
            Some((n_x / n_mag, n_y / n_mag, n_z / n_mag))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curvature_line() {
        // For a line: d1 = (1, 0, 0), d2 = (0, 0, 0)
        let d1 = (1.0, 0.0, 0.0);
        let d2 = (0.0, 0.0, 0.0);
        let k = LPropCurveUtils::curvature(d1, d2);
        assert!(k.abs() < 1e-6); // Line has zero curvature
    }

    #[test]
    fn test_curvature_circle() {
        // For a unit circle at parameter t=0: d1 = (0, 1, 0), d2 = (-1, 0, 0)
        let d1 = (0.0, 1.0, 0.0);
        let d2 = (-1.0, 0.0, 0.0);
        let k = LPropCurveUtils::curvature(d1, d2);
        assert!((k - 1.0).abs() < 1e-6); // Unit circle has unit curvature
    }

    #[test]
    fn test_tangent_normalized() {
        let d1 = (3.0, 4.0, 0.0);
        let t = LPropCurveUtils::tangent(d1).unwrap();
        let mag = (t.0 * t.0 + t.1 * t.1 + t.2 * t.2).sqrt();
        assert!((mag - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_tangent_zero() {
        let d1 = (0.0, 0.0, 0.0);
        let t = LPropCurveUtils::tangent(d1);
        assert!(t.is_none());
    }

    #[test]
    fn test_torsion_line() {
        // For a line: torsion = 0
        let d1 = (1.0, 0.0, 0.0);
        let d2 = (0.0, 0.0, 0.0);
        let d3 = (0.0, 0.0, 0.0);
        let tau = LPropCurveUtils::torsion(d1, d2, d3);
        assert!(tau.abs() < 1e-6);
    }
}
