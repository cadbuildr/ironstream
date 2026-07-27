//! Ported OpenCascade unit tests — `Geom_OffsetCurve`.
//!
//! Faithful Rust port of the OCCT `Geom_OffsetCurve` tests
//! (Open-Cascade-SAS/OCCT, package `TKG3d`). Same numeric inputs, same
//! expected values, same tolerances (`Precision::Confusion` → 1e-7).
//!
//! OCCT helper-type mapping:
//! - `gp_Pnt`                  → [`ironstream::gp::Pnt`]
//! - `gp_Vec`                  → [`ironstream::gp::Vec3`]
//! - `gp_Dir`                  → [`ironstream::gp::Pnt`] (unit vector)
//! - `gp_Ax2`                  → [`ironstream::gp::Ax3`]
//! - `gp_Trsf`                 → [`ironstream::gp::Trsf`]
//! - `Handle(Geom_Circle)`     → [`ironstream::geom_circle::GeomCircle`]
//! - `Handle(Geom_Line)`       → [`ironstream::geom_line::GeomLine`]
//! - `Handle(Geom_OffsetCurve)` → [`ironstream::geom_offset_curve::GeomOffsetCurve`]

use ironstream::geom_circle::GeomCircle;
use ironstream::geom_ellipse::GeomEllipse;
use ironstream::geom_line::GeomLine;
use ironstream::geom_offset_curve::{BasisCurve, GeomOffsetCurve};
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision::{ANGULAR, CONFUSION};
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

fn near_pnt(p: Pnt, q: Pnt, tol: f64) {
    assert!(
        p.distance(q) <= tol,
        "expected {p:?} ≈ {q:?} (dist {} > tol {tol})",
        p.distance(q)
    );
}

/// Build a right-handed coordinate system from an origin and a main (Z)
/// direction, using the same convention as OCCT's `gp_Ax2(P, D)`.
fn ax2(loc: Pnt, n: Pnt) -> Ax3 {
    let v = n.normalized();
    let (a, b, c) = (v.x, v.y, v.z);
    let (aa, ba, ca) = (a.abs(), b.abs(), c.abs());
    let xdir = if ba <= aa && ba <= ca {
        if aa > ca {
            Pnt::new(-c, 0.0, a)
        } else {
            Pnt::new(c, 0.0, -a)
        }
    } else if aa <= ba && aa <= ca {
        if ba > ca {
            Pnt::new(0.0, -c, b)
        } else {
            Pnt::new(0.0, c, -b)
        }
    } else if aa > ba {
        Pnt::new(-b, a, 0.0)
    } else {
        Pnt::new(b, -a, 0.0)
    };
    Ax3::from_origin_normal(loc, v, xdir)
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 1: Constructor and basic accessors
// ─────────────────────────────────────────────────────────────────────────────

/// `Geom_OffsetCurve(C, Offset, V)` — verify the offset and direction are
/// stored correctly.
#[test]
fn constructor_and_accessors() {
    // Build a circle in the XY plane, center at origin, radius 3.
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 3.0);

    let offset_dist = 1.0;
    let ref_dir = Pnt::new(0.0, 0.0, 1.0); // along Z
    let oc = GeomOffsetCurve::new(BasisCurve::Circle(circle), offset_dist, ref_dir);

    near(oc.offset(), offset_dist, CONFUSION);
    // direction should be stored as a unit Z vector
    let d = oc.direction();
    near(d.x, 0.0, CONFUSION);
    near(d.y, 0.0, CONFUSION);
    near(d.z, 1.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 2: Value / D0 on offset circle
// ─────────────────────────────────────────────────────────────────────────────

/// For a circle of radius R in the XY plane offset by `d` along Z (so the
/// reference direction is Z), the offset normal is `normalize(Z × T)`.
///
/// At u = 0 the tangent of the unit circle in XY is `(0, R, 0)` → `(0, 1, 0)`.
/// `Z × (0, 1, 0) = (-1, 0, 0)` so the offset point is `(R - d, 0, 0)`.
#[test]
fn value_on_circle_offset() {
    let r = 5.0;
    let d = 1.0;
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    let ref_dir = Pnt::new(0.0, 0.0, 1.0);
    let oc = GeomOffsetCurve::new(BasisCurve::Circle(circle), d, ref_dir);

    // u = 0: base point (R, 0, 0), tangent (0, R, 0), normal = Z×T/|Z×T| = (-1,0,0)
    let p0 = oc.value(0.0);
    near_pnt(p0, Pnt::new(r - d, 0.0, 0.0), CONFUSION);

    // u = PI/2: base point (0, R, 0), tangent (-R, 0, 0)
    // Z × (-1, 0, 0) = (0, -1, 0) → offset: (0, R+d, 0)   (check sign)
    // actually Z×(-1,0,0) = (0*0 - 1*0, 1*(-1) - 0*0, 0*0 - 0*(-1)) = (0,-1,0)
    // offset point = (0, R, 0) + d*(0,-1,0) = (0, R-d, 0)
    let p1 = oc.value(PI / 2.0);
    near_pnt(p1, Pnt::new(0.0, r - d, 0.0), CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 3: Parameter range mirrors basis curve
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn parameter_range_from_circle() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 2.0);
    let oc = GeomOffsetCurve::new(
        BasisCurve::Circle(circle),
        0.5,
        Pnt::new(0.0, 0.0, 1.0),
    );
    near(oc.first_parameter(), 0.0, CONFUSION);
    near(oc.last_parameter(), 2.0 * PI, CONFUSION);
}

#[test]
fn parameter_range_from_line() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let oc = GeomOffsetCurve::new(
        BasisCurve::Line(line),
        1.0,
        Pnt::new(0.0, 0.0, 1.0),
    );
    // Line's first/last are ±INFINITE; just check the sign / magnitude.
    assert!(oc.first_parameter() < -1e100);
    assert!(oc.last_parameter() > 1e100);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 4: IsClosed / IsPeriodic
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn is_closed_and_periodic_circle() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 4.0);
    let oc = GeomOffsetCurve::new(
        BasisCurve::Circle(circle),
        0.5,
        Pnt::new(0.0, 0.0, 1.0),
    );
    assert!(oc.is_periodic(), "circle offset should be periodic");
    assert!(oc.is_closed(), "circle offset should be closed");
}

