// FILE: tests/occt_geom_local_props.rs
//! Integration tests for `geom_local_props` — data-only local property types
//! for curves and surfaces.
//!
//! Covers `SurfaceLocalProps` and `CurveLocalProps` through the public API only.
//! Numeric expectations are derived from well-known geometric identities
//! (circle curvature, sphere Gaussian curvature, cylinder shape index, etc.).

extern crate ironstream;
use ironstream::geom_local_props::*;

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn near(a: f64, b: f64, tol: f64, label: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{label}: got {a:.15}, expected {b:.15} (tol {tol})"
    );
}

fn vec3_norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

// ===========================================================================
// SurfaceLocalProps — construction and field access
// ===========================================================================

/// A freshly constructed instance has sensible defaults and marks both the
/// normal and curvatures as undefined.
#[test]
fn surface_local_props_new_defaults() {
    let props = SurfaceLocalProps::new(1.5, 2.5);
    assert_eq!(props.u, 1.5);
    assert_eq!(props.v, 2.5);
    assert!(!props.is_normal_defined(), "normal should be undefined initially");
    assert!(!props.is_curvature_defined(), "curvature should be undefined initially");
}

/// `set_point` stores the given coordinates verbatim.
#[test]
fn surface_local_props_set_point_round_trip() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_point([3.0, -1.0, 7.0]);
    assert_eq!(props.point(), [3.0, -1.0, 7.0]);
}

/// `set_normal` stores the normal and marks it defined.
#[test]
fn surface_local_props_set_normal_marks_defined() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    assert!(!props.is_normal_defined());
    props.set_normal([0.0, 0.0, 1.0]);
    assert!(props.is_normal_defined());
    assert_eq!(props.normal(), [0.0, 0.0, 1.0]);
}

/// `set_curvatures` marks the curvature as defined and reorders min/max.
#[test]
fn surface_local_props_set_curvatures_reorders() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    assert!(!props.is_curvature_defined());
    // Intentionally pass larger value first.
    props.set_curvatures(4.0, 2.0);
    assert!(props.is_curvature_defined());
    assert_eq!(props.min_curvature(), 2.0);
    assert_eq!(props.max_curvature(), 4.0);
}

/// `set_curvatures` with equal values leaves both equal.
#[test]
fn surface_local_props_set_curvatures_equal() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(1.0, 1.0);
    assert_eq!(props.min_curvature(), 1.0);
    assert_eq!(props.max_curvature(), 1.0);
}

// ===========================================================================
// SurfaceLocalProps — derived curvature quantities
// ===========================================================================

/// Plane: both principal curvatures zero -> K = 0, H = 0.
#[test]
fn surface_local_props_plane_zero_curvatures() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_normal([0.0, 0.0, 1.0]);
    props.set_curvatures(0.0, 0.0);
    near(props.gaussian_curvature(), 0.0, 1e-14, "plane K = 0");
    near(props.mean_curvature(), 0.0, 1e-14, "plane H = 0");
}

/// Unit sphere: kmin = kmax = 1 -> K = 1, H = 1.
#[test]
fn surface_local_props_unit_sphere_curvatures() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_normal([0.0, 0.0, 1.0]);
    props.set_curvatures(1.0, 1.0);
    near(props.gaussian_curvature(), 1.0, 1e-14, "unit sphere K = 1");
    near(props.mean_curvature(), 1.0, 1e-14, "unit sphere H = 1");
}

/// Sphere of radius r: kmin = kmax = 1/r -> K = 1/r^2.
#[test]
fn surface_local_props_sphere_r_gaussian_curvature() {
    for r in [1.0_f64, 2.0, 5.0, 10.0] {
        let mut props = SurfaceLocalProps::new(0.0, 0.0);
        let k = 1.0 / r;
        props.set_curvatures(k, k);
        near(
            props.gaussian_curvature(),
            1.0 / (r * r),
            1e-13,
            &format!("sphere r={r} K = 1/r^2"),
        );
    }
}

/// Unit cylinder: kmax = 1, kmin = 0 -> K = 0, H = 0.5.
#[test]
fn surface_local_props_unit_cylinder_curvatures() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_normal([1.0, 0.0, 0.0]);
    props.set_curvatures(0.0, 1.0);
    near(props.gaussian_curvature(), 0.0, 1e-14, "cylinder K = 0");
    near(props.mean_curvature(), 0.5, 1e-14, "cylinder H = 0.5");
}

