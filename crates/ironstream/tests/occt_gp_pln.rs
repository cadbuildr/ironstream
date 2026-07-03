// FILE: tests/occt_gp_pln.rs
//! Integration tests for `gp_pln::GpPln` — an analytic infinite plane,
//! mirroring OCCT's `gp_Pln` tests.

extern crate ironstream;

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_pln::GpPln;
use std::f64::consts::FRAC_PI_2;

const EPS: f64 = 1e-9;

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= EPS, "expected {a} ≈ {b}");
}

fn pnt_near(a: Pnt, b: Pnt) {
    assert!(
        (a.x - b.x).abs() <= EPS && (a.y - b.y).abs() <= EPS && (a.z - b.z).abs() <= EPS,
        "expected ({}, {}, {}) ≈ ({}, {}, {})",
        a.x, a.y, a.z, b.x, b.y, b.z
    );
}

// ─── Construction ────────────────────────────────────────────────────────────

#[test]
fn default_plane_is_xy_plane() {
    let p = GpPln::default();
    pnt_near(p.location(), Pnt::origin());
    pnt_near(p.normal(), Pnt::new(0.0, 0.0, 1.0));
}

#[test]
fn from_point_normal_stores_correct_origin_and_normal() {
    let p = GpPln::from_point_normal(Pnt::new(1.0, 2.0, 3.0), Pnt::new(0.0, 1.0, 0.0));
    pnt_near(p.location(), Pnt::new(1.0, 2.0, 3.0));
    // Normal should be unit +Y.
    let n = p.normal();
    near(n.x, 0.0);
    near(n.y, 1.0);
    near(n.z, 0.0);
}

#[test]
fn from_coefficients_xy_plane() {
    // 0x + 0y + 1z + 0 = 0  =>  the XY plane.
    let p = GpPln::from_coefficients(0.0, 0.0, 1.0, 0.0);
    near(p.normal().z, 1.0);
    near(p.location().z, 0.0);
}

#[test]
fn from_coefficients_offset_plane() {
    // 0x + 0y + 1z - 5 = 0  =>  z = 5.
    let p = GpPln::from_coefficients(0.0, 0.0, 1.0, -5.0);
    near(p.location().z, 5.0);
    near(p.normal().z, 1.0);
}

// ─── Coefficients ────────────────────────────────────────────────────────────

#[test]
fn coefficients_round_trip() {
    // Build a plane from coefficients and verify the returned (a,b,c,d) values.
    // Plane: y = 3  =>  0x + 1y + 0z - 3 = 0
    let p = GpPln::from_coefficients(0.0, 1.0, 0.0, -3.0);
    let (a, b, c, d) = p.coefficients();
    // Normal should be unit +Y.
    near(a, 0.0);
    near(b, 1.0);
    near(c, 0.0);
    near(d, -3.0);
}

// ─── Distance & contains ─────────────────────────────────────────────────────

#[test]
fn distance_point_above_xy_plane() {
    let p = GpPln::default();
    near(p.distance_point(Pnt::new(10.0, -5.0, 7.0)), 7.0);
    near(p.distance_point(Pnt::new(0.0, 0.0, 0.0)), 0.0);
}

#[test]
fn contains_point_on_plane() {
    let p = GpPln::default();
    assert!(p.contains_point(Pnt::new(100.0, -200.0, 0.0), 1e-9));
    assert!(!p.contains_point(Pnt::new(0.0, 0.0, 1e-6), 1e-9));
}

#[test]
fn distance_between_parallel_planes() {
    let p1 = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let p2 = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 13.0), Pnt::new(0.0, 0.0, 1.0));
    near(p1.distance_plane(&p2), 13.0);
}

