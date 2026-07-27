//! Ported OpenCascade unit tests -- `gp_Parab2d`.
//!
//! Faithful Rust ports of OCCT's `gp_Parab2d` tests. Same numeric inputs,
//! expected values, and tolerances as upstream. The standard fixture is a
//! parabola at the origin with focal length 2 and its axis of symmetry along +X.
//!
//! OCCT helper-type mapping:
//! - `gp_Pnt2d`   -> [`ironstream::gp2d::Pnt2d`]
//! - `gp_Ax2d`    -> [`ironstream::gp2d::Ax2d`]
//! - `gp_Ax22d`   -> [`ironstream::geom2d_circle::Ax22d2`]
//! - `gp_Trsf2d`  -> [`ironstream::gp2d::Trsf2d`]
//! - `gp_Parab2d` -> [`ironstream::gp_parab2d::Parab2d`]

use ironstream::geom2d_circle::Ax22d2;
use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::gp_parab2d::Parab2d;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::FRAC_PI_2;

// ─────────────────────────────────────────────────────── helpers ──────────

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

fn pnt_near(a: Pnt2d, b: Pnt2d, tol: f64) {
    assert!(
        (a - b).norm() <= tol,
        "expected {a:?} ≈ {b:?} (tol {tol})"
    );
}

/// Standard fixture: parabola at origin, focal = 2, axis of symmetry along +X.
fn p2() -> Parab2d {
    Parab2d::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        2.0,
    )
}

// ─────────────────────────────────────────────────────────── tests ────────

// TEST(gp_Parab2d, Constructor)
#[test]
fn constructor_stores_focal() {
    let p = p2();
    near(p.focal(), 2.0, CONFUSION);
    // parameter p = 2 * focal
    near(p.parameter(), 4.0, CONFUSION);
}

// TEST(gp_Parab2d, LocationIsVertex)
#[test]
fn location_is_vertex() {
    let vertex = Pnt2d::new(3.0, -1.0);
    let p = Parab2d::new(
        Ax2d::new(vertex, Pnt2d::new(1.0, 0.0)),
        5.0,
    );
    pnt_near(p.location(), vertex, CONFUSION);
}

// TEST(gp_Parab2d, FocusPosition)
#[test]
fn focus_position() {
    // Focus = vertex + focal * XDir = (0,0) + 2*(1,0) = (2,0)
    let p = p2();
    let f = p.focus();
    near(f.x, 2.0, CONFUSION);
    near(f.y, 0.0, CONFUSION);
}

// TEST(gp_Parab2d, FocusOnOffAxisParabola)
#[test]
fn focus_on_offset_vertex() {
    // vertex at (1, 2), focal = 3, axis +X → focus at (4, 2)
    let p = Parab2d::new(
        Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0)),
        3.0,
    );
    let f = p.focus();
    near(f.x, 4.0, CONFUSION);
    near(f.y, 2.0, CONFUSION);
}

// TEST(gp_Parab2d, DirectrixPosition)
#[test]
fn directrix_position() {
    // Directrix at x = -focal = -2, running in Y direction.
    let p = p2();
    let d = p.directrix();
    near(d.location.x, -2.0, CONFUSION);
    near(d.location.y, 0.0, CONFUSION);
    // Direction must be Y-like (perpendicular to the X axis of the parabola)
    near(d.direction.x.abs(), 0.0, ANGULAR);
    near(d.direction.y.abs(), 1.0, ANGULAR);
}

// TEST(gp_Parab2d, DirectrixFocusDistanceProperty)
#[test]
fn directrix_focus_distance_property() {
    // For a parabola the distance from a point on the curve to the focus
    // equals the distance to the directrix.
    // We verify this for the vertex: dist_vertex_to_focus = focal = 2,
    // dist_vertex_to_directrix = focal = 2.
    let p = p2();
    let vertex = p.location();
    let focus = p.focus();
    let directrix_x = p.directrix().location.x;

    let dist_to_focus = vertex.distance(focus);
    let dist_to_directrix = (vertex.x - directrix_x).abs();
    near(dist_to_focus, dist_to_directrix, CONFUSION);
}

// TEST(gp_Parab2d, SetFocalUpdates)
#[test]
fn set_focal_updates() {
    let mut p = p2();
    p.set_focal(7.0);
    near(p.focal(), 7.0, CONFUSION);
    near(p.parameter(), 14.0, CONFUSION);
    // Focus should update accordingly
    let f = p.focus();
    near(f.x, 7.0, CONFUSION);
}

