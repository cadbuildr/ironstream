//! Ported OCCT GTest `Geom2d_BSplineCurve_Test.cxx`.
//!
//! Faithful Rust port of OpenCascade's `Geom2d_BSplineCurve` unit tests:
//! same numeric inputs, same expected values, same tolerances. `gp_Pnt2d`/
//! `gp_Vec2d` map to `ironstream::gp2d::{Pnt2d, Vec2d}`, `gp_Trsf2d` to
//! `ironstream::gp2d::Trsf2d`, `Handle(Geom2d_BSplineCurve)` to the value.
//! `Precision::Confusion` maps to `ironstream::precision::CONFUSION`.

use ironstream::gp2d::{Pnt2d, Trsf2d, Vec2d};
use ironstream::geom2d_bspline::Geom2dBSplineCurve;

/// `gp_Pnt2d::IsEqual(other, tol)` — coincidence within `tol`.
fn is_equal(a: Pnt2d, b: Pnt2d, tol: f64) -> bool {
    a.distance(b) <= tol
}

/// The shared fixture curve (`SetUp()`): a cubic non-rational B-spline with
/// 4 poles and end-clamped knots {0,1} of multiplicity {4,4}.
fn original_curve() -> Geom2dBSplineCurve {
    let poles = [
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(2.0, 1.0),
        Pnt2d::new(3.0, 0.0),
    ];
    let knots = [0.0, 1.0];
    let mults = [4, 4];
    Geom2dBSplineCurve::new(&poles, &knots, &mults, 3, false)
}

#[test]
fn copy_constructor_basic_properties() {
    let original = original_curve();
    let copied = original.copy();

    assert_eq!(original.degree(), copied.degree());
    assert_eq!(original.nb_poles(), copied.nb_poles());
    assert_eq!(original.nb_knots(), copied.nb_knots());
    assert_eq!(original.is_periodic(), copied.is_periodic());
    assert_eq!(original.is_rational(), copied.is_rational());
}

#[test]
fn copy_constructor_poles() {
    let original = original_curve();
    let copied = original.copy();

    for i in 1..=original.nb_poles() {
        let orig_pole = original.pole(i);
        let copy_pole = copied.pole(i);
        assert!(is_equal(orig_pole, copy_pole, 1e-10));
    }
}

#[test]
fn copy_constructor_knots() {
    let original = original_curve();
    let copied = original.copy();

    for i in 1..=original.nb_knots() {
        assert_eq!(original.knot(i), copied.knot(i));
        assert_eq!(original.multiplicity(i), copied.multiplicity(i));
    }
}

#[test]
fn copy_method_uses_optimized_constructor() {
    let original = original_curve();
    let copied = original.copy();

    assert_eq!(original.degree(), copied.degree());
    assert_eq!(original.nb_poles(), copied.nb_poles());

    // Test evaluation at several points.
    let mut u = 0.0;
    while u <= 1.0 + 1e-12 {
        let orig_pnt = original.value(u);
        let copy_pnt = copied.value(u);
        assert!(is_equal(orig_pnt, copy_pnt, 1e-10));
        u += 0.25;
    }
}

