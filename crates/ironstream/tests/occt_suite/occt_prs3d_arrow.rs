// FILE: rust/ironstream/crates/ironstream/tests/occt_prs3d_arrow.rs
extern crate ironstream;
use ironstream::prs3d_arrow::*;

use std::f64::consts::PI;

// ── helpers ─────────────────────────────────────────────────────────────────

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn len(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

// ── Prs3dArrowType ───────────────────────────────────────────────────────────

#[test]
fn arrow_type_variants_are_distinct() {
    assert_ne!(Prs3dArrowType::External, Prs3dArrowType::Internal);
    assert_ne!(Prs3dArrowType::Internal, Prs3dArrowType::UserDefined);
    assert_ne!(Prs3dArrowType::External, Prs3dArrowType::UserDefined);
}

#[test]
fn arrow_type_copy_clone() {
    let t = Prs3dArrowType::Internal;
    let t2 = t;
    assert_eq!(t, t2);
    let t3 = t.clone();
    assert_eq!(t, t3);
}

// ── construction defaults ────────────────────────────────────────────────────

#[test]
fn new_stores_location() {
    let a = Prs3dArrow::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 5.0);
    assert_eq!(a.location(), [1.0, 2.0, 3.0]);
}

#[test]
fn new_stores_direction() {
    let a = Prs3dArrow::new([0.0; 3], [1.0, 0.0, 0.0], 5.0);
    assert_eq!(a.direction(), [1.0, 0.0, 0.0]);
}

#[test]
fn new_stores_length() {
    let a = Prs3dArrow::new([0.0; 3], [0.0, 1.0, 0.0], 7.5);
    assert!((a.length() - 7.5).abs() < 1e-14);
}

#[test]
fn new_default_angle_is_pi_over_12() {
    let a = Prs3dArrow::new([0.0; 3], [0.0, 1.0, 0.0], 1.0);
    assert!((a.angle() - PI / 12.0).abs() < 1e-14);
}

#[test]
fn new_default_arrow_type_is_external() {
    let a = Prs3dArrow::new([0.0; 3], [0.0, 1.0, 0.0], 1.0);
    assert_eq!(a.arrow_type(), Prs3dArrowType::External);
}

// ── setters roundtrip ────────────────────────────────────────────────────────

#[test]
fn set_angle_roundtrip() {
    let mut a = Prs3dArrow::new([0.0; 3], [1.0, 0.0, 0.0], 1.0);
    a.set_angle(PI / 6.0);
    assert!((a.angle() - PI / 6.0).abs() < 1e-14);
}

#[test]
fn set_arrow_type_roundtrip() {
    let mut a = Prs3dArrow::new([0.0; 3], [1.0, 0.0, 0.0], 1.0);
    a.set_arrow_type(Prs3dArrowType::Internal);
    assert_eq!(a.arrow_type(), Prs3dArrowType::Internal);
    a.set_arrow_type(Prs3dArrowType::UserDefined);
    assert_eq!(a.arrow_type(), Prs3dArrowType::UserDefined);
}

// ── tip ──────────────────────────────────────────────────────────────────────

#[test]
fn tip_along_z_axis() {
    let a = Prs3dArrow::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    let t = a.tip();
    assert!((t[0]).abs() < 1e-14);
    assert!((t[1]).abs() < 1e-14);
    assert!((t[2] - 3.0).abs() < 1e-14);
}

#[test]
fn tip_along_x_axis_with_offset_location() {
    let a = Prs3dArrow::new([1.0, 2.0, 3.0], [1.0, 0.0, 0.0], 4.0);
    let t = a.tip();
    assert!((t[0] - 5.0).abs() < 1e-14);
    assert!((t[1] - 2.0).abs() < 1e-14);
    assert!((t[2] - 3.0).abs() < 1e-14);
}

#[test]
fn tip_distance_from_location_equals_length() {
    let loc = [1.0, -2.0, 3.0];
    let dir = [0.0, 1.0, 0.0];
    let length = 6.0;
    let a = Prs3dArrow::new(loc, dir, length);
    assert!((dist(a.tip(), loc) - length).abs() < 1e-12);
}

// ── wings: geometry properties ───────────────────────────────────────────────

/// Both wings must be the same distance from the tip.
#[test]
fn wings_equidistant_from_tip() {
    let a = Prs3dArrow::new([0.0; 3], [0.0, 1.0, 0.0], 2.0);
    let t = a.tip();
    let dl = dist(a.left_wing(), t);
    let dr = dist(a.right_wing(), t);
    assert!((dl - dr).abs() < 1e-12, "dl={dl} dr={dr}");
}

/// The half-angle at the tip between each wing and the shaft direction equals
/// the configured angle.
#[test]
fn wing_half_angle_matches_configured_angle() {
    let length = 3.0;
    let angle = PI / 12.0;
    let a = Prs3dArrow::new([0.0; 3], [0.0, 0.0, 1.0], length);
    let t = a.tip();
    let lw = a.left_wing();
    // vector from tip to wing
    let v = sub(lw, t);
    let mag = len(v);
    // unit direction reversed (from tip toward tail)
    let shaft = [0.0_f64, 0.0, -1.0];
    // cos of half-angle between shaft and (tip→wing)
    let cos_a = dot(v, shaft) / mag;
    let computed_angle = cos_a.acos();
    assert!(
        (computed_angle - angle).abs() < 1e-10,
        "expected {angle} got {computed_angle}"
    );
}

