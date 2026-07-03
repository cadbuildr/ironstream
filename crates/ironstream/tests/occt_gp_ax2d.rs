//! Ported OpenCascade unit tests — `gp_Ax2d`.
//!
//! Faithful Rust ports of OCCT's `gp_Ax2d` test semantics
//! (`src/FoundationClasses/TKMath/gp/gp_Ax2d.hxx`).
//! Same inputs, same expected values, same tolerances
//! (`Precision::Confusion` / `Precision::Angular`). OCCT is the reference;
//! IronStream's [`ironstream::gp_ax2d::Ax2d`] must reproduce it.

use ironstream::gp2d::Pnt2d;
use ironstream::gp_ax2d::Ax2d;
use ironstream::gp_trsf2d::Trsf2d;
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

fn near(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol:.2e}); diff = {:.2e}",
        (a - b).abs()
    );
}

fn pnt_near(p: Pnt2d, x: f64, y: f64, tol: f64) {
    near(p.x, x, tol);
    near(p.y, y, tol);
}

// ---------------------------------------------------------------------------
// 1. Constructor / accessors
// ---------------------------------------------------------------------------

/// `gp_Ax2d(P, V)` — location and direction are stored correctly; direction is
/// normalised on construction.
#[test]
fn constructor_and_accessors() {
    let ax = Ax2d::new(Pnt2d::new(3.0, 4.0), Pnt2d::new(3.0, 4.0));
    pnt_near(ax.location(), 3.0, 4.0, CONFUSION);
    // Direction must be unit-length: (3,4)/5 = (0.6, 0.8)
    near(ax.direction().x, 0.6, CONFUSION);
    near(ax.direction().y, 0.8, CONFUSION);
    near(
        ax.direction().x * ax.direction().x + ax.direction().y * ax.direction().y,
        1.0,
        CONFUSION,
    );
}

/// `SetLocation` and `SetDirection` mutate independently.
#[test]
fn set_location_and_direction() {
    let mut ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    ax.set_location(Pnt2d::new(5.0, -3.0));
    pnt_near(ax.location(), 5.0, -3.0, CONFUSION);
    pnt_near(ax.direction(), 1.0, 0.0, CONFUSION);

    ax.set_direction(Pnt2d::new(0.0, 2.0)); // non-unit; should normalise
    pnt_near(ax.direction(), 0.0, 1.0, CONFUSION);
    pnt_near(ax.location(), 5.0, -3.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 2. Translate / Translated
// ---------------------------------------------------------------------------

/// `Translated(V)` shifts only the location; direction is unchanged.
#[test]
fn translated_by_vector() {
    let ax = Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0));
    let t = ax.translated(Pnt2d::new(3.0, -5.0));
    pnt_near(t.location(), 4.0, -3.0, CONFUSION);
    pnt_near(t.direction(), 1.0, 0.0, CONFUSION);
}

