// FILE: rust/ironstream/crates/ironstream/tests/occt_geom_mirror.rs
//! Ported OpenCascade unit tests — `geom_mirror` (BRepBuilderAPI_Transform).
//!
//! Verifies `MirrorPlane`, `MirrorAxis`, and `TransformMatrix` against the
//! same geometric expectations that OCCT's mirror/transform API guarantees.

extern crate ironstream;
use ironstream::geom_mirror::*;

const TOL: f64 = 1.0e-12;

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= TOL,
        "expected {a} ≈ {b}; diff = {:.2e}",
        (a - b).abs()
    );
}

fn arr_near(got: [f64; 3], expected: [f64; 3]) {
    near(got[0], expected[0]);
    near(got[1], expected[1]);
    near(got[2], expected[2]);
}

// ─────────────────────────────────────────────────────────────────────────────
// MirrorPlane
// ─────────────────────────────────────────────────────────────────────────────

/// Reflecting a point that lies ON the plane leaves it unchanged.
#[test]
fn mirror_plane_point_on_plane_is_fixed() {
    // XY plane through origin
    let plane = MirrorPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let p = [3.0, 4.0, 0.0];
    arr_near(plane.reflect_point(p), p);
}

/// Reflecting through the XY plane (normal = Z) negates the Z coordinate.
#[test]
fn mirror_plane_xy_negates_z() {
    let plane = MirrorPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let p = [1.0, 2.0, 5.0];
    arr_near(plane.reflect_point(p), [1.0, 2.0, -5.0]);
}

/// Reflecting through the YZ plane (normal = X) negates the X coordinate.
#[test]
fn mirror_plane_yz_negates_x() {
    let plane = MirrorPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let p = [3.0, 7.0, -2.0];
    arr_near(plane.reflect_point(p), [-3.0, 7.0, -2.0]);
}

/// Reflecting twice returns the original point (involution).
#[test]
fn mirror_plane_double_reflection_is_identity() {
    let plane = MirrorPlane::new([1.0, 2.0, 3.0], [1.0, 1.0, 0.0]);
    let p = [4.0, -1.0, 7.0];
    let once = plane.reflect_point(p);
    let twice = plane.reflect_point(once);
    arr_near(twice, p);
}

/// Plane not through origin: the offset is respected.
#[test]
fn mirror_plane_offset_origin() {
    // Plane z = 5 (normal Z, origin (0,0,5))
    let plane = MirrorPlane::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let p = [1.0, 2.0, 3.0];
    // Distance from plane: 3 - 5 = -2; reflected z = 3 - 2*(-2) = 7
    arr_near(plane.reflect_point(p), [1.0, 2.0, 7.0]);
}

/// `origin()` and `normal()` accessors return the stored (normalized) values.
#[test]
fn mirror_plane_accessors() {
    let plane = MirrorPlane::new([1.0, 2.0, 3.0], [0.0, 0.0, 2.0]);
    arr_near(plane.origin(), [1.0, 2.0, 3.0]);
    // Normal (0,0,2) should be normalized to (0,0,1)
    arr_near(plane.normal(), [0.0, 0.0, 1.0]);
}

// ─────────────────────────────────────────────────────────────────────────────
// MirrorAxis
// ─────────────────────────────────────────────────────────────────────────────

/// Reflecting a point that lies ON the axis leaves it unchanged.
#[test]
fn mirror_axis_point_on_axis_is_fixed() {
    // Z axis through origin
    let axis = MirrorAxis::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let p = [0.0, 0.0, 5.0];
    arr_near(axis.reflect_point(p), p);
}

/// Reflecting across the Z axis negates X and Y.
#[test]
fn mirror_axis_z_negates_xy() {
    let axis = MirrorAxis::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let p = [3.0, 4.0, 7.0];
    arr_near(axis.reflect_point(p), [-3.0, -4.0, 7.0]);
}

