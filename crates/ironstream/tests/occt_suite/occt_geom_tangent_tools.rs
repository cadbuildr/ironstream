// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_tangent_tools.rs
//! Ported OpenCascade unit tests — Frenet / tangent-frame tools.
//!
//! Covers `FrenetFrame`, `frenet_line`, and `frenet_circle` from
//! [`ironstream::geom_tangent_tools`], mirroring the checks that OCCT performs
//! inside `LProp_CLProps` and `BRep_CurveRepresentation` tests.

extern crate ironstream;
use ironstream::geom_tangent_tools::*;

const TOL: f64 = 1e-12;

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b}  (diff {diff}, tol {tol})",
        diff = (a - b).abs()
    );
}

fn near3(a: [f64; 3], b: [f64; 3], tol: f64) {
    for i in 0..3 {
        near(a[i], b[i], tol);
    }
}

fn norm3(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

// ---------------------------------------------------------------------------
// FrenetFrame — builder / getter round-trips
// ---------------------------------------------------------------------------

// TEST(FrenetFrame, NewDefaultsZero)
#[test]
fn frenet_frame_new_defaults_zero() {
    let f = FrenetFrame::new(1.5);
    near(f.parameter(), 1.5, TOL);
    near3(f.point(), [0.0, 0.0, 0.0], TOL);
    near3(f.tangent(), [0.0, 0.0, 0.0], TOL);
    near3(f.normal(), [0.0, 0.0, 0.0], TOL);
    near(f.curvature(), 0.0, TOL);
    near(f.torsion(), 0.0, TOL);
}

// TEST(FrenetFrame, SetPoint)
#[test]
fn frenet_frame_set_point() {
    let mut f = FrenetFrame::new(0.0);
    f.set_point([1.0, 2.0, 3.0]);
    near3(f.point(), [1.0, 2.0, 3.0], TOL);
}

// TEST(FrenetFrame, SetTangentNormalises)
#[test]
fn frenet_frame_set_tangent_normalises() {
    let mut f = FrenetFrame::new(0.0);
    // Provide an un-normalised tangent; the setter must normalise it.
    f.set_tangent([3.0, 0.0, 0.0]);
    near3(f.tangent(), [1.0, 0.0, 0.0], TOL);
    near(norm3(f.tangent()), 1.0, TOL);
}

// TEST(FrenetFrame, SetNormalNormalises)
#[test]
fn frenet_frame_set_normal_normalises() {
    let mut f = FrenetFrame::new(0.0);
    f.set_normal([0.0, 0.0, 5.0]);
    near3(f.normal(), [0.0, 0.0, 1.0], TOL);
    near(norm3(f.normal()), 1.0, TOL);
}

// TEST(FrenetFrame, SetCurvatureAndTorsion)
#[test]
fn frenet_frame_set_curvature_and_torsion() {
    let mut f = FrenetFrame::new(0.0);
    f.set_curvature(2.5);
    f.set_torsion(-0.3);
    near(f.curvature(), 2.5, TOL);
    near(f.torsion(), -0.3, TOL);
}

// TEST(FrenetFrame, BinormalIsCrossProduct)
#[test]
fn frenet_frame_binormal_is_cross_product() {
    let mut f = FrenetFrame::new(0.0);
    // T = (1,0,0), N = (0,1,0)  =>  B = T × N = (0,0,1)
    f.set_tangent([1.0, 0.0, 0.0]);
    f.set_normal([0.0, 1.0, 0.0]);
    near3(f.binormal(), [0.0, 0.0, 1.0], TOL);
}

// TEST(FrenetFrame, BinormalOrthogonalToTangentAndNormal)
#[test]
fn frenet_frame_binormal_orthogonal() {
    let mut f = FrenetFrame::new(0.0);
    // Use an oblique but orthonormal T/N pair.
    let sqrt2_inv = 1.0_f64 / 2.0_f64.sqrt();
    f.set_tangent([sqrt2_inv, sqrt2_inv, 0.0]);
    f.set_normal([-sqrt2_inv, sqrt2_inv, 0.0]);
    let b = f.binormal();
    // B · T == 0
    let dot_bt = b[0] * f.tangent()[0] + b[1] * f.tangent()[1] + b[2] * f.tangent()[2];
    near(dot_bt, 0.0, TOL);
    // B · N == 0
    let dot_bn = b[0] * f.normal()[0] + b[1] * f.normal()[1] + b[2] * f.normal()[2];
    near(dot_bn, 0.0, TOL);
}

// TEST(FrenetFrame, SetZeroTangentKept)
#[test]
fn frenet_frame_set_zero_tangent_kept() {
    // Degenerate case: a zero vector in should produce a zero vector out.
    let mut f = FrenetFrame::new(0.0);
    f.set_tangent([0.0, 0.0, 0.0]);
    near3(f.tangent(), [0.0, 0.0, 0.0], TOL);
}

// ---------------------------------------------------------------------------
// frenet_line
// ---------------------------------------------------------------------------

// TEST(FrenetLine, PointOnLine)
#[test]
fn frenet_line_point_on_line() {
    // Line along X from origin, t = 3.
    let f = frenet_line(3.0, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    near3(f.point(), [3.0, 0.0, 0.0], TOL);
}

// TEST(FrenetLine, PointWithOffset)
#[test]
fn frenet_line_point_with_offset() {
    // Line along Z starting at (1, 2, 0), t = 5.
    let f = frenet_line(5.0, [0.0, 0.0, 1.0], [1.0, 2.0, 0.0]);
    near3(f.point(), [1.0, 2.0, 5.0], TOL);
}

// TEST(FrenetLine, TangentIsUnitDirection)
#[test]
fn frenet_line_tangent_is_unit_direction() {
    // Supply a non-unit direction; tangent must be normalised.
    let f = frenet_line(0.0, [0.0, 4.0, 0.0], [0.0, 0.0, 0.0]);
    near3(f.tangent(), [0.0, 1.0, 0.0], TOL);
    near(norm3(f.tangent()), 1.0, TOL);
}

// TEST(FrenetLine, CurvatureZero)
#[test]
fn frenet_line_curvature_zero() {
    let f = frenet_line(1.0, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    near(f.curvature(), 0.0, TOL);
}

// TEST(FrenetLine, TorsionZero)
#[test]
fn frenet_line_torsion_zero() {
    let f = frenet_line(1.0, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    near(f.torsion(), 0.0, TOL);
}

// TEST(FrenetLine, NormalUndefined)
#[test]
fn frenet_line_normal_undefined() {
    // For a straight line the normal is the zero vector (curvature = 0).
    let f = frenet_line(2.0, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    near3(f.normal(), [0.0, 0.0, 0.0], TOL);
}

// TEST(FrenetLine, ParameterStored)
#[test]
fn frenet_line_parameter_stored() {
    let f = frenet_line(7.0, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    near(f.parameter(), 7.0, TOL);
}

// TEST(FrenetLine, DiagonalDirection)
#[test]
fn frenet_line_diagonal_direction() {
    // Line along (1,1,0) from origin, t=2. Normalised direction=(1/√2,1/√2,0).
    let sqrt2_inv = 1.0_f64 / 2.0_f64.sqrt();
    let f = frenet_line(2.0, [1.0, 1.0, 0.0], [0.0, 0.0, 0.0]);
    near3(
        f.point(),
        [2.0 * sqrt2_inv, 2.0 * sqrt2_inv, 0.0],
        TOL,
    );
    near3(f.tangent(), [sqrt2_inv, sqrt2_inv, 0.0], TOL);
}

// ---------------------------------------------------------------------------
// frenet_circle
// ---------------------------------------------------------------------------

// TEST(FrenetCircle, PointAtT0)
#[test]
fn frenet_circle_point_at_t0() {
    // Unit circle centred at origin; at t=0 point = (1,0,0).
    let f = frenet_circle(0.0, 1.0, [0.0, 0.0, 0.0]);
    near3(f.point(), [1.0, 0.0, 0.0], TOL);
}

// TEST(FrenetCircle, PointAtHalfPi)
#[test]
fn frenet_circle_point_at_half_pi() {
    use std::f64::consts::FRAC_PI_2;
    let f = frenet_circle(FRAC_PI_2, 1.0, [0.0, 0.0, 0.0]);
    near3(f.point(), [0.0, 1.0, 0.0], TOL);
}

// TEST(FrenetCircle, PointWithRadius)
#[test]
fn frenet_circle_point_with_radius() {
    // Radius 3, centred at (1,2,0), at t=0: point = (4,2,0).
    let f = frenet_circle(0.0, 3.0, [1.0, 2.0, 0.0]);
    near3(f.point(), [4.0, 2.0, 0.0], TOL);
}

// TEST(FrenetCircle, TangentAtT0)
#[test]
fn frenet_circle_tangent_at_t0() {
    // At t=0: tangent = (-sin0, cos0, 0) = (0, 1, 0).
    let f = frenet_circle(0.0, 2.0, [0.0, 0.0, 0.0]);
    near3(f.tangent(), [0.0, 1.0, 0.0], TOL);
}

// TEST(FrenetCircle, TangentUnitLength)
#[test]
fn frenet_circle_tangent_unit_length() {
    use std::f64::consts::PI;
    for &t in &[0.0, PI / 4.0, PI / 2.0, PI, 3.0 * PI / 2.0] {
        let f = frenet_circle(t, 5.0, [0.0, 0.0, 0.0]);
        near(norm3(f.tangent()), 1.0, TOL);
    }
}

// TEST(FrenetCircle, NormalAtT0)
#[test]
fn frenet_circle_normal_at_t0() {
    // At t=0: N = (-cos0, -sin0, 0) = (-1, 0, 0) — points toward center.
    let f = frenet_circle(0.0, 1.0, [0.0, 0.0, 0.0]);
    near3(f.normal(), [-1.0, 0.0, 0.0], TOL);
}

// TEST(FrenetCircle, NormalUnitLength)
#[test]
fn frenet_circle_normal_unit_length() {
    use std::f64::consts::PI;
    for &t in &[0.0, PI / 6.0, PI / 3.0, PI, 5.0 * PI / 4.0] {
        let f = frenet_circle(t, 2.0, [0.0, 0.0, 0.0]);
        near(norm3(f.normal()), 1.0, TOL);
    }
}

// TEST(FrenetCircle, TangentOrthogonalToNormal)
#[test]
fn frenet_circle_tangent_orthogonal_to_normal() {
    use std::f64::consts::PI;
    for &t in &[0.0, 0.3, PI / 2.0, PI, 1.9 * PI] {
        let f = frenet_circle(t, 3.0, [0.0, 0.0, 0.0]);
        let dot = f.tangent()[0] * f.normal()[0]
            + f.tangent()[1] * f.normal()[1]
            + f.tangent()[2] * f.normal()[2];
        near(dot, 0.0, TOL);
    }
}

// TEST(FrenetCircle, BinormalIsOutOfPlane)
#[test]
fn frenet_circle_binormal_is_out_of_plane() {
    use std::f64::consts::PI;
    // For a circle in the XY plane B = (0,0,1) everywhere.
    for &t in &[0.0, PI / 4.0, PI / 2.0, PI] {
        let f = frenet_circle(t, 1.0, [0.0, 0.0, 0.0]);
        near3(f.binormal(), [0.0, 0.0, 1.0], TOL);
    }
}

// TEST(FrenetCircle, CurvatureIsInverseRadius)
#[test]
fn frenet_circle_curvature_is_inverse_radius() {
    let f = frenet_circle(0.0, 4.0, [0.0, 0.0, 0.0]);
    near(f.curvature(), 0.25, TOL);
}

// TEST(FrenetCircle, TorsionZero)
#[test]
fn frenet_circle_torsion_zero() {
    let f = frenet_circle(1.0, 2.0, [1.0, 1.0, 0.0]);
    near(f.torsion(), 0.0, TOL);
}

// TEST(FrenetCircle, ParameterStored)
#[test]
fn frenet_circle_parameter_stored() {
    use std::f64::consts::PI;
    let t = PI / 3.0;
    let f = frenet_circle(t, 1.0, [0.0, 0.0, 0.0]);
    near(f.parameter(), t, TOL);
}

// TEST(FrenetCircle, ZeroRadius)
#[test]
fn frenet_circle_zero_radius() {
    // Degenerate: radius 0 => point at center, curvature 0 (not 1/0).
    let f = frenet_circle(0.5, 0.0, [3.0, 4.0, 0.0]);
    near3(f.point(), [3.0, 4.0, 0.0], TOL);
    near(f.curvature(), 0.0, TOL);
}
