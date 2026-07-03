// FILE: plate_free_gto_c_constraint.rs
// occt: Plate_FreeGtoCConstraint

use crate::plate_d1::PlateD1;
use crate::plate_d1::{XY, XYZ};
use crate::plate_d2::PlateD2;
use crate::plate_d3::PlateD3;
use crate::plate_linear_scalar_constraint::PlateLinearScalarConstraint;
use crate::plate_pinpoint_constraint::PlatePinpointConstraint;

const NORMIN: f64 = 1e-10;
const COSMIN: f64 = 1e-2;

/// Plate_FreeGtoCConstraint: Define a G1, G2 or G3 constraint on the Plate
/// using weaker constraint than GtoCConstraint
pub struct PlateFreeGtoCConstraint {
    pnt2d: XY,
    nb_pp_constraints: usize,
    nb_ls_constraints: usize,
    ppc: [PlatePinpointConstraint; 5],
    lsc: [PlateLinearScalarConstraint; 4],
}

impl PlateFreeGtoCConstraint {
    /// G1 constraint
    pub fn new_g1(
        point2d: XY,
        d1s: &PlateD1,
        d1t: &PlateD1,
        incremental_load: f64,
        orientation: i32,
    ) -> Self {
        let mut result = PlateFreeGtoCConstraint {
            pnt2d: point2d,
            nb_pp_constraints: 0,
            nb_ls_constraints: 0,
            ppc: [PlatePinpointConstraint::new(); 5],
            lsc: [
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
            ],
        };

        let mut normale = d1t.du.cross(&d1t.dv);
        if normale.modulus() < NORMIN {
            return result;
        }
        normale.normalize();

        if (incremental_load - 1.0).abs() > 1e-14 {
            let mut n0 = d1s.du.cross(&d1s.dv);
            if n0.modulus() < NORMIN {
                return result;
            }
            n0.normalize();

            let mut n1 = normale;
            if orientation != 0 {
                n1 = n1 * (orientation as f64);
            }

            let mut c = n0.dot(&n1);
            if orientation == 0 {
                if c < 0.0 {
                    c = -c;
                    n1 = -n1;
                }
            }

            let s = n0.cross_magnitude(&n1);
            if s < 1e-2 && c < 0.0 {
                return result;
            }

            let angle = s.atan2(c);
            let mut d = n0.cross(&n1);
            d.normalize();

            let rot_angle = angle * (incremental_load - 1.0);
            let cos_angle = rot_angle.cos();
            let sin_angle = rot_angle.sin();

            // Apply rotation: v' = v*cos + (d x v)*sin + d*(d . v)*(1-cos)
            let dxn = d.cross(&normale);
            let d_dot_n = d.dot(&normale);
            normale = XYZ::new(
                normale.x * cos_angle + dxn.x * sin_angle + d.x * d_dot_n * (1.0 - cos_angle),
                normale.y * cos_angle + dxn.y * sin_angle + d.y * d_dot_n * (1.0 - cos_angle),
                normale.z * cos_angle + dxn.z * sin_angle + d.z * d_dot_n * (1.0 - cos_angle),
            );
        }

        let du = d1s.du * -1.0;
        let dv = d1s.dv * -1.0;

        result.lsc[0] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, du, 1, 0),
            normale,
        );
        result.lsc[1] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, dv, 0, 1),
            normale,
        );
        result.nb_ls_constraints = 2;

        result
    }

    /// G1 + G2 constraint
    pub fn new_g1_g2(
        point2d: XY,
        d1s: &PlateD1,
        d1t0: &PlateD1,
        d2s: &PlateD2,
        d2t0: &PlateD2,
        incremental_load: f64,
        orientation: i32,
    ) -> Self {
        let mut result = PlateFreeGtoCConstraint {
            pnt2d: point2d,
            nb_pp_constraints: 0,
            nb_ls_constraints: 0,
            ppc: [PlatePinpointConstraint::new(); 5],
            lsc: [
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
            ],
        };

        let mut d1t = *d1t0;
        let mut d2t = *d2t0;

        let mut normale = d1t.du.cross(&d1t.dv);
        if normale.modulus() < NORMIN {
            return result;
        }
        normale.normalize();

        let mut normale_s = d1s.du.cross(&d1s.dv);
        if normale_s.modulus() < NORMIN {
            if (incremental_load - 1.0).abs() > 1e-14 {
                return result;
            }
            let du = d1s.du * -1.0;
            let dv = d1s.dv * -1.0;
            result.lsc[0] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, du, 1, 0),
                normale,
            );
            result.lsc[1] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, dv, 0, 1),
                normale,
            );
            result.nb_ls_constraints = 2;
            return result;
        }
        normale_s.normalize();

        if (incremental_load - 1.0).abs() > 1e-14 {
            let n0 = normale_s;
            let mut n1 = normale;
            if orientation != 0 {
                n1 = n1 * (orientation as f64);
            }
            let mut c = n0.dot(&n1);
            if orientation == 0 {
                if c < 0.0 {
                    c = -c;
                    n1 = -n1;
                }
            }
            let s = n0.cross_magnitude(&n1);
            if s < 1e-2 && c < 0.0 {
                return result;
            }
            let angle = s.atan2(c);
            let mut d = n0.cross(&n1);
            d.normalize();

            let rot_angle = angle * (incremental_load - 1.0);
            let cos_angle = rot_angle.cos();
            let sin_angle = rot_angle.sin();

            // Rotate normale
            let dxn = d.cross(&normale);
            let d_dot_n = d.dot(&normale);
            normale = XYZ::new(
                normale.x * cos_angle + dxn.x * sin_angle + d.x * d_dot_n * (1.0 - cos_angle),
                normale.y * cos_angle + dxn.y * sin_angle + d.y * d_dot_n * (1.0 - cos_angle),
                normale.z * cos_angle + dxn.z * sin_angle + d.z * d_dot_n * (1.0 - cos_angle),
            );

            // Rotate d1t derivatives
            let dxdu = d.cross(&d1t.du);
            let d_dot_du = d.dot(&d1t.du);
            d1t.du = XYZ::new(
                d1t.du.x * cos_angle + dxdu.x * sin_angle + d.x * d_dot_du * (1.0 - cos_angle),
                d1t.du.y * cos_angle + dxdu.y * sin_angle + d.y * d_dot_du * (1.0 - cos_angle),
                d1t.du.z * cos_angle + dxdu.z * sin_angle + d.z * d_dot_du * (1.0 - cos_angle),
            );

            let dxdv = d.cross(&d1t.dv);
            let d_dot_dv = d.dot(&d1t.dv);
            d1t.dv = XYZ::new(
                d1t.dv.x * cos_angle + dxdv.x * sin_angle + d.x * d_dot_dv * (1.0 - cos_angle),
                d1t.dv.y * cos_angle + dxdv.y * sin_angle + d.y * d_dot_dv * (1.0 - cos_angle),
                d1t.dv.z * cos_angle + dxdv.z * sin_angle + d.z * d_dot_dv * (1.0 - cos_angle),
            );

            // Rotate d2t derivatives
            let dxduu = d.cross(&d2t.duu);
            let d_dot_duu = d.dot(&d2t.duu);
            d2t.duu = XYZ::new(
                d2t.duu.x * cos_angle + dxduu.x * sin_angle + d.x * d_dot_duu * (1.0 - cos_angle),
                d2t.duu.y * cos_angle + dxduu.y * sin_angle + d.y * d_dot_duu * (1.0 - cos_angle),
                d2t.duu.z * cos_angle + dxduu.z * sin_angle + d.z * d_dot_duu * (1.0 - cos_angle),
            );

            let dxduv = d.cross(&d2t.duv);
            let d_dot_duv = d.dot(&d2t.duv);
            d2t.duv = XYZ::new(
                d2t.duv.x * cos_angle + dxduv.x * sin_angle + d.x * d_dot_duv * (1.0 - cos_angle),
                d2t.duv.y * cos_angle + dxduv.y * sin_angle + d.y * d_dot_duv * (1.0 - cos_angle),
                d2t.duv.z * cos_angle + dxduv.z * sin_angle + d.z * d_dot_duv * (1.0 - cos_angle),
            );

            let dxdvv = d.cross(&d2t.dvv);
            let d_dot_dvv = d.dot(&d2t.dvv);
            d2t.dvv = XYZ::new(
                d2t.dvv.x * cos_angle + dxdvv.x * sin_angle + d.x * d_dot_dvv * (1.0 - cos_angle),
                d2t.dvv.y * cos_angle + dxdvv.y * sin_angle + d.y * d_dot_dvv * (1.0 - cos_angle),
                d2t.dvv.z * cos_angle + dxdvv.z * sin_angle + d.z * d_dot_dvv * (1.0 - cos_angle),
            );
        }

        let cos_normales = normale.dot(&normale_s);
        if cos_normales.abs() < COSMIN {
            let du = d1s.du * -1.0;
            let dv = d1s.dv * -1.0;
            result.lsc[0] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, du, 1, 0),
                normale,
            );
            result.lsc[1] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, dv, 0, 1),
                normale,
            );
            result.nb_ls_constraints = 2;
            return result;
        }

        let invcos = 1.0 / cos_normales;
        let du = normale_s * (-(normale.dot(&d1s.du) * invcos));
        let dv = normale_s * (-(normale.dot(&d1s.dv) * invcos));

        result.ppc[0] = PlatePinpointConstraint::with_values(point2d, du, 1, 0);
        result.ppc[1] = PlatePinpointConstraint::with_values(point2d, dv, 0, 1);
        result.nb_pp_constraints = 2;

        // G2 constraints: solve 2x2 linear system
        let su = d1s.du + du;
        let sv = d1s.dv + dv;

        let m00 = su.dot(&d1t.du);
        let m01 = su.dot(&d1t.dv);
        let m10 = sv.dot(&d1t.du);
        let m11 = sv.dot(&d1t.dv);

        let det = m00 * m11 - m01 * m10;
        if det.abs() < 1e-15 {
            return result;
        }

        let inv_det = 1.0 / det;

        // Solve for [a, b]^T from [m00 m01; m10 m11] * [a; b] = [su·su; su·sv]
        let rhs0 = su.dot(&su);
        let rhs1 = su.dot(&sv);
        let a = inv_det * (m11 * rhs0 - m01 * rhs1);
        let b = inv_det * (-m10 * rhs0 + m00 * rhs1);

        // Solve for [c, d]^T from [m00 m01; m10 m11] * [c; d] = [sv·su; sv·sv]
        let rhs0 = sv.dot(&su);
        let rhs1 = sv.dot(&sv);
        let c = inv_det * (m11 * rhs0 - m01 * rhs1);
        let d = inv_det * (-m10 * rhs0 + m00 * rhs1);

        let suu = d2t.duu * (a * a) + d2t.duv * (2.0 * a * b) + d2t.dvv * (b * b);
        let suv = d2t.duu * (a * c) + d2t.duv * (a * d + b * c) + d2t.dvv * (b * d);
        let svv = d2t.duu * (c * c) + d2t.duv * (2.0 * c * d) + d2t.dvv * (d * d);

        let duu = (suu + d2s.duu * -1.0) * incremental_load;
        let duv = (suv + d2s.duv * -1.0) * incremental_load;
        let dvv = (svv + d2s.dvv * -1.0) * incremental_load;

        result.lsc[0] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, duu, 2, 0),
            normale,
        );
        result.lsc[1] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, duv, 1, 1),
            normale,
        );
        result.lsc[2] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, dvv, 0, 2),
            normale,
        );
        result.nb_ls_constraints = 3;

        result
    }

    /// G1 + G2 + G3 constraint
    pub fn new_g1_g2_g3(
        point2d: XY,
        d1s: &PlateD1,
        d1t0: &PlateD1,
        d2s: &PlateD2,
        d2t0: &PlateD2,
        d3s: &PlateD3,
        d3t0: &PlateD3,
        incremental_load: f64,
        orientation: i32,
    ) -> Self {
        let mut result = PlateFreeGtoCConstraint {
            pnt2d: point2d,
            nb_pp_constraints: 0,
            nb_ls_constraints: 0,
            ppc: [PlatePinpointConstraint::new(); 5],
            lsc: [
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
                PlateLinearScalarConstraint::new(),
            ],
        };

        let mut d1t = *d1t0;
        let mut d2t = *d2t0;
        let mut d3t = *d3t0;

        let mut normale = d1t.du.cross(&d1t.dv);
        if normale.modulus() < NORMIN {
            return result;
        }
        normale.normalize();

        let mut normale_s = d1s.du.cross(&d1s.dv);
        if normale_s.modulus() < NORMIN {
            if (incremental_load - 1.0).abs() > 1e-14 {
                return result;
            }
            let du = d1s.du * -1.0;
            let dv = d1s.dv * -1.0;
            result.lsc[0] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, du, 1, 0),
                normale,
            );
            result.lsc[1] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, dv, 0, 1),
                normale,
            );
            result.nb_ls_constraints = 2;
            return result;
        }
        normale_s.normalize();

        if (incremental_load - 1.0).abs() > 1e-14 {
            let n0 = normale_s;
            let mut n1 = normale;
            if orientation != 0 {
                n1 = n1 * (orientation as f64);
            }
            let mut c = n0.dot(&n1);
            if orientation == 0 {
                if c < 0.0 {
                    c = -c;
                    n1 = -n1;
                }
            }
            let s = n0.cross_magnitude(&n1);
            if s < 1e-2 && c < 0.0 {
                return result;
            }
            let angle = s.atan2(c);
            let mut d = n0.cross(&n1);
            d.normalize();

            let rot_angle = angle * (incremental_load - 1.0);
            let cos_angle = rot_angle.cos();
            let sin_angle = rot_angle.sin();

            // Apply rotation
            let apply_rot = |v: XYZ| {
                let dxv = d.cross(&v);
                let d_dot_v = d.dot(&v);
                XYZ::new(
                    v.x * cos_angle + dxv.x * sin_angle + d.x * d_dot_v * (1.0 - cos_angle),
                    v.y * cos_angle + dxv.y * sin_angle + d.y * d_dot_v * (1.0 - cos_angle),
                    v.z * cos_angle + dxv.z * sin_angle + d.z * d_dot_v * (1.0 - cos_angle),
                )
            };

            normale = apply_rot(normale);
            d1t.du = apply_rot(d1t.du);
            d1t.dv = apply_rot(d1t.dv);
            d2t.duu = apply_rot(d2t.duu);
            d2t.duv = apply_rot(d2t.duv);
            d2t.dvv = apply_rot(d2t.dvv);
            d3t.duuu = apply_rot(d3t.duuu);
            d3t.duuv = apply_rot(d3t.duuv);
            d3t.duvv = apply_rot(d3t.duvv);
            d3t.dvvv = apply_rot(d3t.dvvv);
        }

        let cos_normales = normale.dot(&normale_s);
        if cos_normales.abs() < COSMIN {
            let du = d1s.du * -1.0;
            let dv = d1s.dv * -1.0;
            result.lsc[0] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, du, 1, 0),
                normale,
            );
            result.lsc[1] = PlateLinearScalarConstraint::from_single(
                PlatePinpointConstraint::with_values(point2d, dv, 0, 1),
                normale,
            );
            result.nb_ls_constraints = 2;
            return result;
        }

        let invcos = 1.0 / cos_normales;
        let du = normale_s * (-(normale.dot(&d1s.du) * invcos));
        let dv = normale_s * (-(normale.dot(&d1s.dv) * invcos));

        result.ppc[0] = PlatePinpointConstraint::with_values(point2d, du, 1, 0);
        result.ppc[1] = PlatePinpointConstraint::with_values(point2d, dv, 0, 1);
        result.nb_pp_constraints = 2;

        // G2 constraints: solve 2x2 linear system
        let su = d1s.du + du;
        let sv = d1s.dv + dv;

        let m00 = su.dot(&d1t.du);
        let m01 = su.dot(&d1t.dv);
        let m10 = sv.dot(&d1t.du);
        let m11 = sv.dot(&d1t.dv);

        let det = m00 * m11 - m01 * m10;
        if det.abs() < 1e-15 {
            return result;
        }

        let inv_det = 1.0 / det;

        let rhs0 = su.dot(&su);
        let rhs1 = su.dot(&sv);
        let a = inv_det * (m11 * rhs0 - m01 * rhs1);
        let b = inv_det * (-m10 * rhs0 + m00 * rhs1);

        let rhs0 = sv.dot(&su);
        let rhs1 = sv.dot(&sv);
        let c = inv_det * (m11 * rhs0 - m01 * rhs1);
        let d = inv_det * (-m10 * rhs0 + m00 * rhs1);

        let suu = d2t.duu * (a * a) + d2t.duv * (2.0 * a * b) + d2t.dvv * (b * b);
        let suv = d2t.duu * (a * c) + d2t.duv * (a * d + b * c) + d2t.dvv * (b * d);
        let svv = d2t.duu * (c * c) + d2t.duv * (2.0 * c * d) + d2t.dvv * (d * d);

        let duu = normale_s * (normale.dot(&(suu + d2s.duu * -1.0)) * invcos);
        let duv = normale_s * (normale.dot(&(suv + d2s.duv * -1.0)) * invcos);
        let dvv = normale_s * (normale.dot(&(svv + d2s.dvv * -1.0)) * invcos);

        result.ppc[2] = PlatePinpointConstraint::with_values(point2d, duu, 2, 0);
        result.ppc[3] = PlatePinpointConstraint::with_values(point2d, duv, 1, 1);
        result.ppc[4] = PlatePinpointConstraint::with_values(point2d, dvv, 0, 2);
        result.nb_pp_constraints = 5;

        // G3 constraints
        let b0uu = solve_2x2(
            m00,
            m01,
            m10,
            m11,
            (d2s.duu + duu + suu * -1.0).dot(&su),
            (d2s.duu + duu + suu * -1.0).dot(&sv),
        )
        .0;
        let b1uu = solve_2x2(
            m00,
            m01,
            m10,
            m11,
            (d2s.duu + duu + suu * -1.0).dot(&su),
            (d2s.duu + duu + suu * -1.0).dot(&sv),
        )
        .1;

        let b0uv = solve_2x2(
            m00,
            m01,
            m10,
            m11,
            (d2s.duv + duv + suv * -1.0).dot(&su),
            (d2s.duv + duv + suv * -1.0).dot(&sv),
        )
        .0;
        let b1uv = solve_2x2(
            m00,
            m01,
            m10,
            m11,
            (d2s.duv + duv + suv * -1.0).dot(&su),
            (d2s.duv + duv + suv * -1.0).dot(&sv),
        )
        .1;

        let b0vv = solve_2x2(
            m00,
            m01,
            m10,
            m11,
            (d2s.dvv + dvv + svv * -1.0).dot(&su),
            (d2s.dvv + dvv + svv * -1.0).dot(&sv),
        )
        .0;
        let b1vv = solve_2x2(
            m00,
            m01,
            m10,
            m11,
            (d2s.dvv + dvv + svv * -1.0).dot(&su),
            (d2s.dvv + dvv + svv * -1.0).dot(&sv),
        )
        .1;

        let suuu = d3t.duuu * (a * a * a)
            + d3t.duuv * (3.0 * a * a * b)
            + d3t.duvv * (3.0 * a * b * b)
            + d3t.dvvv * (b * b * b);
        let suuv = d3t.duuu * (a * a * c)
            + d3t.duuv * (a * a * d + 2.0 * a * b * c)
            + d3t.duvv * (b * b * c + 2.0 * a * b * d)
            + d3t.dvvv * (b * b * d);
        let suvv = d3t.duuu * (a * c * c)
            + d3t.duuv * (b * c * c + 2.0 * a * c * d)
            + d3t.duvv * (a * d * d + 2.0 * b * c * d)
            + d3t.dvvv * (b * d * d);
        let svvv = d3t.duuu * (c * c * c)
            + d3t.duuv * (3.0 * c * c * d)
            + d3t.duvv * (3.0 * c * d * d)
            + d3t.dvvv * (d * d * d);

        let a0u = a;
        let a1u = b;
        let a0v = c;
        let a1v = d;

        let duuu = (suuu
            + d2t.duu * (3.0 * a0u * b0uu)
            + d2t.duv * (3.0 * (a0u * b1uu + a1u * b0uu))
            + d2t.dvv * (3.0 * a1u * b1uu)
            + d3s.duuu * -1.0)
            * incremental_load;
        let duuv = (suuv
            + d2t.duu * (2.0 * a0u * b0uv + a0v * b0uu)
            + d2t.duv * (2.0 * (a0u * b1uv + a1u * b0uv) + a0v * b1uu + a1v * b0uu)
            + d2t.dvv * (2.0 * a1u * b1uv + a1v * b1uu)
            + d3s.duuv * -1.0)
            * incremental_load;
        let duvv = (suvv
            + d2t.duu * (a0u * b0vv + 2.0 * a0v * b0uv)
            + d2t.duv * (2.0 * (a0v * b1uv + a1v * b0uv) + a0u * b1vv + a1u * b0vv)
            + d2t.dvv * (2.0 * a1v * b1uv + a1u * b1vv)
            + d3s.duvv * -1.0)
            * incremental_load;
        let dvvv = (svvv
            + d2t.duu * (3.0 * a0v * b0vv)
            + d2t.duv * (3.0 * (a0v * b1vv + a1v * b0vv))
            + d2t.dvv * (3.0 * a1v * b1vv)
            + d3s.dvvv * -1.0)
            * incremental_load;

        result.lsc[0] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, duuu, 3, 0),
            normale,
        );
        result.lsc[1] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, duuv, 2, 1),
            normale,
        );
        result.lsc[2] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, duvv, 1, 2),
            normale,
        );
        result.lsc[3] = PlateLinearScalarConstraint::from_single(
            PlatePinpointConstraint::with_values(point2d, dvvv, 0, 3),
            normale,
        );
        result.nb_ls_constraints = 4;

        result
    }

    pub fn nb_ppc(&self) -> usize {
        self.nb_pp_constraints
    }

    pub fn get_ppc(&self, index: usize) -> Option<&PlatePinpointConstraint> {
        if index < self.nb_pp_constraints {
            Some(&self.ppc[index])
        } else {
            None
        }
    }

    pub fn nb_lsc(&self) -> usize {
        self.nb_ls_constraints
    }

    pub fn lsc(&self, index: usize) -> Option<&PlateLinearScalarConstraint> {
        if index < self.nb_ls_constraints {
            Some(&self.lsc[index])
        } else {
            None
        }
    }
}