#[test]
fn is_not_closed_line() {
    let line = GeomLine::new(Ax1::new(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0)));
    let oc = GeomOffsetCurve::new(
        BasisCurve::Line(line),
        1.0,
        Pnt::new(0.0, 0.0, 1.0),
    );
    assert!(!oc.is_closed());
    assert!(!oc.is_periodic());
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 5: D1 — first derivative
// ─────────────────────────────────────────────────────────────────────────────

/// For a circle of radius R, the tangent at u=0 is (0, R, 0).
/// The offset tangent Q'(u) = C'(u) + d * N'(u).
/// We verify the magnitude makes sense and numerical check via finite diffs.
#[test]
fn d1_circle_numerical_check() {
    let r = 3.0;
    let d = 0.5;
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    let ref_dir = Pnt::new(0.0, 0.0, 1.0);
    let oc = GeomOffsetCurve::new(BasisCurve::Circle(circle), d, ref_dir);

    let u = 1.0;
    let (_, v1_analytic) = oc.d1(u);

    // Finite-difference check.
    let h = 1e-6;
    let p_plus = oc.value(u + h);
    let p_minus = oc.value(u - h);
    let v1_fd = (p_plus - p_minus) * (1.0 / (2.0 * h));

    near(v1_analytic.x, v1_fd.x, 1e-5);
    near(v1_analytic.y, v1_fd.y, 1e-5);
    near(v1_analytic.z, v1_fd.z, 1e-5);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 6: D2 — second derivative (numerical check)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d2_circle_numerical_check() {
    let r = 4.0;
    let d = 0.3;
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    let ref_dir = Pnt::new(0.0, 0.0, 1.0);
    let oc = GeomOffsetCurve::new(BasisCurve::Circle(circle), d, ref_dir);

    let u = 0.7;
    let (_, _, v2_analytic) = oc.d2(u);

    // Finite-difference D2 from D1.
    let h = 1e-5;
    let (_, v1_plus) = oc.d1(u + h);
    let (_, v1_minus) = oc.d1(u - h);
    let v2_fd = (v1_plus - v1_minus) * (1.0 / (2.0 * h));

    near(v2_analytic.x, v2_fd.x, 1e-4);
    near(v2_analytic.y, v2_fd.y, 1e-4);
    near(v2_analytic.z, v2_fd.z, 1e-4);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 7: D3 — third derivative (numerical check)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d3_circle_numerical_check() {
    let r = 2.0;
    let d = 0.2;
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    let ref_dir = Pnt::new(0.0, 0.0, 1.0);
    let oc = GeomOffsetCurve::new(BasisCurve::Circle(circle), d, ref_dir);

    let u = 1.2;
    let (_, _, _, v3_analytic) = oc.d3(u);

    // Finite-difference D3 from D2.
    let h = 1e-4;
    let (_, _, v2_plus) = oc.d2(u + h);
    let (_, _, v2_minus) = oc.d2(u - h);
    let v3_fd = (v2_plus - v2_minus) * (1.0 / (2.0 * h));

    near(v3_analytic.x, v3_fd.x, 1e-3);
    near(v3_analytic.y, v3_fd.y, 1e-3);
    near(v3_analytic.z, v3_fd.z, 1e-3);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 8: DN delegate to D1/D2/D3
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn dn_delegates_correctly() {
    let r = 6.0;
    let d = 1.0;
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, r);
    let ref_dir = Pnt::new(0.0, 0.0, 1.0);
    let oc = GeomOffsetCurve::new(BasisCurve::Circle(circle), d, ref_dir);

    let u = 0.5;
    let d1 = oc.d1(u).1;
    let d2 = oc.d2(u).2;
    let d3 = oc.d3(u).3;

    near_pnt(oc.dn(u, 1), d1, CONFUSION);
    near_pnt(oc.dn(u, 2), d2, CONFUSION);
    near_pnt(oc.dn(u, 3), d3, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 9: Reverse negates offset distance
// ─────────────────────────────────────────────────────────────────────────────

/// Reversing changes the sign of the offset distance.
#[test]
fn reverse_negates_offset() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 5.0);
    let d = 2.0;
    let mut oc = GeomOffsetCurve::new(
        BasisCurve::Circle(circle),
        d,
        Pnt::new(0.0, 0.0, 1.0),
    );
    oc.reverse();
    near(oc.offset(), -d, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 10: Transform scales offset
// ─────────────────────────────────────────────────────────────────────────────

/// Applying a uniform scale `s` to the offset curve scales the offset by `s`.
#[test]
fn transform_scales_offset() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 3.0);
    let d = 1.0;
    let mut oc = GeomOffsetCurve::new(
        BasisCurve::Circle(circle),
        d,
        Pnt::new(0.0, 0.0, 1.0),
    );

    let scale = 2.0;
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), scale);
    oc.transform(&t);

    near(oc.offset(), d * scale, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 11: Line offset — value check
// ─────────────────────────────────────────────────────────────────────────────

/// Offset a line along X by distance 1 using Z as reference direction.
///
/// Line: `P(u) = (u, 0, 0)`, tangent `T = (1, 0, 0)`.
/// `Z × T = (0, 0, 1) × (1, 0, 0) = (0, 1, 0)` → offset point `(u, d, 0)`.
#[test]
fn value_on_line_offset() {
    let line = GeomLine::from_point_dir(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
    let d = 2.0;
    let oc = GeomOffsetCurve::new(
        BasisCurve::Line(line),
        d,
        Pnt::new(0.0, 0.0, 1.0),
    );

    let u = 5.0;
    let p = oc.value(u);
    near(p.x, u, CONFUSION);
    near(p.y, d, CONFUSION);
    near(p.z, 0.0, CONFUSION);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 12: Ellipse offset — parameter range
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ellipse_offset_parameter_range() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let ellipse = GeomEllipse::from_ax3(axis, 5.0, 3.0);
    let oc = GeomOffsetCurve::new(
        BasisCurve::Ellipse(ellipse),
        0.5,
        Pnt::new(0.0, 0.0, 1.0),
    );
    near(oc.first_parameter(), 0.0, CONFUSION);
    near(oc.last_parameter(), 2.0 * PI, CONFUSION);
    assert!(oc.is_periodic());
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 13: Direction is normalised even for non-unit input
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn direction_is_normalised() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 2.0);
    // Pass a non-unit vector; constructor must normalise it.
    let raw = Pnt::new(0.0, 0.0, 3.7);
    let oc = GeomOffsetCurve::new(BasisCurve::Circle(circle), 1.0, raw);
    let dir = oc.direction();
    near((dir.norm() - 1.0).abs(), 0.0, ANGULAR.max(1e-12));
    near(dir.z, 1.0, 1e-12);
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 14: BasisCurve accessor
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn basis_curve_accessor() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 7.0);
    let oc = GeomOffsetCurve::new(
        BasisCurve::Circle(circle),
        1.5,
        Pnt::new(0.0, 0.0, 1.0),
    );
    // The wrapped basis curve should be a Circle variant.
    assert!(matches!(oc.basis_curve(), BasisCurve::Circle(_)));
}

// ─────────────────────────────────────────────────────────────────────────────
// TEST 15: Transformed returns a correctly offset copy
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn transformed_copy_is_independent() {
    let axis = ax2(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let circle = GeomCircle::from_ax3(axis, 4.0);
    let oc = GeomOffsetCurve::new(
        BasisCurve::Circle(circle),
        1.0,
        Pnt::new(0.0, 0.0, 1.0),
    );
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 3.0);

    let oc2 = oc.transformed(&t);
    // Original unchanged.
    near(oc.offset(), 1.0, CONFUSION);
    // Copy has scaled offset.
    near(oc2.offset(), 3.0, CONFUSION);
}