/// `Translated(P1, P2)` shifts by the vector P1 → P2.
#[test]
fn translated_by_two_points() {
    let ax = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 1.0));
    let t = ax.translated_2pts(Pnt2d::new(1.0, 1.0), Pnt2d::new(4.0, 3.0));
    pnt_near(t.location(), 3.0, 2.0, CONFUSION);
    pnt_near(t.direction(), 0.0, 1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 3. Rotate / Rotated
// ---------------------------------------------------------------------------

/// Rotating a horizontal axis 90° CCW about the origin gives a vertical axis.
#[test]
fn rotated_90_about_origin() {
    let ax = Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
    let r = ax.rotated(Pnt2d::origin(), FRAC_PI_2);
    pnt_near(r.location(), 0.0, 1.0, CONFUSION);
    pnt_near(r.direction(), 0.0, 1.0, CONFUSION);
}

/// Rotating 180° negates both location and direction.
#[test]
fn rotated_180_about_origin() {
    let ax = Ax2d::new(Pnt2d::new(2.0, 0.0), Pnt2d::new(1.0, 0.0));
    let r = ax.rotated(Pnt2d::origin(), PI);
    near(r.location().x, -2.0, CONFUSION);
    near(r.location().y, 0.0, CONFUSION);
    near(r.direction().x, -1.0, CONFUSION);
    near(r.direction().y, 0.0, CONFUSION);
}

/// Rotating 45° produces the expected diagonal direction.
#[test]
fn rotated_45_about_center() {
    let ax = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    // Rotate about (1,0)
    let r = ax.rotated(Pnt2d::new(1.0, 0.0), FRAC_PI_4);
    let root2_inv = 1.0 / 2.0_f64.sqrt();
    near(r.direction().x, root2_inv, CONFUSION);
    near(r.direction().y, root2_inv, CONFUSION);
    // rel = (0,0) - (1,0) = (-1, 0)
    // rotated 45°: (-r2_inv, -r2_inv)
    // new location: (1,0) + (-r2_inv, -r2_inv) = (1 - r2_inv, -r2_inv)
    near(r.location().x, 1.0 - root2_inv, CONFUSION);
    near(r.location().y, -root2_inv, CONFUSION);
}

// ---------------------------------------------------------------------------
// 4. Mirror / Mirrored (point)
// ---------------------------------------------------------------------------

/// Point-mirror through origin inverts location and reverses direction.
#[test]
fn mirrored_through_point() {
    let ax = Ax2d::new(Pnt2d::new(3.0, 1.0), Pnt2d::new(1.0, 0.0));
    let m = ax.mirrored_point(Pnt2d::origin());
    pnt_near(m.location(), -3.0, -1.0, CONFUSION);
    pnt_near(m.direction(), -1.0, 0.0, CONFUSION);
}

/// Point-mirror through a non-origin point.
#[test]
fn mirrored_through_nonorigin_point() {
    let ax = Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(0.0, 1.0));
    let m = ax.mirrored_point(Pnt2d::new(2.0, 0.0));
    // loc' = 2*(2,0) - (1,0) = (3,0)
    pnt_near(m.location(), 3.0, 0.0, CONFUSION);
    // direction flips sign
    pnt_near(m.direction(), 0.0, -1.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 5. Mirror / Mirrored (axis)
// ---------------------------------------------------------------------------

/// Reflecting across the X axis (Y=0) negates the Y component of location and
/// direction.
#[test]
fn mirrored_across_x_axis() {
    let x_axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let ax = Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(0.0, 1.0));
    let m = ax.mirrored_axis(&x_axis);
    pnt_near(m.location(), 2.0, -3.0, CONFUSION);
    pnt_near(m.direction(), 0.0, -1.0, CONFUSION);
}

/// Reflecting across the Y axis negates the X component.
#[test]
fn mirrored_across_y_axis() {
    let y_axis = Ax2d::new(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
    let ax = Ax2d::new(Pnt2d::new(2.0, 3.0), Pnt2d::new(1.0, 0.0));
    let m = ax.mirrored_axis(&y_axis);
    pnt_near(m.location(), -2.0, 3.0, CONFUSION);
    pnt_near(m.direction(), -1.0, 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 6. Scale / Scaled
// ---------------------------------------------------------------------------

/// Scaling by 2 about origin doubles the location; direction unchanged.
#[test]
fn scaled_by_2_about_origin() {
    let ax = Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0));
    let s = ax.scaled(Pnt2d::origin(), 2.0);
    pnt_near(s.location(), 2.0, 4.0, CONFUSION);
    pnt_near(s.direction(), 1.0, 0.0, CONFUSION);
}

/// Scaling by a negative factor reverses the direction and maps location.
#[test]
fn scaled_negative_reverses_direction() {
    let ax = Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
    let s = ax.scaled(Pnt2d::origin(), -1.0);
    pnt_near(s.location(), -1.0, 0.0, CONFUSION);
    pnt_near(s.direction(), -1.0, 0.0, CONFUSION);
}

// ---------------------------------------------------------------------------
// 7. Transform / Transformed
// ---------------------------------------------------------------------------

/// Applying a 90° rotation transform via `Trsf2d` gives same result as
/// `Rotated`.
#[test]
fn transformed_matches_rotated() {
    let ax = Ax2d::new(Pnt2d::new(1.0, 0.0), Pnt2d::new(1.0, 0.0));
    let mut t = Trsf2d::identity();
    t.set_rotation(Pnt2d::origin(), FRAC_PI_2);
    let via_trsf = ax.transformed(&t);
    let via_rotated = ax.rotated(Pnt2d::origin(), FRAC_PI_2);
    pnt_near(via_trsf.location(), via_rotated.location().x, via_rotated.location().y, CONFUSION);
    pnt_near(via_trsf.direction(), via_rotated.direction().x, via_rotated.direction().y, CONFUSION);
}

// ---------------------------------------------------------------------------
// 8. IsCoaxial
// ---------------------------------------------------------------------------

/// Two axes on the same line are coaxial.
#[test]
fn is_coaxial_same_line() {
    let a = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let b = Ax2d::new(Pnt2d::new(5.0, 0.0), Pnt2d::new(1.0, 0.0));
    assert!(a.is_coaxial(&b, CONFUSION, ANGULAR));
}

/// Two parallel but offset axes are NOT coaxial.
#[test]
fn is_coaxial_parallel_but_offset() {
    let a = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let b = Ax2d::new(Pnt2d::new(0.0, 1.0), Pnt2d::new(1.0, 0.0));
    assert!(!a.is_coaxial(&b, CONFUSION, ANGULAR));
}

// ---------------------------------------------------------------------------
// 9. IsNormal / IsOpposite / IsParallel
// ---------------------------------------------------------------------------

/// Perpendicular axes are normal; parallel same-direction axes are not.
#[test]
fn direction_predicates() {
    let horiz = Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0));
    let vert = Ax2d::new(Pnt2d::origin(), Pnt2d::new(0.0, 1.0));
    let horiz_neg = Ax2d::new(Pnt2d::new(3.0, 0.0), Pnt2d::new(-1.0, 0.0));

    // IsNormal
    assert!(horiz.is_normal(&vert, ANGULAR));
    assert!(!horiz.is_normal(&horiz, ANGULAR));

    // IsOpposite
    assert!(horiz.is_opposite(&horiz_neg, ANGULAR));
    assert!(!horiz.is_opposite(&vert, ANGULAR));

    // IsParallel (covers both same and opposite)
    assert!(horiz.is_parallel(&horiz, ANGULAR));
    assert!(horiz.is_parallel(&horiz_neg, ANGULAR));
    assert!(!horiz.is_parallel(&vert, ANGULAR));
}