fn solve_2x2(m00: f64, m01: f64, m10: f64, m11: f64, rhs0: f64, rhs1: f64) -> (f64, f64) {
    let det = m00 * m11 - m01 * m10;
    if det.abs() < 1e-15 {
        return (0.0, 0.0);
    }
    let inv_det = 1.0 / det;
    let x = inv_det * (m11 * rhs0 - m01 * rhs1);
    let y = inv_det * (-m10 * rhs0 + m00 * rhs1);
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g1_constraint() {
        let pt = XY::new(0.0, 0.0);
        let d1s = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d1t = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let c = PlateFreeGtoCConstraint::new_g1(pt, &d1s, &d1t, 1.0, 0);
        assert_eq!(c.nb_lsc(), 2);
    }

    #[test]
    fn test_g1_g2_constraint() {
        let pt = XY::new(0.0, 0.0);
        let d1s = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d1t = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d2s = PlateD2::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let d2t = PlateD2::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let c = PlateFreeGtoCConstraint::new_g1_g2(pt, &d1s, &d1t, &d2s, &d2t, 1.0, 0);
        assert!(c.nb_lsc() >= 2);
    }

    #[test]
    fn test_g1_g2_g3_constraint() {
        let pt = XY::new(0.0, 0.0);
        let d1s = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d1t = PlateD1::new(XYZ::new(1.0, 0.0, 0.0), XYZ::new(0.0, 1.0, 0.0));
        let d2s = PlateD2::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let d2t = PlateD2::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let d3s = PlateD3::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let d3t = PlateD3::new(
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
            XYZ::new(0.0, 0.0, 0.0),
        );
        let c =
            PlateFreeGtoCConstraint::new_g1_g2_g3(pt, &d1s, &d1t, &d2s, &d2t, &d3s, &d3t, 1.0, 0);
        assert!(c.nb_lsc() >= 2);
    }
}