/// Saddle: kmax = 1, kmin = -1 -> K = -1, H = 0.
#[test]
fn surface_local_props_saddle_curvatures() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(-1.0, 1.0);
    near(props.gaussian_curvature(), -1.0, 1e-14, "saddle K = -1");
    near(props.mean_curvature(), 0.0, 1e-14, "saddle H = 0");
}

/// Mean curvature formula: H = (kmax + kmin) / 2.
#[test]
fn surface_local_props_mean_curvature_formula() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(1.0, 3.0);
    near(props.mean_curvature(), 2.0, 1e-14, "H = (1+3)/2 = 2");
}

/// Gaussian curvature formula: K = kmax * kmin.
#[test]
fn surface_local_props_gaussian_curvature_formula() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(2.0, 4.0);
    near(props.gaussian_curvature(), 8.0, 1e-14, "K = 2*4 = 8");
}

// ===========================================================================
// SurfaceLocalProps — shape index
// ===========================================================================

/// Sphere (kmax == kmin): shape index = 0.
#[test]
fn surface_local_props_shape_index_sphere() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(1.0, 1.0);
    near(props.shape_index(), 0.0, 1e-14, "sphere shape index = 0");
}

/// Flat surface (kmax == kmin == 0): shape index = 0.
#[test]
fn surface_local_props_shape_index_flat() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(0.0, 0.0);
    near(props.shape_index(), 0.0, 1e-14, "flat shape index = 0");
}

/// Cylinder (kmax = 1, kmin = 0):
/// shape index = (2/pi) * atan((1+0)/(1-0)) = (2/pi) * atan(1) = (2/pi) * (pi/4) = 0.5.
#[test]
fn surface_local_props_shape_index_cylinder() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(0.0, 1.0);
    near(props.shape_index(), 0.5, 1e-12, "cylinder shape index = 0.5");
}

/// Inverted cylinder (kmax = 0, kmin = -1):
/// shape index = (2/pi) * atan((-1+0)/(0-(-1))) = (2/pi) * atan(-1) = -0.5.
#[test]
fn surface_local_props_shape_index_inverted_cylinder() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(-1.0, 0.0);
    near(props.shape_index(), -0.5, 1e-12, "inverted cylinder shape index = -0.5");
}

/// Saddle (kmax = 1, kmin = -1): sum = 0 -> atan(0) = 0 -> shape index = 0.
#[test]
fn surface_local_props_shape_index_saddle() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(-1.0, 1.0);
    near(props.shape_index(), 0.0, 1e-14, "saddle shape index = 0");
}

/// Generic case: verify the exact formula value.
#[test]
fn surface_local_props_shape_index_generic() {
    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_curvatures(1.0, 3.0);
    // (2/pi) * atan((3+1)/(3-1)) = (2/pi) * atan(2).
    let expected = (2.0 / PI) * (2.0_f64.atan());
    near(props.shape_index(), expected, 1e-12, "generic shape index");
}

/// Shape index is in (−1, 1] for all non-degenerate configurations.
#[test]
fn surface_local_props_shape_index_bounds() {
    let cases: &[(f64, f64)] = &[
        (-5.0, -1.0),
        (-1.0, 0.0),
        (0.0, 1.0),
        (1.0, 2.0),
        (1.0, 100.0),
        (-100.0, -1.0),
    ];
    for &(kmin, kmax) in cases {
        let mut props = SurfaceLocalProps::new(0.0, 0.0);
        props.set_curvatures(kmin, kmax);
        let si = props.shape_index();
        assert!(
            si > -1.0 - 1e-12 && si <= 1.0 + 1e-12,
            "shape index {si} out of (-1,1] for kmin={kmin} kmax={kmax}"
        );
    }
}

// ===========================================================================
// CurveLocalProps — construction and field access
// ===========================================================================

/// Freshly constructed instance defaults to zero curvature and torsion.
#[test]
fn curve_local_props_new_defaults() {
    let props = CurveLocalProps::new(2.71828);
    assert_eq!(props.parameter, 2.71828);
    near(props.curvature(), 0.0, 1e-14, "default curvature = 0");
    near(props.torsion(), 0.0, 1e-14, "default torsion = 0");
}

