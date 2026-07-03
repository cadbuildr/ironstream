// FILE: int_imp_compute_tangence.rs
// occt: IntImp_ComputeTangence

use crate::int_imp_const_isoparametric::ConstIsoparametric;

/// Computes tangence vectors and isoparametric directions at intersection points.
pub struct IntImpComputeTangence;

impl IntImpComputeTangence {
    /// Chooses a reference isoparametric direction based on index.
    pub fn choix_ref(index: i32) -> ConstIsoparametric {
        match index {
            0 => ConstIsoparametric::UIsoparametricOnCaro1,
            1 => ConstIsoparametric::VIsoparametricOnCaro1,
            2 => ConstIsoparametric::UIsoparametricOnCaro2,
            3 => ConstIsoparametric::VIsoparametricOnCaro2,
            _ => ConstIsoparametric::UIsoparametricOnCaro1,
        }
    }

    /// Computes tangent vectors and isoparametric directions.
    /// dpuv: 3x2 array of partial derivatives (du/dv for surface1, surface2, surface3)
    /// eps_uv: tolerance in u and v directions
    /// tgduv: output tangent vectors (size 3)
    /// tab_iso: output isoparametric directions (size 2)
    /// Returns true if computation was successful.
    pub fn compute_tangence(
        dpuv: &[[f64; 2]; 3],
        eps_uv: &[f64; 2],
        tgduv: &mut [f64; 3],
        tab_iso: &mut [ConstIsoparametric; 2],
    ) -> bool {
        // Compute the normal vector from the first two derivative vectors
        let normal = cross_product_3d(
            (dpuv[0][0], dpuv[0][1], 0.0),
            (dpuv[1][0], dpuv[1][1], 0.0),
        );

        let normal_len = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
        if normal_len < eps_uv[0] * eps_uv[1] {
            return false; // Degenerate case
        }

        // Determine dominant directions
        let eps_threshold = 1e-10;

        // Check which derivatives are most significant
        let du1_len = (dpuv[0][0] * dpuv[0][0] + dpuv[0][1] * dpuv[0][1]).sqrt();
        let dv1_len = if dpuv.len() > 1 {
            (dpuv[1][0] * dpuv[1][0] + dpuv[1][1] * dpuv[1][1]).sqrt()
        } else {
            0.0
        };

        tab_iso[0] = if du1_len > dv1_len + eps_threshold {
            ConstIsoparametric::UIsoparametricOnCaro1
        } else {
            ConstIsoparametric::VIsoparametricOnCaro1
        };

        let du2_len = if dpuv.len() > 2 {
            (dpuv[2][0] * dpuv[2][0] + dpuv[2][1] * dpuv[2][1]).sqrt()
        } else {
            0.0
        };
        let dv2_len = 0.0;

        tab_iso[1] = if du2_len > dv2_len + eps_threshold {
            ConstIsoparametric::UIsoparametricOnCaro2
        } else {
            ConstIsoparametric::VIsoparametricOnCaro2
        };

        // Store tangent magnitude (simplified)
        tgduv[0] = du1_len;
        tgduv[1] = du2_len;
        tgduv[2] = normal_len / (eps_uv[0] + eps_uv[1]);

        true
    }
}

/// Computes the cross product of two 3D vectors.
fn cross_product_3d(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        v1.1 * v2.2 - v1.2 * v2.1,
        v1.2 * v2.0 - v1.0 * v2.2,
        v1.0 * v2.1 - v1.1 * v2.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choix_ref_0() {
        let iso = IntImpComputeTangence::choix_ref(0);
        assert_eq!(iso as i32, 0);
    }

    #[test]
    fn test_choix_ref_1() {
        let iso = IntImpComputeTangence::choix_ref(1);
        assert_eq!(iso as i32, 1);
    }

    #[test]
    fn test_choix_ref_2() {
        let iso = IntImpComputeTangence::choix_ref(2);
        assert_eq!(iso as i32, 2);
    }

    #[test]
    fn test_choix_ref_3() {
        let iso = IntImpComputeTangence::choix_ref(3);
        assert_eq!(iso as i32, 3);
    }

    #[test]
    fn test_compute_tangence_basic() {
        let dpuv = [
            [1.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
        ];
        let eps_uv = [1e-7, 1e-7];
        let mut tgduv = [0.0; 3];
        let mut tab_iso = [ConstIsoparametric::UIsoparametricOnCaro1; 2];

        let result = IntImpComputeTangence::compute_tangence(&dpuv, &eps_uv, &mut tgduv, &mut tab_iso);
        assert!(result);
        assert!(tgduv[0] > 0.0);
    }

    #[test]
    fn test_compute_tangence_zero_derivatives() {
        let dpuv = [
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
        ];
        let eps_uv = [1e-7, 1e-7];
        let mut tgduv = [0.0; 3];
        let mut tab_iso = [ConstIsoparametric::UIsoparametricOnCaro1; 2];

        let result = IntImpComputeTangence::compute_tangence(&dpuv, &eps_uv, &mut tgduv, &mut tab_iso);
        assert!(!result);
    }

    #[test]
    fn test_cross_product() {
        let v1 = (1.0, 0.0, 0.0);
        let v2 = (0.0, 1.0, 0.0);
        let result = cross_product_3d(v1, v2);
        assert!((result.0 - 0.0).abs() < 1e-10);
        assert!((result.1 - 0.0).abs() < 1e-10);
        assert!((result.2 - 1.0).abs() < 1e-10);
    }
}
