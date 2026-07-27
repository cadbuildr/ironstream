//! Port of OCCT's `Geom2d_BezierCurve_Test.cxx` GTest suite to Rust `#[test]`
//! functions, with the same numeric inputs, expected values and tolerances.
//!
//! Source: OCCT `src/ModelingData/TKG2d/GTests/Geom2d_BezierCurve_Test.cxx`.
//! Maps `gp_Pnt2d`/`gp_Vec2d` -> `ironstream::gp2d::Pnt2d`/`Vec2d`,
//! `NCollection_Array1` -> `Vec`, `Handle(Geom2d_BezierCurve)` -> the value,
//! `Precision` -> `ironstream::precision`.

use ironstream::geom2d_bezier::Geom2dBezierCurve;
use ironstream::gp2d::{Pnt2d, Trsf2d, Vec2d};

/// `gp_Pnt2d::IsEqual(Other, Tol)`.
fn pnt_is_equal(a: Pnt2d, b: Pnt2d, tol: f64) -> bool {
    a.distance(b) <= tol
}

/// `gp_Vec2d::Magnitude()`.
fn magnitude(v: Vec2d) -> f64 {
    v.norm()
}

/// The fixture's `SetUp()`: a simple degree-3 Bezier curve.
fn original_curve() -> Geom2dBezierCurve {
    Geom2dBezierCurve::new(&[
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(2.0, 1.0),
        Pnt2d::new(3.0, 0.0),
    ])
}

#[test]
fn copy_constructor_basic_properties() {
    let original = original_curve();
    let copied = original.copy();

    assert_eq!(original.degree(), copied.degree());
    assert_eq!(original.nb_poles(), copied.nb_poles());
    assert_eq!(original.is_rational(), copied.is_rational());
    assert_eq!(original.is_closed(), copied.is_closed());
}

#[test]
fn copy_constructor_poles() {
    let original = original_curve();
    let copied = original.copy();

    for i in 1..=original.nb_poles() {
        let orig_pole = original.pole(i);
        let copy_pole = copied.pole(i);
        assert!(pnt_is_equal(orig_pole, copy_pole, 1e-10));
    }
}

#[test]
fn copy_method_uses_optimized_constructor() {
    let original = original_curve();
    let copied = original.copy();

    assert_eq!(original.degree(), copied.degree());
    assert_eq!(original.nb_poles(), copied.nb_poles());

    // Evaluate at several points.
    let mut u = 0.0;
    while u <= 1.0 + 1e-12 {
        let orig_pnt = original.value(u);
        let copy_pnt = copied.value(u);
        assert!(pnt_is_equal(orig_pnt, copy_pnt, 1e-10));
        u += 0.25;
    }
}

#[test]
fn rational_curve_copy_constructor() {
    let rational = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 0.0),
        ],
        &[1.0, 2.0, 1.0],
    );
    let copied = rational.copy();

    assert!(copied.is_rational());

    for i in 1..=rational.nb_poles() {
        assert_eq!(rational.weight(i), copied.weight(i));
    }
}

#[test]
fn copy_independence() {
    let mut original = original_curve();
    let copied = original.copy();

    // Modify the original.
    let new_pole = Pnt2d::new(10.0, 10.0);
    original.set_pole(2, new_pole);

    // The copy must be unaffected.
    let orig_pole = copied.pole(2);
    assert!(!pnt_is_equal(orig_pole, new_pole, 1e-10));
}

#[test]
fn evaluation_d0() {
    let curve = original_curve();
    let pnt = curve.value(0.0);
    assert!(pnt_is_equal(pnt, Pnt2d::new(0.0, 0.0), 1e-10));
    let pnt = curve.value(1.0);
    assert!(pnt_is_equal(pnt, Pnt2d::new(3.0, 0.0), 1e-10));
}

#[test]
fn evaluation_d1() {
    let curve = original_curve();
    let (_p, v1) = curve.eval_d1(0.5);
    assert!(magnitude(v1) > 0.0);
}

#[test]
fn evaluation_d2() {
    let curve = original_curve();
    let (_p, v1, _v2) = curve.eval_d2(0.5);
    assert!(magnitude(v1) > 0.0);
}

#[test]
fn evaluation_dn() {
    let curve = original_curve();
    let dn1 = curve.eval_dn(0.5, 1);
    assert!(magnitude(dn1) > 0.0);
    let dn4 = curve.eval_dn(0.5, 4);
    assert!((magnitude(dn4) - 0.0).abs() < 1e-10);
}

