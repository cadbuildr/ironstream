//! Ported OpenCascade unit tests -- `gp_Parab`.
//!
//! Faithful Rust ports of OCCT's `gp_Parab` semantics.
//! Same inputs, expected values, and tolerances.

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_parab::Parab;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::FRAC_PI_2;

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

/// Build a `gp_Ax2` with the Z-plane normal and X along world-X.
fn xy_plane_ax2() -> Ax3 {
    Ax3::from_origin_normal(
        Pnt::origin(),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    )
}

/// Standard parabola at the origin in the XY plane with focal = 3.
fn p3() -> Parab {
    Parab::new(xy_plane_ax2(), 3.0)
}

// ================================================================== tests

#[test]
fn constructor_stores_focal() {
    let p = p3();
    near(p.focal(), 3.0, CONFUSION);
}

#[test]
fn parameter_equals_twice_focal() {
    // p = 2 * Focal = 6
    let p = p3();
    near(p.parameter(), 6.0, CONFUSION);
}

#[test]
fn focus_position_at_origin_frame() {
    // Focus = Vertex + Focal * XDir = origin + 3 * (1,0,0) = (3, 0, 0)
    let p = p3();
    pnt_near(p.focus(), Pnt::new(3.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn directrix_axis_location_and_direction() {
    // Directrix through Vertex - Focal * XDir = (-3, 0, 0), along YDir=(0,1,0)
    let p = p3();
    let d = p.directrix_axis();
    pnt_near(d.location, Pnt::new(-3.0, 0.0, 0.0), CONFUSION);
    assert!(
        d.direction.is_equal_dir(Pnt::new(0.0, 1.0, 0.0), ANGULAR),
        "directrix direction should be YDir"
    );
}

#[test]
fn axis_directions_from_identity_frame() {
    let p = Parab::new(Ax3::identity(), 2.0);
    assert!(
        p.x_axis().direction.is_equal_dir(Pnt::new(1.0, 0.0, 0.0), ANGULAR),
        "XAxis should be +X"
    );
    assert!(
        p.y_axis().direction.is_equal_dir(Pnt::new(0.0, 1.0, 0.0), ANGULAR),
        "YAxis should be +Y"
    );
    assert!(
        p.axis().direction.is_equal_dir(Pnt::new(0.0, 0.0, 1.0), ANGULAR),
        "Axis (normal) should be +Z"
    );
}

#[test]
fn translate_moves_vertex() {
    let p = p3();
    let v = Pnt::new(10.0, 20.0, 30.0);
    let t = p.translated(v);
    pnt_near(t.location(), v, CONFUSION);
    near(t.focal(), 3.0, CONFUSION);
    // Focus must have moved too: new_focus = v + 3*(1,0,0)
    pnt_near(t.focus(), Pnt::new(13.0, 20.0, 30.0), CONFUSION);
}

#[test]
fn rotate_90_about_z_moves_vertex_and_axis() {
    // Vertex at (1, 0, 0); rotate 90° about Z → vertex at (0, 1, 0).
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let p = Parab::new(ax2, 3.0);
    let rot = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let r = p.rotated(rot, FRAC_PI_2);
    pnt_near(r.location(), Pnt::new(0.0, 1.0, 0.0), CONFUSION);
    near(r.focal(), 3.0, CONFUSION);
    // XDir should now point along +Y (was +X, rotated 90°)
    assert!(
        r.x_axis().direction.is_equal_dir(Pnt::new(0.0, 1.0, 0.0), CONFUSION),
        "XDir should point +Y after 90° rotation"
    );
}

#[test]
fn scale_adjusts_focal_and_vertex_location() {
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let p = Parab::new(ax2, 3.0);
    let s = p.scaled(Pnt::origin(), 2.0);
    near(s.focal(), 6.0, CONFUSION);
    pnt_near(s.location(), Pnt::new(2.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn transform_with_translation_trsf() {
    let p = p3();
    let mut t = Trsf::identity();
    t.set_translation(Pnt::new(0.0, 0.0, 7.0));
    let tp = p.transformed(&t);
    near(tp.location().z, 7.0, CONFUSION);
    near(tp.focal(), 3.0, CONFUSION);
}

#[test]
fn transform_with_scale_trsf() {
    let p = p3();
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 4.0);
    let tp = p.transformed(&t);
    near(tp.focal(), 12.0, CONFUSION);
}

#[test]
fn mirror_point_symmetry() {
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(2.0, 3.0, 4.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let p = Parab::new(ax2, 3.0);
    let m = p.mirrored_point(Pnt::origin());
    pnt_near(m.location(), Pnt::new(-2.0, -3.0, -4.0), CONFUSION);
    near(m.focal(), 3.0, CONFUSION);
}

#[test]
fn mirror_axis1_reflects_vertex() {
    // Parab vertex at (1, 1, 0); reflect across X axis → vertex at (1, -1, 0).
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let p = Parab::new(ax2, 3.0);
    let axis = Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let m = p.mirrored_axis1(axis);
    near(m.location().x, 1.0, CONFUSION);
    near(m.location().y, -1.0, CONFUSION);
    near(m.location().z, 0.0, CONFUSION);
    near(m.focal(), 3.0, CONFUSION);
}

#[test]
fn mirror_plane_reflects_vertex() {
    // Parab vertex at z = 5; mirror in the XY plane → vertex at z = -5.
    let ax2 = Ax3::from_origin_normal(
        Pnt::new(0.0, 0.0, 5.0),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let p = Parab::new(ax2, 3.0);
    let mirror = Ax3::from_origin_normal(
        Pnt::origin(),
        Pnt::new(0.0, 0.0, 1.0),
        Pnt::new(1.0, 0.0, 0.0),
    );
    let m = p.mirrored_plane(mirror);
    near(m.location().z, -5.0, CONFUSION);
    near(m.focal(), 3.0, CONFUSION);
}

#[test]
fn set_focal_updates_geometry() {
    let mut p = p3();
    p.set_focal(10.0);
    near(p.focal(), 10.0, CONFUSION);
    near(p.parameter(), 20.0, CONFUSION);
    pnt_near(p.focus(), Pnt::new(10.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn focus_to_directrix_equals_focal() {
    // For any parabola: distance from Focus to Directrix = 2 * Focal.
    let p = p3();
    let focus = p.focus();
    let d = p.directrix();
    let dist = d.distance_point(focus);
    near(dist, 2.0 * p.focal(), CONFUSION);
}
