//! Ported OpenCascade unit tests -- `Geom2d_Hyperbola`.
//!
//! These tests exercise the analytic 2D hyperbola defined in
//! `ironstream::geom2d_hyperbola` and mirror the checks performed by
//! OCCT's own test suite for `Geom2d_Hyperbola` / `gp_Hypr2d`.

use ironstream::geom2d_circle::Ax22d2;
use ironstream::geom2d_hyperbola::{Geom2dHyperbola, Hypr2d};
use ironstream::gp2d::{Ax2d, Pnt2d, Trsf2d};
use ironstream::precision::{ANGULAR, CONFUSION};

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Standard hyperbola: a=3, b=4, c=5, e=5/3, centered at origin, X major axis.
fn h34() -> Geom2dHyperbola {
    Geom2dHyperbola::new(
        Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
        3.0,
        4.0,
        true,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

/// Construction via `Hypr2d` value type and retrieval of basic properties.
#[test]
fn construct_from_hypr2d() {
    let axis = Ax22d2::from_x_axis(Pnt2d::new(1.0, 2.0), Pnt2d::new(1.0, 0.0), true);
    let h = Geom2dHyperbola::from_hypr2d(Hypr2d::from_axis(axis, 5.0, 3.0));
    assert!((h.major_radius() - 5.0).abs() < CONFUSION);
    assert!((h.minor_radius() - 3.0).abs() < CONFUSION);
    assert!((h.location().x - 1.0).abs() < CONFUSION);
    assert!((h.location().y - 2.0).abs() < CONFUSION);
}

/// At `U = 0`, the point lies on the +X vertex: `P = (a, 0)`.
#[test]
fn vertex_at_u_zero() {
    let h = h34();
    let p = h.value(0.0);
    assert!((p.x - 3.0).abs() < CONFUSION, "x={}", p.x);
    assert!(p.y.abs() < CONFUSION, "y={}", p.y);
}

/// Eccentricity for the 3-4-5 hyperbola must be 5/3.
#[test]
fn eccentricity_three_four_five() {
    let h = h34();
    assert!((h.eccentricity() - 5.0 / 3.0).abs() < CONFUSION);
}

/// Focus positions: F1 = (c, 0), F2 = (−c, 0) with c = sqrt(a²+b²).
#[test]
fn focus_positions() {
    let h = h34(); // c = 5
    let f1 = h.focus1();
    let f2 = h.focus2();
    assert!((f1.x - 5.0).abs() < CONFUSION && f1.y.abs() < CONFUSION, "f1={f1:?}");
    assert!((f2.x + 5.0).abs() < CONFUSION && f2.y.abs() < CONFUSION, "f2={f2:?}");
}

/// `Focal()` returns the full inter-focal distance 2c.
#[test]
fn focal_distance() {
    let h = h34();
    assert!((h.focal() - 10.0).abs() < CONFUSION);
}

/// `Parameter()` = b² / a for a hyperbola.
#[test]
fn semi_latus_rectum() {
    let h = h34(); // 16/3
    assert!((h.parameter() - 16.0 / 3.0).abs() < CONFUSION);
}

/// A hyperbola is open and non-periodic.
#[test]
fn not_closed_not_periodic() {
    let h = h34();
    assert!(!h.is_closed());
    assert!(!h.is_periodic());
}

/// `reversed_parameter(U)` is `−U`.
#[test]
fn reversed_parameter_is_negation() {
    let h = h34();
    let u = 1.23_f64;
    assert!((h.reversed_parameter(u) + u).abs() < ANGULAR);
}

/// `D0(U)` satisfies the hyperbola equation `(x/a)² − (y/b)² = 1`.
#[test]
fn hyperbola_equation_at_various_u() {
    let h = h34();
    let a = h.major_radius();
    let b = h.minor_radius();
    for u in [-3.0_f64, -1.5, -0.5, 0.0, 0.5, 1.5, 3.0] {
        let p = h.value(u);
        let residual = (p.x / a).powi(2) - (p.y / b).powi(2) - 1.0;
        assert!(residual.abs() < 1e-10, "u={u}, residual={residual}");
    }
}

/// First derivative at `U = 0` is `(0, b)`.
#[test]
fn d1_at_u_zero() {
    let h = h34();
    let (_p, v1) = h.d1(0.0);
    // sinh(0)=0, cosh(0)=1 => V1 = (0, b) = (0, 4)
    assert!(v1.x.abs() < CONFUSION, "v1.x={}", v1.x);
    assert!((v1.y - 4.0).abs() < CONFUSION, "v1.y={}", v1.y);
}

/// Second derivative equals the displacement from center (P − O).
#[test]
fn d2_equals_p_minus_center() {
    let h = h34();
    for u in [-1.0_f64, 0.0, 1.0, 2.0] {
        let (_p, _v1, v2) = h.d2(u);
        let offset = h.value(u) - h.location();
        assert!((v2.x - offset.x).abs() < CONFUSION, "u={u}");
        assert!((v2.y - offset.y).abs() < CONFUSION, "u={u}");
    }
}

/// Odd-order derivatives are identical (derivative cycling).
#[test]
fn dn_odd_cycling() {
    let h = h34();
    let u = 1.1_f64;
    let d1 = h.dn(u, 1);
    let d3 = h.dn(u, 3);
    assert!((d1.x - d3.x).abs() < CONFUSION && (d1.y - d3.y).abs() < CONFUSION);
}

/// Even-order derivatives are identical (derivative cycling).
#[test]
fn dn_even_cycling() {
    let h = h34();
    let u = 0.8_f64;
    let d2 = h.dn(u, 2);
    let d4 = h.dn(u, 4);
    assert!((d2.x - d4.x).abs() < CONFUSION && (d2.y - d4.y).abs() < CONFUSION);
}

/// Asymptote directions for the 3-4-5 hyperbola: `(3,±4)/5 = (0.6, ±0.8)`.
#[test]
fn asymptote_directions() {
    let h = h34();
    let a1 = h.asymptote1(); // direction (0.6, 0.8)
    let a2 = h.asymptote2(); // direction (0.6, -0.8)
    assert!((a1.direction.x - 0.6).abs() < CONFUSION, "{:?}", a1.direction);
    assert!((a1.direction.y - 0.8).abs() < CONFUSION, "{:?}", a1.direction);
    assert!((a2.direction.x - 0.6).abs() < CONFUSION, "{:?}", a2.direction);
    assert!((a2.direction.y + 0.8).abs() < CONFUSION, "{:?}", a2.direction);
}

/// Pure translation leaves radii unchanged and shifts the center.
#[test]
fn transform_translation() {
    let h = h34().transformed(&Trsf2d::translation(Pnt2d::new(5.0, -3.0)));
    assert!((h.location().x - 5.0).abs() < CONFUSION);
    assert!((h.location().y + 3.0).abs() < CONFUSION);
    assert!((h.major_radius() - 3.0).abs() < CONFUSION);
    assert!((h.minor_radius() - 4.0).abs() < CONFUSION);
}

/// Uniform scaling by factor 2 doubles both radii.
#[test]
fn transform_scale_doubles_radii() {
    let scale = 2.0_f64;
    let t = Trsf2d {
        m: [[scale, 0.0], [0.0, scale]],
        t: [0.0, 0.0],
    };
    let h = h34().transformed(&t);
    assert!((h.major_radius() - 6.0).abs() < CONFUSION);
    assert!((h.minor_radius() - 8.0).abs() < CONFUSION);
}

/// `Reverse` flips the Y direction; the point at `−U` on the reversed curve
/// matches the point at `U` on the original (X coordinate is the same;
/// Y coordinate is negated because the Y axis is flipped).
#[test]
fn reverse_preserves_shape() {
    let h = h34();
    let u = 1.0_f64;
    let p_orig = h.value(u);
    let rev = h.reversed();
    // On the reversed hyperbola (Y axis negated), P_rev(-U) should equal P_orig(U)
    let p_rev = rev.value(-u);
    assert!((p_orig.x - p_rev.x).abs() < CONFUSION, "{} vs {}", p_orig.x, p_rev.x);
    assert!((p_orig.y - p_rev.y).abs() < CONFUSION, "{} vs {}", p_orig.y, p_rev.y);
}

/// `SetMajorRadius` / `SetMinorRadius` mutate in place.
#[test]
fn set_radii() {
    let mut h = h34();
    h.set_major_radius(10.0);
    h.set_minor_radius(6.0);
    assert!((h.major_radius() - 10.0).abs() < CONFUSION);
    assert!((h.minor_radius() - 6.0).abs() < CONFUSION);
}

/// Directrix 1 is at `a²/c = 9/5 = 1.8` from the center along +X.
#[test]
fn directrix1_position() {
    let h = h34(); // a=3, b=4, c=5, d=9/5=1.8
    let dir1 = h.directrix1();
    assert!((dir1.location.x - 1.8).abs() < CONFUSION, "x={}", dir1.location.x);
    assert!(dir1.location.y.abs() < CONFUSION, "y={}", dir1.location.y);
    // Direction of the directrix line is along Y axis
    assert!(dir1.direction.x.abs() < CONFUSION);
    assert!((dir1.direction.y.abs() - 1.0).abs() < CONFUSION);
}

/// `Copy()` produces an independent clone.
#[test]
fn copy_is_independent() {
    let h = h34();
    let mut c = h.copy();
    c.set_major_radius(99.0);
    assert!((h.major_radius() - 3.0).abs() < CONFUSION); // original unchanged
}