// TEST(gp_Parab2d, SetLocationUpdates)
#[test]
fn set_location_updates() {
    let mut p = p2();
    p.set_location(Pnt2d::new(5.0, -3.0));
    pnt_near(p.location(), Pnt2d::new(5.0, -3.0), CONFUSION);
    near(p.focal(), 2.0, CONFUSION);
    // Focus moves with the vertex
    let f = p.focus();
    near(f.x, 7.0, CONFUSION);
    near(f.y, -3.0, CONFUSION);
}

// TEST(gp_Parab2d, TranslateMovesVertex)
#[test]
fn translate_moves_vertex() {
    let p = p2();
    let t = p.translated(Pnt2d::new(10.0, 20.0));
    pnt_near(t.location(), Pnt2d::new(10.0, 20.0), CONFUSION);
    near(t.focal(), 2.0, CONFUSION);
    // Focus should have moved too
    let f = t.focus();
    near(f.x, 12.0, CONFUSION);
    near(f.y, 20.0, CONFUSION);
}

// TEST(gp_Parab2d, RotateMovesVertex)
#[test]
fn rotate_moves_vertex() {
    // Parabola at (1,0), focal=3; rotate 90° CCW about origin → vertex (0,1).
    let p = Parab2d::new(
        Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
        3.0,
    );
    let r = p.rotated(Pnt2d::origin(), FRAC_PI_2);
    near(r.location().x, 0.0, CONFUSION);
    near(r.location().y, 1.0, CONFUSION);
    near(r.focal(), 3.0, CONFUSION);
    // Axis of symmetry is now along +Y; focus at (0, 1+3) = (0, 4).
    let f = r.focus();
    near(f.x, 0.0, CONFUSION);
    near(f.y, 4.0, CONFUSION);
}

// TEST(gp_Parab2d, ScaleAdjustsFocalAndLocation)
#[test]
fn scale_adjusts_focal_and_location() {
    // Parabola at (1,0), focal=2; scale by 3 about origin.
    let p = Parab2d::new(
        Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0)),
        2.0,
    );
    let s = p.scaled(Pnt2d::origin(), 3.0);
    near(s.focal(), 6.0, CONFUSION);
    near(s.location().x, 3.0, CONFUSION);
    near(s.location().y, 0.0, CONFUSION);
}

// TEST(gp_Parab2d, MirrorPointReflectsVertex)
#[test]
fn mirror_point_reflects_vertex() {
    // Parabola at (2,3); mirror in origin → (-2,-3).
    let p = Parab2d::new(
        Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0)),
        2.0,
    );
    let m = p.mirrored_point(Pnt2d::origin());
    pnt_near(m.location(), Pnt2d::new(-2.0, -3.0), CONFUSION);
    near(m.focal(), 2.0, CONFUSION);
}

// TEST(gp_Parab2d, MirrorAxisReflectsVertex)
#[test]
fn mirror_axis_reflects_vertex() {
    // Parabola at (1,1); reflect across X axis (Y=0) → vertex (1,-1).
    let p = Parab2d::new(
        Ax2d::new(Pnt2d::new(1.0, 1.0), Pnt2d::new(1.0, 0.0)),
        2.0,
    );
    let axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let m = p.mirrored_axis(axis);
    near(m.location().x, 1.0, CONFUSION);
    near(m.location().y, -1.0, CONFUSION);
    near(m.focal(), 2.0, CONFUSION);
}

// TEST(gp_Parab2d, TransformWithTranslation)
#[test]
fn transform_with_translation() {
    let p = p2();
    let t = Trsf2d::translation(Pnt2d::new(7.0, 11.0));
    let tp = p.transformed(&t);
    near(tp.location().x, 7.0, CONFUSION);
    near(tp.location().y, 11.0, CONFUSION);
    near(tp.focal(), 2.0, CONFUSION);
}

// TEST(gp_Parab2d, TransformWithRotation)
#[test]
fn transform_with_rotation() {
    // Parabola at (5,0); rotate 90° CCW → vertex at (0,5).
    let p = Parab2d::new(
        Ax2d::new(Pnt2d::new(5.0, 0.0), Pnt2d::new(1.0, 0.0)),
        2.0,
    );
    let t = Trsf2d::rotation(Pnt2d::origin(), FRAC_PI_2);
    let tp = p.transformed(&t);
    near(tp.location().x, 0.0, CONFUSION);
    near(tp.location().y, 5.0, CONFUSION);
    near(tp.focal(), 2.0, CONFUSION);
}