#[test]
fn rational_curve_copy_constructor() {
    let poles = [
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(2.0, 0.0),
    ];
    let weights = [1.0, 2.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let rational = Geom2dBSplineCurve::with_weights(&poles, &weights, &knots, &mults, 2, false);
    let copied = rational.copy();

    assert!(copied.is_rational());

    for i in 1..=rational.nb_poles() {
        assert_eq!(rational.weight(i), copied.weight(i));
    }
}

#[test]
fn evaluation_d0() {
    let curve = original_curve();
    let p0 = curve.d0(0.0);
    assert!(is_equal(p0, Pnt2d::new(0.0, 0.0), 1e-10));
    let p1 = curve.d0(1.0);
    assert!(is_equal(p1, Pnt2d::new(3.0, 0.0), 1e-10));
}

#[test]
fn evaluation_d1() {
    let curve = original_curve();
    let (_p, v1) = curve.d1(0.5);
    assert!(v1.norm() > 0.0);
}

#[test]
fn evaluation_d2() {
    let curve = original_curve();
    let (_p, v1, _v2) = curve.d2(0.5);
    assert!(v1.norm() > 0.0);
}

#[test]
fn evaluation_dn() {
    let curve = original_curve();
    let dn1 = curve.dn(0.5, 1);
    assert!(dn1.norm() > 0.0);
    // DN(4) for degree 3 should be zero.
    let dn4 = curve.dn(0.5, 4);
    assert!((dn4.norm() - 0.0).abs() < 1e-10);
}

#[test]
fn start_end_points() {
    let curve = original_curve();
    let start = curve.start_point();
    let end = curve.end_point();
    assert!(is_equal(start, Pnt2d::new(0.0, 0.0), 1e-10));
    assert!(is_equal(end, Pnt2d::new(3.0, 0.0), 1e-10));
}

#[test]
fn properties() {
    let curve = original_curve();
    assert_eq!(curve.degree(), 3);
    assert!(!curve.is_periodic());
    assert!(!curve.is_rational());
    assert!(!curve.is_closed());
    assert!(curve.is_cn(3));
}

#[test]
fn set_pole() {
    let mut curve = original_curve();
    let new_pole = Pnt2d::new(1.0, 5.0);
    curve.set_pole(2, new_pole);
    assert!(is_equal(curve.pole(2), new_pole, 1e-10));
}

#[test]
fn increase_degree() {
    let mut curve = original_curve();
    let val_before = curve.value(0.5);
    curve.increase_degree(5);
    assert_eq!(curve.degree(), 5);
    let val_after = curve.value(0.5);
    assert!(is_equal(val_before, val_after, 1e-10));
}

#[test]
fn insert_knot() {
    let mut curve = original_curve();
    let val_before = curve.value(0.5);
    curve.insert_knot(0.5, 1, 0.0);
    assert_eq!(curve.nb_knots(), 3);
    let val_after = curve.value(0.5);
    assert!(is_equal(val_before, val_after, 1e-10));
}

#[test]
fn remove_knot() {
    let mut curve = original_curve();
    curve.insert_knot(0.5, 1, 0.0);
    assert_eq!(curve.nb_knots(), 3);

    let is_removed = curve.remove_knot(2, 0, 1e-6);
    assert!(is_removed);
    assert_eq!(curve.nb_knots(), 2);
}

#[test]
fn segment() {
    let mut curve = original_curve();
    let p25 = curve.value(0.25);
    let p75 = curve.value(0.75);

    curve.segment(0.25, 0.75);
    assert!((curve.first_parameter() - 0.25).abs() < 1e-10);
    assert!((curve.last_parameter() - 0.75).abs() < 1e-10);
    assert!(is_equal(curve.start_point(), p25, 1e-6));
    assert!(is_equal(curve.end_point(), p75, 1e-6));
}

#[test]
fn reverse() {
    let mut curve = original_curve();
    let start = curve.start_point();
    let end = curve.end_point();
    curve.reverse();
    assert!(is_equal(curve.start_point(), end, 1e-10));
    assert!(is_equal(curve.end_point(), start, 1e-10));
}

#[test]
fn resolution() {
    let curve = original_curve();
    let u_tol = curve.resolution(1.0);
    assert!(u_tol > 0.0);
}

#[test]
fn transform() {
    let mut curve = original_curve();
    let mut trsf = Trsf2d::identity();
    trsf = set_translation(&mut trsf, Vec2d::new(10.0, 20.0));
    let pt_before = curve.value(0.5);
    curve.transform(&trsf);
    let pt_after = curve.value(0.5);
    assert!((pt_after.x - (pt_before.x + 10.0)).abs() < 1e-10);
    assert!((pt_after.y - (pt_before.y + 20.0)).abs() < 1e-10);
}

/// `gp_Trsf2d::SetTranslation(gp_Vec2d)` — build a pure-translation transform.
fn set_translation(_t: &mut Trsf2d, v: Vec2d) -> Trsf2d {
    Trsf2d::translation(Pnt2d::new(v.x, v.y))
}

#[test]
fn periodic_curve() {
    // Periodic BSpline: degree 3, 6 knots of mult 1 -> NbPoles = 5.
    let poles = [
        Pnt2d::new(1.0, 0.0),
        Pnt2d::new(0.309, 0.951),
        Pnt2d::new(-0.809, 0.588),
        Pnt2d::new(-0.809, -0.588),
        Pnt2d::new(0.309, -0.951),
    ];
    let mut knots = [0.0; 6];
    for (i, k) in knots.iter_mut().enumerate() {
        *k = i as f64 * 0.2;
    }
    let mults = [1, 1, 1, 1, 1, 1];

    let mut curve = Geom2dBSplineCurve::new(&poles, &knots, &mults, 3, true);
    assert!(curve.is_periodic());

    let val1 = curve.value(0.5);
    curve.set_not_periodic();
    assert!(!curve.is_periodic());

    let val2 = curve.value(0.5);
    assert!(is_equal(val1, val2, 1e-10));
}

#[test]
fn weights_access_non_rational() {
    let curve = original_curve();
    let weights = curve.weights();
    assert!(weights.is_none());
}

#[test]
fn knot_access() {
    let curve = original_curve();
    assert_eq!(curve.knot(1), 0.0);
    assert_eq!(curve.knot(2), 1.0);

    let knots = curve.knots();
    assert_eq!(knots.len(), 2);

    let mults = curve.multiplicities();
    assert_eq!(mults.len(), 2);
    assert_eq!(mults[0], 4);
}

#[test]
fn copy_independence() {
    let mut original = original_curve();
    let copy = original.copy();
    original.set_pole(2, Pnt2d::new(10.0, 10.0));
    assert!(!is_equal(copy.pole(2), Pnt2d::new(10.0, 10.0), 1e-10));
}

#[test]
fn set_weight() {
    let poles = [
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(2.0, 0.0),
    ];
    let weights = [1.0, 1.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let mut curve = Geom2dBSplineCurve::with_weights(&poles, &weights, &knots, &mults, 2, false);

    let mid_before = curve.value(0.5);
    curve.set_weight(2, 5.0);
    assert_eq!(curve.weight(2), 5.0);
    assert!(curve.is_rational());

    let mid_after = curve.value(0.5);
    assert!(mid_after.y > mid_before.y);
}

#[test]
fn move_point() {
    let mut curve = original_curve();
    let target = Pnt2d::new(1.5, 0.8);
    let np = curve.nb_poles();
    let (first, _last) = curve.move_point(0.5, target, 1, np);
    assert!(first > 0);
    let moved = curve.value(0.5);
    assert!(is_equal(moved, target, 1e-6));
}

#[test]
fn locate_u() {
    let mut curve = original_curve();
    curve.insert_knot(0.5, 1, 0.0);
    let (i1, i2) = curve.locate_u(0.25, 1e-10);
    assert!(i1 >= 1);
    assert!(i2 <= curve.nb_knots());
}

#[test]
fn local_d1() {
    let mut curve = original_curve();
    curve.insert_knot(0.5, 1, 0.0);
    let (pnt, v1) = curve.d1(0.25);
    let (pnt_l, v1_l) = curve.local_d1(0.25, 1, 2);
    assert!(is_equal(pnt, pnt_l, 1e-10));
    assert!((v1.x - v1_l.x).abs() < 1e-10);
    assert!((v1.y - v1_l.y).abs() < 1e-10);
}

#[test]
fn rational_curve_segment() {
    let poles = [
        Pnt2d::new(1.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(0.0, 1.0),
    ];
    let weights = [1.0, 1.0 / 2.0_f64.sqrt(), 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let mut curve = Geom2dBSplineCurve::with_weights(&poles, &weights, &knots, &mults, 2, false);

    let mid = curve.value(0.5);
    curve.segment(0.25, 0.75);
    let mid_after = curve.value(0.5);
    assert!(is_equal(mid, mid_after, 1e-6));
    assert!(curve.is_rational());
}

#[test]
fn closed_curve() {
    let poles = [
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(2.0, 0.0),
        Pnt2d::new(0.0, 0.0),
    ];
    let knots = [0.0, 1.0];
    let mults = [4, 4];

    let curve = Geom2dBSplineCurve::new(&poles, &knots, &mults, 3, false);
    assert!(curve.is_closed());
}

#[test]
fn insert_knots_multiple() {
    let mut curve = original_curve();
    let new_knots = [0.25, 0.75];
    let new_mults = [1, 1];

    let val_before = curve.value(0.5);
    curve.insert_knots(&new_knots, &new_mults, 0.0, false);
    assert_eq!(curve.nb_knots(), 4);
    let val_after = curve.value(0.5);
    assert!(is_equal(val_before, val_after, 1e-10));
}

#[test]
fn weights_array_non_rational_returns_unit_weights() {
    let curve = original_curve();
    assert!(!curve.is_rational());

    let weights = curve.weights_array();
    assert_eq!(weights.len(), curve.nb_poles());
    for &w in weights {
        assert_eq!(w, 1.0);
    }
    // Reference stability: same slice address on repeated calls.
    let a = curve.weights_array().as_ptr();
    let b = curve.weights_array().as_ptr();
    assert_eq!(a, b);
}

#[test]
fn weights_array_rational_returns_owning() {
    let poles = [
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(2.0, 1.0),
        Pnt2d::new(3.0, 0.0),
    ];
    let weights_in = [1.0, 2.0, 3.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [4, 4];

    let rational = Geom2dBSplineCurve::with_weights(&poles, &weights_in, &knots, &mults, 3, false);
    assert!(rational.is_rational());

    let weights = rational.weights_array();
    assert_eq!(weights.len(), 4);
    assert_eq!(weights[0], 1.0);
    assert_eq!(weights[1], 2.0);
    assert_eq!(weights[2], 3.0);
    assert_eq!(weights[3], 1.0);
    // Reference stability.
    let a = rational.weights_array().as_ptr();
    let b = rational.weights_array().as_ptr();
    assert_eq!(a, b);
}