#[test]
fn start_end_points() {
    let curve = original_curve();
    let start = curve.start_point();
    let end = curve.end_point();
    assert!(pnt_is_equal(start, Pnt2d::new(0.0, 0.0), 1e-10));
    assert!(pnt_is_equal(end, Pnt2d::new(3.0, 0.0), 1e-10));
}

#[test]
fn properties() {
    let curve = original_curve();
    assert_eq!(curve.degree(), 3);
    assert!(!curve.is_periodic());
    assert!(!curve.is_rational());
    assert!(!curve.is_closed());
    assert!(curve.is_cn(100));
    assert_eq!(curve.first_parameter(), 0.0);
    assert_eq!(curve.last_parameter(), 1.0);
}

#[test]
fn set_pole() {
    let mut curve = original_curve();
    let new_pole = Pnt2d::new(1.0, 5.0);
    curve.set_pole(2, new_pole);
    assert!(pnt_is_equal(curve.pole(2), new_pole, 1e-10));
    assert!(pnt_is_equal(curve.start_point(), Pnt2d::new(0.0, 0.0), 1e-10));
}

#[test]
fn set_weight() {
    let mut curve = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 0.0),
        ],
        &[1.0, 1.0, 1.0],
    );
    let mid_before = curve.value(0.5);

    curve.set_weight(2, 10.0);
    assert_eq!(curve.weight(2), 10.0);
    assert!(curve.is_rational());

    let mid_after = curve.value(0.5);
    assert!(mid_after.y > mid_before.y);
}

#[test]
fn insert_pole_after() {
    let mut curve = original_curve();
    let nb_before = curve.nb_poles();
    curve.insert_pole_after(2, Pnt2d::new(1.5, 2.0), 1.0);
    assert_eq!(curve.nb_poles(), nb_before + 1);
    assert_eq!(curve.degree(), 4);
}

#[test]
fn remove_pole() {
    let mut curve = original_curve();
    curve.insert_pole_after(2, Pnt2d::new(1.5, 2.0), 1.0);
    assert_eq!(curve.nb_poles(), 5);

    curve.remove_pole(3);
    assert_eq!(curve.nb_poles(), 4);
}

#[test]
fn increase() {
    let mut curve = original_curve();
    let val_before = curve.value(0.5);
    curve.increase(5);
    assert_eq!(curve.degree(), 5);
    let val_after = curve.value(0.5);
    assert!(pnt_is_equal(val_before, val_after, 1e-10));
}

#[test]
fn segment() {
    let mut curve = original_curve();
    let p25 = curve.value(0.25);
    let p75 = curve.value(0.75);

    curve.segment(0.25, 0.75);
    assert!(pnt_is_equal(curve.start_point(), p25, 1e-6));
    assert!(pnt_is_equal(curve.end_point(), p75, 1e-6));
}

#[test]
fn reverse() {
    let mut curve = original_curve();
    let start = curve.start_point();
    let end = curve.end_point();
    curve.reverse();
    assert!(pnt_is_equal(curve.start_point(), end, 1e-10));
    assert!(pnt_is_equal(curve.end_point(), start, 1e-10));
}

#[test]
fn resolution() {
    let mut curve = original_curve();
    let u_tol = curve.resolution(1.0);
    assert!(u_tol > 0.0);
}

#[test]
fn transform() {
    let mut curve = original_curve();
    let trsf = Trsf2d::translation(Vec2d::new(10.0, 20.0));
    let pt_before = curve.value(0.5);
    curve.transform(&trsf);
    let pt_after = curve.value(0.5);
    assert!((pt_after.x - (pt_before.x + 10.0)).abs() < 1e-10);
    assert!((pt_after.y - (pt_before.y + 20.0)).abs() < 1e-10);
}

#[test]
fn poles_access() {
    let curve = original_curve();
    let poles = curve.poles();
    assert_eq!(poles.len(), 4);
    assert!(pnt_is_equal(poles[0], Pnt2d::new(0.0, 0.0), 1e-10));
}

#[test]
fn weights_access_non_rational() {
    let curve = original_curve();
    // OCCT returns nullptr for non-rational; we return None.
    let weights = curve.weights();
    assert!(weights.is_none());
}

#[test]
fn max_degree() {
    assert!(Geom2dBezierCurve::max_degree() >= 25);
}

