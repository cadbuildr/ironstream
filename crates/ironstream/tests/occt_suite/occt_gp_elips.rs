//! Ported OpenCascade unit tests -- `gp_Elips`.
//!
//! Faithful Rust ports of OCCT's `gp_Elips_Test.cxx`
//! (Open-Cascade-SAS/OCCT, `src/FoundationClasses/TKMath/GTests/`).
//! Same inputs, expected values, and tolerances.

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_elips::Elips;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, PI};

// ----------------------------------------------------------------- helpers

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

fn pnt_near(a: Pnt, b: Pnt, tol: f64) {
    assert!(
        a.is_equal(b, tol),
        "expected {a:?} ≈ {b:?} (tol {tol})"
    );
}

/// Build a standard `gp_Ax2` with the Z-plane normal and X along world-X.
fn xy_plane_ax2() -> Ax3 {
    Ax3::from_origin_normal(
        Pnt::origin(),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    )
}

/// Standard 5×3 ellipse at the origin in the XY plane.
fn e53() -> Elips {
    Elips::new(xy_plane_ax2(), 5.0, 3.0)
}

// ================================================================== tests

#[test]
fn constructor_stores_radii() {
    let e = e53();
    near(e.major_radius(), 5.0, CONFUSION);
    near(e.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn location_is_centre() {
    let centre = Pnt::new(1.0, 2.0, 3.0);
    let ax2 = Ax3::from_origin_normal(centre, Pnt::new(0.0, 0.0, 1.0), Pnt::new(1.0, 0.0, 0.0));
    let e = Elips::new(ax2, 10.0, 4.0);
    pnt_near(e.location(), centre, CONFUSION);
}

#[test]
fn eccentricity_value() {
    // e = sqrt(a² - b²) / a  = sqrt(25-9)/5 = 4/5 = 0.8
    let e = e53();
    near(e.eccentricity(), 0.8, CONFUSION);
}

#[test]
fn circle_has_zero_eccentricity() {
    // When major == minor the ellipse degenerates to a circle; e must be 0.
    let e = Elips::new(xy_plane_ax2(), 7.0, 7.0);
    near(e.eccentricity(), 0.0, CONFUSION);
    near(e.focal(), 0.0, CONFUSION);
}

#[test]
fn focal_distance() {
    // focal = 2*sqrt(25-9) = 2*4 = 8
    let e = e53();
    near(e.focal(), 8.0, CONFUSION);
}

#[test]
fn foci_positions() {
    // c = 4, so Focus1 = (+4, 0, 0), Focus2 = (-4, 0, 0)
    let e = e53();
    pnt_near(e.focus1(), Pnt::new(4.0, 0.0, 0.0), CONFUSION);
    pnt_near(e.focus2(), Pnt::new(-4.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn parameter_semi_latus_rectum() {
    // p = b²/a = 9/5 = 1.8
    let e = e53();
    near(e.parameter(), 1.8, CONFUSION);
}

#[test]
fn area() {
    // A = π * 5 * 3 = 15π
    let e = e53();
    near(e.area(), 15.0 * PI, CONFUSION);
}

#[test]
fn axis_directions_identity_frame() {
    let e = Elips::new(Ax3::identity(), 5.0, 3.0);
    assert!(
        e.x_axis()
            .direction
            .is_equal_dir(Pnt::new(1.0, 0.0, 0.0), ANGULAR),
        "XAxis should point +X"
    );
    assert!(
        e.y_axis()
            .direction
            .is_equal_dir(Pnt::new(0.0, 1.0, 0.0), ANGULAR),
        "YAxis should point +Y"
    );
    assert!(
        e.axis()
            .direction
            .is_equal_dir(Pnt::new(0.0, 0.0, 1.0), ANGULAR),
        "Axis should point +Z"
    );
}

#[test]
fn translate_moves_centre() {
    let e = e53();
    let v = Pnt::new(10.0, 20.0, 30.0);
    let t = e.translated(v);
    pnt_near(t.location(), v, CONFUSION);
    // Radii must be unchanged.
    near(t.major_radius(), 5.0, CONFUSION);
    near(t.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn rotate_moves_centre_keeps_radii() {
    // Ellipse centred at (1, 0, 0); rotate 90° about the Z axis → (0, 1, 0).
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let e = Elips::new(ax2, 5.0, 3.0);
    let rot_axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = e.rotated(rot_axis, FRAC_PI_2);
    pnt_near(r.location(), Pnt::new(0.0, 1.0, 0.0), CONFUSION);
    near(r.major_radius(), 5.0, CONFUSION);
    near(r.minor_radius(), 3.0, CONFUSION);
}

#[test]
fn scale_adjusts_radii_and_location() {
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let e = Elips::new(ax2, 5.0, 3.0);
    let s = e.scaled(Pnt::origin(), 2.0);
    near(s.major_radius(), 10.0, CONFUSION);
    near(s.minor_radius(), 6.0, CONFUSION);
    pnt_near(s.location(), Pnt::new(2.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn transform_with_translation() {
    let e = e53();
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(0.0, 0.0, 7.0));
    let te = e.transformed(&t);
    near(te.location().z, 7.0, CONFUSION);
    near(te.major_radius(), 5.0, CONFUSION);
}

#[test]
fn transform_with_scale() {
    let e = e53();
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 3.0);
    let te = e.transformed(&t);
    near(te.major_radius(), 15.0, CONFUSION);
    near(te.minor_radius(), 9.0, CONFUSION);
}

#[test]
fn mirror_point_negates_centre() {
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(2.0, 3.0, 4.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let e = Elips::new(ax2, 5.0, 3.0);
    let m = e.mirrored_point(Pnt::origin());
    pnt_near(m.location(), Pnt::new(-2.0, -3.0, -4.0), CONFUSION);
    near(m.major_radius(), 5.0, CONFUSION);
}

#[test]
fn set_major_radius_updates() {
    let mut e = e53();
    e.set_major_radius(10.0);
    near(e.major_radius(), 10.0, CONFUSION);
    // Eccentricity should be recalculated accordingly: sqrt(100-9)/10
    let expected_e = (91.0_f64).sqrt() / 10.0;
    near(e.eccentricity(), expected_e, CONFUSION);
}

#[test]
fn set_minor_radius_updates() {
    let mut e = e53();
    e.set_minor_radius(1.0);
    near(e.minor_radius(), 1.0, CONFUSION);
}

#[test]
fn foci_sum_of_distances_is_major_axis() {
    // For any point P on the ellipse: |P-F1| + |P-F2| = 2*a
    let e = e53();
    let f1 = e.focus1();
    let f2 = e.focus2();
    // Vertex along +X: (5, 0, 0)
    let vertex = Pnt::new(5.0, 0.0, 0.0);
    let d1 = vertex.distance(f1);
    let d2 = vertex.distance(f2);
    near(d1 + d2, 2.0 * 5.0, CONFUSION);
}

#[test]
fn mirrored_plane_reflects_centre() {
    // Ellipse at z = 5; mirror in the XY plane → centre at z = -5.
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 5.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let e = Elips::new(ax2, 5.0, 3.0);
    let mirror = Ax3::from_origin_normal(
        Pnt::origin(),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let m = e.mirrored_plane(mirror);
    near(m.location().z, -5.0, CONFUSION);
    near(m.major_radius(), 5.0, CONFUSION);
}

#[test]
fn mirrored_axis1_reflects_centre() {
    // Ellipse at (1, 1, 0); reflect across the X axis → (1, -1, 0).
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let e = Elips::new(ax2, 5.0, 3.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let m = e.mirrored_axis1(axis);
    near(m.location().x, 1.0, CONFUSION);
    near(m.location().y, -1.0, CONFUSION);
    near(m.location().z, 0.0, CONFUSION);
    near(m.major_radius(), 5.0, CONFUSION);
}