#[test]
fn distance_between_intersecting_planes_is_zero() {
    let p1 = GpPln::from_point_normal(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let p2 = GpPln::from_point_normal(Pnt::origin(), Pnt::new(0.0, 1.0, 0.0));
    near(p1.distance_plane(&p2), 0.0);
}

// ─── Parametric value & projection ───────────────────────────────────────────

#[test]
fn value_produces_point_on_plane() {
    let p = GpPln::default();
    let pt = p.value(5.0, -3.0);
    pnt_near(pt, Pnt::new(5.0, -3.0, 0.0));
    near(p.distance_point(pt), 0.0);
}

#[test]
fn project_point_returns_foot_of_perpendicular() {
    let p = GpPln::default();
    let pt = Pnt::new(3.0, 4.0, 7.0);
    let foot = p.project_point(pt);
    pnt_near(foot, Pnt::new(3.0, 4.0, 0.0));
    near(p.distance_point(foot), 0.0);
}

#[test]
fn project_returns_uv_coords() {
    let p = GpPln::default();
    let (u, v) = p.project(Pnt::new(3.0, 4.0, 99.0));
    near(u, 3.0);
    near(v, 4.0);
}

// ─── Transforms ──────────────────────────────────────────────────────────────

#[test]
fn translate_shifts_origin() {
    let p = GpPln::default();
    let p2 = p.translated(Pnt::new(1.0, 2.0, 3.0));
    pnt_near(p2.location(), Pnt::new(1.0, 2.0, 3.0));
    pnt_near(p2.normal(), Pnt::new(0.0, 0.0, 1.0));
    // Original is unchanged.
    pnt_near(p.location(), Pnt::origin());
}

#[test]
fn rotate_90_deg_about_x_axis_changes_normal() {
    // XY plane rotated 90° about X: normal +Z → +Y (approximately)
    let p = GpPln::default();
    let ax = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let p2 = p.rotated(ax, FRAC_PI_2);
    let n = p2.normal();
    near(n.x, 0.0);
    // After R_x(90°): (0,0,1) → (0,-1,0) or (0,1,0) depending on convention.
    // Rodrigues with +X axis, +90°: z → -y.
    near(n.y, -1.0);
    near(n.z, 0.0);
}

#[test]
fn scale_positive_moves_origin_and_preserves_normal() {
    let p = GpPln::from_point_normal(Pnt::new(4.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
    let p2 = p.scaled(Pnt::origin(), 2.0);
    pnt_near(p2.location(), Pnt::new(8.0, 0.0, 0.0));
    pnt_near(p2.normal(), Pnt::new(1.0, 0.0, 0.0));
}

#[test]
fn mirror_through_point_reflects_origin() {
    let p = GpPln::from_point_normal(Pnt::new(0.0, 0.0, 4.0), Pnt::new(0.0, 0.0, 1.0));
    let p2 = p.mirrored_through_point(Pnt::origin());
    pnt_near(p2.location(), Pnt::new(0.0, 0.0, -4.0));
}

#[test]
fn mirror_through_xy_plane_negates_z_origin() {
    let p = GpPln::from_point_normal(Pnt::new(1.0, 2.0, 5.0), Pnt::new(0.0, 0.0, 1.0));
    let xy = Ax3::identity(); // XY plane
    let p2 = p.mirrored_through_plane(xy);
    pnt_near(p2.location(), Pnt::new(1.0, 2.0, -5.0));
}

#[test]
fn transformed_with_trsf_translation() {
    let p = GpPln::default();
    let mut t = Trsf::default();
    t.set_translation(Pnt::new(0.0, 0.0, 10.0));
    let p2 = p.transformed(&t);
    pnt_near(p2.location(), Pnt::new(0.0, 0.0, 10.0));
    pnt_near(p2.normal(), Pnt::new(0.0, 0.0, 1.0));
}

// ─── Axes & handedness ───────────────────────────────────────────────────────

#[test]
fn x_and_y_axes_are_in_plane_and_orthonormal() {
    let p = GpPln::from_point_normal(
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(1.0, 0.0, 0.0), // normal = +X
    );
    let xa = p.x_axis();
    let ya = p.y_axis();
    let n = p.normal();
    // Both axes perpendicular to normal.
    near(xa.direction.dot(n), 0.0);
    near(ya.direction.dot(n), 0.0);
    // Mutually perpendicular.
    near(xa.direction.dot(ya.direction), 0.0);
    // Unit length.
    near(xa.direction.norm(), 1.0);
    near(ya.direction.norm(), 1.0);
}

#[test]
fn is_direct_on_standard_frame() {
    let p = GpPln::default();
    assert!(p.is_direct());
}