#[test]
fn rational_curve_evaluation() {
    // Rational quadratic Bezier approximating a circular arc.
    let curve = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(1.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(0.0, 1.0),
        ],
        &[1.0, 1.0 / 2.0_f64.sqrt(), 1.0],
    );
    assert!(curve.is_rational());

    let mid = curve.value(0.5);
    let radius = (mid.x * mid.x + mid.y * mid.y).sqrt();
    assert!((radius - 1.0).abs() < 1e-6);
}

#[test]
fn set_pole_with_weight() {
    let mut curve = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 0.0),
        ],
        &[1.0, 1.0, 1.0],
    );
    curve.set_pole_with_weight(2, Pnt2d::new(1.0, 2.0), 5.0);
    assert!(pnt_is_equal(curve.pole(2), Pnt2d::new(1.0, 2.0), 1e-10));
    assert_eq!(curve.weight(2), 5.0);
    assert!(curve.is_rational());
}

#[test]
fn insert_pole_before_with_weight() {
    let mut curve = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 0.0),
        ],
        &[1.0, 2.0, 1.0],
    );
    curve.insert_pole_before(2, Pnt2d::new(0.5, 0.5), 3.0);
    assert_eq!(curve.nb_poles(), 4);
    assert_eq!(curve.weight(2), 3.0);
}

#[test]
fn closed_curve() {
    let curve = Geom2dBezierCurve::new(&[
        Pnt2d::new(0.0, 0.0),
        Pnt2d::new(1.0, 1.0),
        Pnt2d::new(2.0, 0.0),
        Pnt2d::new(0.0, 0.0),
    ]);
    assert!(curve.is_closed());
}

#[test]
fn rational_segment() {
    let mut curve = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(1.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(0.0, 1.0),
        ],
        &[1.0, 1.0 / 2.0_f64.sqrt(), 1.0],
    );
    let mid = curve.value(0.5);

    curve.segment(0.25, 0.75);
    let mid_after = curve.value(0.5);
    assert!(pnt_is_equal(mid, mid_after, 1e-6));
    assert!(curve.is_rational());
}

#[test]
fn rational_reverse() {
    let mut curve = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 0.0),
        ],
        &[1.0, 3.0, 2.0],
    );
    let start = curve.start_point();
    let end = curve.end_point();

    curve.reverse();
    assert!(pnt_is_equal(curve.start_point(), end, 1e-10));
    assert!(pnt_is_equal(curve.end_point(), start, 1e-10));
    assert_eq!(curve.weight(1), 2.0);
    assert_eq!(curve.weight(3), 1.0);
}

#[test]
fn evaluation_d3() {
    let curve = original_curve();
    let (_p, _v1, _v2, v3) = curve.eval_d3(0.5);
    // D3 of degree 3 is constant.
    let (_pb, _v1b, _v2b, v3b) = curve.eval_d3(0.25);
    assert!((v3.x - v3b.x).abs() < 1e-8);
    assert!((v3.y - v3b.y).abs() < 1e-8);
}

#[test]
fn reversed_parameter() {
    let curve = original_curve();
    assert_eq!(curve.reversed_parameter(0.3), 0.7);
    assert_eq!(curve.reversed_parameter(0.0), 1.0);
}

#[test]
fn linear_curve() {
    let curve = Geom2dBezierCurve::new(&[Pnt2d::new(0.0, 0.0), Pnt2d::new(3.0, 4.0)]);
    assert_eq!(curve.degree(), 1);

    let mid = curve.value(0.5);
    assert!(pnt_is_equal(mid, Pnt2d::new(1.5, 2.0), 1e-10));
}

#[test]
fn rational_increase() {
    let mut curve = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 0.0),
        ],
        &[1.0, 2.0, 1.0],
    );
    let val_before = curve.value(0.5);

    curve.increase(5);
    assert_eq!(curve.degree(), 5);
    assert!(curve.is_rational());
    let val_after = curve.value(0.5);
    assert!(pnt_is_equal(val_before, val_after, 1e-10));
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
}

#[test]
fn weights_array_rational_returns_owning() {
    let rational = Geom2dBezierCurve::with_weights(
        &[
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 1.0),
            Pnt2d::new(2.0, 0.0),
        ],
        &[1.0, 2.0, 1.0],
    );
    assert!(rational.is_rational());

    let weights = rational.weights_array();
    assert_eq!(weights.len(), 3);
    assert_eq!(weights[0], 1.0);
    assert_eq!(weights[1], 2.0);
    assert_eq!(weights[2], 1.0);
}