/// Reflecting across the X axis negates Y and Z.
#[test]
fn mirror_axis_x_negates_yz() {
    let axis = MirrorAxis::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let p = [5.0, 2.0, -3.0];
    arr_near(axis.reflect_point(p), [5.0, -2.0, 3.0]);
}

/// Reflecting twice returns the original point (involution).
#[test]
fn mirror_axis_double_reflection_is_identity() {
    let axis = MirrorAxis::new([1.0, 0.0, 0.0], [0.0, 1.0, 1.0]);
    let p = [3.0, -2.0, 5.0];
    let once = axis.reflect_point(p);
    let twice = axis.reflect_point(once);
    arr_near(twice, p);
}

/// Axis not through origin: the offset is respected.
#[test]
fn mirror_axis_offset_origin() {
    // Z axis through (1,1,0)
    let axis = MirrorAxis::new([1.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    let p = [4.0, 5.0, 3.0];
    // The XY part reflects about (1,1): x' = 2*1 - 4 = -2, y' = 2*1 - 5 = -3
    arr_near(axis.reflect_point(p), [-2.0, -3.0, 3.0]);
}

/// `direction()` accessor returns the normalized direction.
#[test]
fn mirror_axis_accessors() {
    let axis = MirrorAxis::new([0.0, 0.0, 0.0], [3.0, 0.0, 0.0]);
    arr_near(axis.origin(), [0.0, 0.0, 0.0]);
    arr_near(axis.direction(), [1.0, 0.0, 0.0]);
}

// ─────────────────────────────────────────────────────────────────────────────
// TransformMatrix — identity
// ─────────────────────────────────────────────────────────────────────────────

/// `new()` produces an identity that leaves points unchanged.
#[test]
fn transform_matrix_identity() {
    let t = TransformMatrix::new();
    let p = [3.0, -5.0, 7.0];
    arr_near(t.apply(p), p);
}

// ─────────────────────────────────────────────────────────────────────────────
// TransformMatrix — translation
// ─────────────────────────────────────────────────────────────────────────────

/// `from_translation` shifts a point by the given vector.
#[test]
fn transform_matrix_translation() {
    let t = TransformMatrix::from_translation([10.0, -3.0, 5.0]);
    let p = [1.0, 2.0, 3.0];
    arr_near(t.apply(p), [11.0, -1.0, 8.0]);
}

// ─────────────────────────────────────────────────────────────────────────────
// TransformMatrix — scale
// ─────────────────────────────────────────────────────────────────────────────

/// `from_scale` scales uniformly about the given origin.
#[test]
fn transform_matrix_scale_about_origin() {
    let t = TransformMatrix::from_scale(2.0, [0.0, 0.0, 0.0]);
    arr_near(t.apply([3.0, 4.0, 5.0]), [6.0, 8.0, 10.0]);
}

/// `from_scale` with a non-origin center: `p' = center + s*(p - center)`.
#[test]
fn transform_matrix_scale_about_center() {
    let center = [1.0, 1.0, 1.0];
    let t = TransformMatrix::from_scale(3.0, center);
    // p = (2,3,4): p' = (1,1,1) + 3*(1,2,3) = (4,7,10)
    arr_near(t.apply([2.0, 3.0, 4.0]), [4.0, 7.0, 10.0]);
}

/// Scale factor 1 is the identity.
#[test]
fn transform_matrix_scale_one_is_identity() {
    let t = TransformMatrix::from_scale(1.0, [5.0, -3.0, 2.0]);
    let p = [7.0, 8.0, 9.0];
    arr_near(t.apply(p), p);
}

// ─────────────────────────────────────────────────────────────────────────────
// TransformMatrix — from_mirror_plane
// ─────────────────────────────────────────────────────────────────────────────

/// `from_mirror_plane` via the XY plane negates Z.
#[test]
fn transform_matrix_mirror_plane_xy() {
    let plane = MirrorPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let t = TransformMatrix::from_mirror_plane(&plane);
    arr_near(t.apply([1.0, 2.0, 3.0]), [1.0, 2.0, -3.0]);
}

/// `from_mirror_plane` result matches `MirrorPlane::reflect_point` exactly.
#[test]
fn transform_matrix_mirror_plane_matches_direct() {
    let plane = MirrorPlane::new([1.0, 2.0, 0.0], [1.0, 1.0, 0.0]);
    let t = TransformMatrix::from_mirror_plane(&plane);
    let p = [5.0, -3.0, 7.0];
    let direct = plane.reflect_point(p);
    let via_mat = t.apply(p);
    arr_near(via_mat, direct);
}

/// `from_mirror_plane` applied twice is the identity.
#[test]
fn transform_matrix_mirror_plane_double_is_identity() {
    let plane = MirrorPlane::new([0.0, 0.0, 2.0], [0.0, 1.0, 1.0]);
    let t = TransformMatrix::from_mirror_plane(&plane);
    let tt = t.compose(&t);
    let p = [1.0, 3.0, -2.0];
    arr_near(tt.apply(p), p);
}

// ─────────────────────────────────────────────────────────────────────────────
// TransformMatrix — from_mirror_axis
// ─────────────────────────────────────────────────────────────────────────────

/// `from_mirror_axis` via the Z axis negates X and Y.
#[test]
fn transform_matrix_mirror_axis_z() {
    let axis = MirrorAxis::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let t = TransformMatrix::from_mirror_axis(&axis);
    arr_near(t.apply([3.0, 4.0, 7.0]), [-3.0, -4.0, 7.0]);
}

/// `from_mirror_axis` result matches `MirrorAxis::reflect_point` exactly.
#[test]
fn transform_matrix_mirror_axis_matches_direct() {
    let axis = MirrorAxis::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let t = TransformMatrix::from_mirror_axis(&axis);
    let p = [4.0, 2.0, -5.0];
    let direct = axis.reflect_point(p);
    let via_mat = t.apply(p);
    arr_near(via_mat, direct);
}

/// `from_mirror_axis` applied twice is the identity.
#[test]
fn transform_matrix_mirror_axis_double_is_identity() {
    let axis = MirrorAxis::new([0.0, 1.0, 0.0], [1.0, 1.0, 0.0]);
    let t = TransformMatrix::from_mirror_axis(&axis);
    let tt = t.compose(&t);
    let p = [2.0, -3.0, 5.0];
    arr_near(tt.apply(p), p);
}

// ─────────────────────────────────────────────────────────────────────────────
// TransformMatrix — compose
// ─────────────────────────────────────────────────────────────────────────────

/// Translation composed with itself doubles the shift.
#[test]
fn transform_matrix_compose_translations() {
    let t = TransformMatrix::from_translation([1.0, 2.0, 3.0]);
    let tt = t.compose(&t);
    arr_near(tt.apply([0.0, 0.0, 0.0]), [2.0, 4.0, 6.0]);
}

/// Mirror-plane composed with translation: first translate, then mirror.
#[test]
fn transform_matrix_compose_mirror_then_translate() {
    // Mirror across XY plane (normal Z)
    let plane = MirrorPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let m = TransformMatrix::from_mirror_plane(&plane);
    // Translate by (0, 0, 10)
    let t = TransformMatrix::from_translation([0.0, 0.0, 10.0]);
    // composed = t * m: apply m first (negate z), then t (shift z by 10)
    let composed = t.compose(&m);
    let p = [1.0, 2.0, 3.0];
    // m(p) = (1, 2, -3); t(m(p)) = (1, 2, 7)
    arr_near(composed.apply(p), [1.0, 2.0, 7.0]);
}

/// `compose` with identity on either side is a no-op.
#[test]
fn transform_matrix_compose_with_identity() {
    let t = TransformMatrix::from_translation([5.0, -2.0, 3.0]);
    let id = TransformMatrix::new();
    let p = [1.0, 1.0, 1.0];
    arr_near(t.compose(&id).apply(p), t.apply(p));
    arr_near(id.compose(&t).apply(p), t.apply(p));
}