// TEST(gp_Parab2d, XAxisAndYAxis)
#[test]
fn x_axis_and_y_axis() {
    let p = p2();
    let xa = p.x_axis();
    let ya = p.y_axis();
    // X axis direction = +X
    near(xa.direction.x, 1.0, ANGULAR);
    near(xa.direction.y, 0.0, ANGULAR);
    // Y axis direction = +Y
    near(ya.direction.x, 0.0, ANGULAR);
    near(ya.direction.y, 1.0, ANGULAR);
    // Both axes pass through the vertex
    pnt_near(xa.location, Pnt2d::origin(), CONFUSION);
    pnt_near(ya.location, Pnt2d::origin(), CONFUSION);
}

// TEST(gp_Parab2d, IsDirectDefault)
#[test]
fn is_direct_default_frame() {
    let p = p2();
    assert!(p.is_direct(), "default (sense=true) must be direct");
}

// TEST(gp_Parab2d, IsDirectIndirect)
#[test]
fn is_direct_indirect_frame() {
    let p = Parab2d::new_with_sense(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        2.0,
        false,
    );
    assert!(!p.is_direct(), "sense=false must be indirect");
}

// TEST(gp_Parab2d, SetPositionUpdates)
#[test]
fn set_position_updates() {
    let mut p = p2();
    let new_pos = Ax22d2::from_x_axis(Pnt2d::new(4.0, 5.0), Pnt2d::new(0.0, 1.0), true);
    p.set_position(new_pos);
    pnt_near(p.location(), Pnt2d::new(4.0, 5.0), CONFUSION);
    near(p.focal(), 2.0, CONFUSION);
    // New axis of symmetry is +Y, so focus is at (4, 5+2) = (4, 7)
    let f = p.focus();
    near(f.x, 4.0, CONFUSION);
    near(f.y, 7.0, CONFUSION);
}

// TEST(gp_Parab2d, FromAxisConstructor)
#[test]
fn from_axis_constructor() {
    let pos = Ax22d2::from_x_axis(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let p = Parab2d::from_axis(pos, 5.0);
    near(p.focal(), 5.0, CONFUSION);
    near(p.parameter(), 10.0, CONFUSION);
    pnt_near(p.location(), Pnt2d::new(1.0, 2.0), CONFUSION);
}

// TEST(gp_Parab2d, FocalZeroAllowed)
#[test]
fn focal_zero_allowed() {
    // focal = 0 is the degenerate case; OCCT allows it.
    let p = Parab2d::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        0.0,
    );
    near(p.focal(), 0.0, CONFUSION);
    near(p.parameter(), 0.0, CONFUSION);
    // Focus and directrix both coincide with the vertex
    pnt_near(p.focus(), Pnt2d::origin(), CONFUSION);
    let d = p.directrix();
    pnt_near(d.location, Pnt2d::origin(), CONFUSION);
}

// TEST(gp_Parab2d, TiltedAxisFocusDirection)
#[test]
fn tilted_axis_focus_direction() {
    // Parabola at origin, axis at 45°: focus must be focal units from vertex
    // along the 45° direction.
    let angle = std::f64::consts::PI / 4.0;
    let dir = Pnt2d::new(angle.cos(), angle.sin());
    let focal = 3.0;
    let p = Parab2d::new(Ax2d::new(Pnt2d::origin(), dir), focal);
    let f = p.focus();
    // Distance from vertex to focus = focal
    near(f.norm(), focal, CONFUSION);
    // Direction from vertex to focus should match the parabola's X direction
    let f_dir = f.normalized();
    near(f_dir.x, angle.cos(), CONFUSION);
    near(f_dir.y, angle.sin(), CONFUSION);
}

// TEST(gp_Parab2d, ScaleNegativeFlipsDirections)
#[test]
fn scale_negative_flips_directions() {
    // Scale by -1 about origin: vertex stays at origin (0,0), focal scales by
    // |-1| = 1 so focal is unchanged, but the X and Y directions both flip.
    // The frame was originally direct; negating both X and Y preserves the
    // cross-product sign ((-X)×(-Y) = X×Y), so is_direct() remains true.
    let p = p2();
    let s = p.scaled(Pnt2d::origin(), -1.0);
    near(s.focal(), 2.0, CONFUSION);
    pnt_near(s.location(), Pnt2d::origin(), CONFUSION);
    // After negative scale the X axis direction must point toward -X.
    let xd = s.x_axis().direction;
    near(xd.x, -1.0, ANGULAR);
    near(xd.y, 0.0, ANGULAR);
    // Focus is now at vertex + focal * (-XDir) = (-2, 0).
    let f = s.focus();
    near(f.x, -2.0, CONFUSION);
    near(f.y, 0.0, CONFUSION);
}