/// `set_point` stores coordinates verbatim.
#[test]
fn curve_local_props_set_point_round_trip() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_point([1.0, -2.0, 3.5]);
    assert_eq!(props.point(), [1.0, -2.0, 3.5]);
}

/// `set_torsion` / `torsion` round-trip.
#[test]
fn curve_local_props_torsion_round_trip() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_torsion(0.314);
    near(props.torsion(), 0.314, 1e-14, "torsion round-trip");
}

// ===========================================================================
// CurveLocalProps — curvature computed by set_derivatives
// ===========================================================================

/// A line (D2 = 0) has zero curvature.
#[test]
fn curve_local_props_line_zero_curvature() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    near(props.curvature(), 0.0, 1e-14, "line curvature = 0");
}

/// Unit circle at t = 0: D1 = (0, 1, 0), D2 = (-1, 0, 0), curvature = 1.
#[test]
fn curve_local_props_unit_circle_curvature() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    near(props.curvature(), 1.0, 1e-14, "unit circle curvature = 1");
}

/// Circle of radius r: curvature = 1/r.
#[test]
fn curve_local_props_circle_radius_r_curvature() {
    for r in [1.0_f64, 2.0, 5.0, 10.0] {
        let mut props = CurveLocalProps::new(0.0);
        // D1 = (0, r, 0), D2 = (-r, 0, 0) at t = 0.
        props.set_derivatives([0.0, r, 0.0], [-r, 0.0, 0.0]);
        near(
            props.curvature(),
            1.0 / r,
            1e-13,
            &format!("circle r={r} curvature = 1/r"),
        );
    }
}

/// After `set_derivatives` with a new D1/D2, the curvature is recomputed.
#[test]
fn curve_local_props_curvature_updates_on_set_derivatives() {
    let mut props = CurveLocalProps::new(0.0);
    // First call: unit circle, k = 1.
    props.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    near(props.curvature(), 1.0, 1e-14, "first curvature = 1");
    // Second call: circle r = 2, k = 0.5.
    props.set_derivatives([0.0, 2.0, 0.0], [-2.0, 0.0, 0.0]);
    near(props.curvature(), 0.5, 1e-14, "updated curvature = 0.5");
}

// ===========================================================================
// CurveLocalProps — tangent
// ===========================================================================

/// Tangent is a unit vector for a non-degenerate D1.
#[test]
fn curve_local_props_tangent_is_unit() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([3.0, 4.0, 0.0], [0.0, 0.0, 0.0]);
    let t = props.tangent();
    near(vec3_norm(t), 1.0, 1e-14, "|tangent| = 1");
}

/// Tangent direction matches normalized D1.
#[test]
fn curve_local_props_tangent_direction() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([3.0, 4.0, 0.0], [0.0, 0.0, 0.0]);
    let t = props.tangent();
    near(t[0], 3.0 / 5.0, 1e-14, "tx = 3/5");
    near(t[1], 4.0 / 5.0, 1e-14, "ty = 4/5");
    near(t[2], 0.0, 1e-14, "tz = 0");
}

/// Tangent for unit circle at t=0 is (0, 1, 0).
#[test]
fn curve_local_props_unit_circle_tangent() {
    let mut props = CurveLocalProps::new(0.0);
    // D1 = (0, 1, 0) for a unit circle at t=0.
    props.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    let t = props.tangent();
    near(t[0], 0.0, 1e-14, "tx = 0");
    near(t[1], 1.0, 1e-14, "ty = 1");
    near(t[2], 0.0, 1e-14, "tz = 0");
}

/// Zero D1 -> tangent returns zero vector (degenerate).
#[test]
fn curve_local_props_tangent_zero_d1() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let t = props.tangent();
    near(vec3_norm(t), 0.0, 1e-14, "zero D1 -> zero tangent");
}

// ===========================================================================
// CurveLocalProps — normal
// ===========================================================================

/// Unit circle at t=0: normal points toward center, i.e. (-1, 0, 0).
#[test]
fn curve_local_props_unit_circle_normal() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    let n = props.normal();
    near(n[0], -1.0, 1e-14, "nx = -1");
    near(n[1], 0.0, 1e-14, "ny = 0");
    near(n[2], 0.0, 1e-14, "nz = 0");
}

/// Normal is a unit vector when curvature is non-zero.
#[test]
fn curve_local_props_normal_is_unit() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([0.0, 2.0, 0.0], [-2.0, 0.0, 0.0]);
    let n = props.normal();
    near(vec3_norm(n), 1.0, 1e-14, "|normal| = 1");
}