// ---------------------------------------------------------------------------
// 10. In-place vs functional consistency
// ---------------------------------------------------------------------------

/// In-place `Translate` / `Rotate` / `Mirror` / `Scale` produce the same
/// result as their functional counterparts.
#[test]
fn inplace_matches_functional() {
    let orig = Ax2d::new(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0));
    let v = Pnt2d::new(3.0, -1.0);

    // Translate
    let functional = orig.translated(v);
    let mut inplace = orig;
    inplace.translate(v);
    pnt_near(inplace.location(), functional.location().x, functional.location().y, CONFUSION);

    // Rotate
    let functional = orig.rotated(Pnt2d::new(1.0, 1.0), FRAC_PI_4);
    let mut inplace = orig;
    inplace.rotate(Pnt2d::new(1.0, 1.0), FRAC_PI_4);
    pnt_near(inplace.location(), functional.location().x, functional.location().y, CONFUSION);
    pnt_near(inplace.direction(), functional.direction().x, functional.direction().y, CONFUSION);

    // Mirror point
    let functional = orig.mirrored_point(Pnt2d::new(2.0, 3.0));
    let mut inplace = orig;
    inplace.mirror_point(Pnt2d::new(2.0, 3.0));
    pnt_near(inplace.location(), functional.location().x, functional.location().y, CONFUSION);
    pnt_near(inplace.direction(), functional.direction().x, functional.direction().y, CONFUSION);

    // Scale
    let functional = orig.scaled(Pnt2d::new(0.0, 0.0), 2.5);
    let mut inplace = orig;
    inplace.scale(Pnt2d::new(0.0, 0.0), 2.5);
    pnt_near(inplace.location(), functional.location().x, functional.location().y, CONFUSION);
    pnt_near(inplace.direction(), functional.direction().x, functional.direction().y, CONFUSION);
}