/// Wings are symmetric: the midpoint of left_wing and right_wing lies on the
/// axis (location + direction * t for some t).
#[test]
fn wings_symmetric_about_shaft_axis() {
    let dir = [0.0_f64, 0.0, 1.0];
    let a = Prs3dArrow::new([1.0, 2.0, 3.0], dir, 5.0);
    let lw = a.left_wing();
    let rw = a.right_wing();
    let mid = [(lw[0] + rw[0]) / 2.0, (lw[1] + rw[1]) / 2.0, (lw[2] + rw[2]) / 2.0];
    let t = a.tip();
    // mid should equal tip − direction * length (the base of the arrowhead on
    // the shaft) projected back; it must lie on the axis, so the component
    // perpendicular to the direction must be zero.
    let from_loc = sub(mid, a.location());
    // perp component = from_loc − (from_loc·dir)*dir
    let proj = dot(from_loc, dir);
    let perp_sq = {
        let p = [
            from_loc[0] - dir[0] * proj,
            from_loc[1] - dir[1] * proj,
            from_loc[2] - dir[2] * proj,
        ];
        p[0] * p[0] + p[1] * p[1] + p[2] * p[2]
    };
    let _ = t; // silence unused warning
    assert!(perp_sq < 1e-24, "perp component squared: {perp_sq}");
}

/// Left and right wings lie on opposite sides of the shaft axis: the component
/// of (lw − tip) perpendicular to the direction is opposite in sign to the
/// corresponding component of (rw − tip).  We verify this by checking that the
/// sum of the two perpendicular components is (nearly) zero while each
/// individual magnitude is non-zero.
#[test]
fn left_and_right_wings_on_opposite_sides() {
    let dir = [1.0_f64, 0.0, 0.0];
    let a = Prs3dArrow::new([0.0; 3], dir, 4.0);
    let t = a.tip();
    let vl = sub(a.left_wing(), t);
    let vr = sub(a.right_wing(), t);
    // Strip the component along the direction (they are equal for both wings).
    let proj_l = dot(vl, dir);
    let proj_r = dot(vr, dir);
    let perp_l = [vl[0] - dir[0] * proj_l, vl[1] - dir[1] * proj_l, vl[2] - dir[2] * proj_l];
    let perp_r = [vr[0] - dir[0] * proj_r, vr[1] - dir[1] * proj_r, vr[2] - dir[2] * proj_r];
    // The perpendicular components must be equal and opposite.
    let sum = [perp_l[0] + perp_r[0], perp_l[1] + perp_r[1], perp_l[2] + perp_r[2]];
    let sum_mag = len(sum);
    let each_mag = len(perp_l);
    assert!(each_mag > 1e-10, "perp magnitude should be non-zero: {each_mag}");
    assert!(sum_mag < 1e-10, "perp components should cancel: sum_mag={sum_mag}");
}

// ── wings: different directions ───────────────────────────────────────────────

#[test]
fn wings_with_y_direction() {
    let a = Prs3dArrow::new([0.0; 3], [0.0, 1.0, 0.0], 3.0);
    let lw = a.left_wing();
    let rw = a.right_wing();
    // wings must not be NaN
    assert!(lw.iter().all(|x| x.is_finite()));
    assert!(rw.iter().all(|x| x.is_finite()));
    // and must differ from each other
    assert!(dist(lw, rw) > 1e-10);
}

#[test]
fn wings_with_diagonal_direction() {
    let s = (1.0_f64 / 3.0_f64).sqrt();
    let a = Prs3dArrow::new([0.0; 3], [s, s, s], 2.0);
    let lw = a.left_wing();
    let rw = a.right_wing();
    assert!(lw.iter().all(|x| x.is_finite()));
    assert!(rw.iter().all(|x| x.is_finite()));
    assert!(dist(lw, rw) > 1e-10);
}

// ── custom angle ─────────────────────────────────────────────────────────────

#[test]
fn wider_angle_produces_farther_wings() {
    let loc = [0.0; 3];
    let dir = [0.0, 0.0, 1.0];
    let length = 5.0;
    let mut a_narrow = Prs3dArrow::new(loc, dir, length);
    a_narrow.set_angle(PI / 24.0); // 7.5°
    let mut a_wide = Prs3dArrow::new(loc, dir, length);
    a_wide.set_angle(PI / 4.0); // 45°

    let t = a_narrow.tip();
    let d_narrow = dist(a_narrow.left_wing(), t);
    let d_wide = dist(a_wide.left_wing(), t);
    assert!(d_wide > d_narrow, "wide={d_wide} narrow={d_narrow}");
}

// ── zero-length edge cases ────────────────────────────────────────────────────

#[test]
fn zero_length_tip_equals_location() {
    let loc = [3.0, 4.0, 5.0];
    let a = Prs3dArrow::new(loc, [1.0, 0.0, 0.0], 0.0);
    assert_eq!(a.tip(), loc);
}