/// Normal is orthogonal to tangent.
#[test]
fn curve_local_props_normal_orthogonal_to_tangent() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    let t = props.tangent();
    let n = props.normal();
    let dot = t[0] * n[0] + t[1] * n[1] + t[2] * n[2];
    near(dot, 0.0, 1e-14, "T · N = 0");
}

/// Line (D2 = 0): normal returns zero vector (undefined).
#[test]
fn curve_local_props_line_normal_is_zero() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    let n = props.normal();
    near(vec3_norm(n), 0.0, 1e-14, "line normal = zero vector");
}

/// Normal for a 3D helix-like input where D2 has a component along D1.
#[test]
fn curve_local_props_normal_removes_tangential_component() {
    // D1 = (1, 0, 0), D2 = (1, 1, 0).
    // T = (1, 0, 0). D2 · T = 1. proj = D2 - 1*(1,0,0) = (0, 1, 0).
    // N = (0, 1, 0).
    let mut props = CurveLocalProps::new(0.0);
    props.set_derivatives([1.0, 0.0, 0.0], [1.0, 1.0, 0.0]);
    let n = props.normal();
    near(n[0], 0.0, 1e-14, "nx = 0");
    near(n[1], 1.0, 1e-14, "ny = 1");
    near(n[2], 0.0, 1e-14, "nz = 0");
}

// ===========================================================================
// SurfaceLocalProps — independence of u, v fields
// ===========================================================================

/// Parameters u and v can be updated by constructing a new struct or mutating.
#[test]
fn surface_local_props_different_params() {
    for (u, v) in [(0.0_f64, 0.0_f64), (1.0, 2.0), (-3.14, 6.28)] {
        let props = SurfaceLocalProps::new(u, v);
        assert_eq!(props.u, u);
        assert_eq!(props.v, v);
    }
}

// ===========================================================================
// Combined scenario: store and query all fields
// ===========================================================================

/// Full round-trip: store point, normal, curvatures, then query all derived
/// quantities for a sphere of radius 2.
#[test]
fn surface_local_props_sphere_r2_full_round_trip() {
    let r = 2.0_f64;
    let k = 1.0 / r; // = 0.5

    let mut props = SurfaceLocalProps::new(0.0, 0.0);
    props.set_point([r, 0.0, 0.0]);
    props.set_normal([1.0, 0.0, 0.0]);
    props.set_curvatures(k, k);

    assert!(props.is_normal_defined());
    assert!(props.is_curvature_defined());
    assert_eq!(props.point(), [r, 0.0, 0.0]);
    assert_eq!(props.normal(), [1.0, 0.0, 0.0]);
    near(props.min_curvature(), 0.5, 1e-14, "kmin = 0.5");
    near(props.max_curvature(), 0.5, 1e-14, "kmax = 0.5");
    near(props.gaussian_curvature(), 0.25, 1e-14, "K = 1/r^2 = 0.25");
    near(props.mean_curvature(), 0.5, 1e-14, "H = 1/r = 0.5");
    near(props.shape_index(), 0.0, 1e-14, "sphere shape index = 0");
}

/// Full round-trip for a curve: unit circle at t = 0.
#[test]
fn curve_local_props_unit_circle_full_round_trip() {
    let mut props = CurveLocalProps::new(0.0);
    props.set_point([1.0, 0.0, 0.0]);
    props.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    props.set_torsion(0.0); // planar curve

    assert_eq!(props.point(), [1.0, 0.0, 0.0]);
    near(props.curvature(), 1.0, 1e-14, "k = 1");
    near(props.torsion(), 0.0, 1e-14, "torsion = 0");

    let t = props.tangent();
    near(t[0], 0.0, 1e-14, "tx = 0");
    near(t[1], 1.0, 1e-14, "ty = 1");
    near(vec3_norm(t), 1.0, 1e-14, "|T| = 1");

    let n = props.normal();
    near(n[0], -1.0, 1e-14, "nx = -1");
    near(vec3_norm(n), 1.0, 1e-14, "|N| = 1");

    // T and N orthogonal.
    let dot = t[0] * n[0] + t[1] * n[1] + t[2] * n[2];
    near(dot, 0.0, 1e-14, "T·N = 0");
}
