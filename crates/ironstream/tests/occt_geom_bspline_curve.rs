//! Ported OCCT GTest for `Geom_BSplineCurve`
//! (`src/ModelingData/TKG3d/GTests/Geom_BSplineCurve_Test.cxx`).
//!
//! Each `#[test]` mirrors one `TEST_F` from the OCCT suite with the SAME numeric
//! inputs, expected values, and tolerances. The OCCT `SetUp()` fixture is
//! reproduced by the `original_curve()` helper.

use ironstream::geom_bspline_curve::{BSplKnotDistribution, GeomBSplineCurve, Shape};
use ironstream::gp::Pnt;
use ironstream::gp::Trsf;

/// OCCT `SetUp()`: a cubic (degree 3) non-rational B-spline with 4 poles and a
/// single Bezier span (knots {0,1}, mults {4,4}).
fn original_curve() -> GeomBSplineCurve {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 1.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    let knots = [0.0, 1.0];
    let mults = [4, 4];
    GeomBSplineCurve::new(&poles, &knots, &mults, 3, false)
}

#[test]
fn copy_constructor_basic_properties() {
    let orig = original_curve();
    let copied = orig.copy();

    assert_eq!(orig.degree(), copied.degree());
    assert_eq!(orig.nb_poles(), copied.nb_poles());
    assert_eq!(orig.nb_knots(), copied.nb_knots());
    assert_eq!(orig.is_periodic(), copied.is_periodic());
    assert_eq!(orig.is_rational(), copied.is_rational());
}

#[test]
fn copy_constructor_poles() {
    let orig = original_curve();
    let copied = orig.copy();
    for i in 1..=orig.nb_poles() {
        assert!(orig.pole(i).is_equal(copied.pole(i), 1e-10));
    }
}

#[test]
fn copy_constructor_knots() {
    let orig = original_curve();
    let copied = orig.copy();
    for i in 1..=orig.nb_knots() {
        assert_eq!(orig.knot(i), copied.knot(i));
        assert_eq!(orig.multiplicity(i), copied.multiplicity(i));
    }
}

#[test]
fn copy_method_uses_optimized_constructor() {
    let orig = original_curve();
    let copied = orig.copy_geom();

    assert_eq!(orig.degree(), copied.degree());
    assert_eq!(orig.nb_poles(), copied.nb_poles());

    let mut u = 0.0;
    while u <= 1.0 + 1e-12 {
        let a = orig.value(u);
        let b = copied.value(u);
        assert!(a.is_equal(b, 1e-10), "u={u}");
        u += 0.25;
    }
}

#[test]
fn rational_curve_copy_constructor() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = [1.0, 2.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let rational = GeomBSplineCurve::new_rational(&poles, &weights, &knots, &mults, 2, false);
    let copied = rational.copy();

    assert!(copied.is_rational());
    for i in 1..=rational.nb_poles() {
        assert_eq!(rational.weight(i), copied.weight(i));
    }
}

#[test]
fn evaluation_d0() {
    let curve = original_curve();
    let p = curve.d0(0.0);
    assert!(p.is_equal(Pnt::new(0.0, 0.0, 0.0), 1e-10));
    let p = curve.d0(1.0);
    assert!(p.is_equal(Pnt::new(3.0, 0.0, 0.0), 1e-10));
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
fn evaluation_d3() {
    let curve = original_curve();
    let (_p, _v1, _v2, v3) = curve.d3(0.5);
    let (_pb, _v1b, _v2b, v3b) = curve.d3(0.25);
    // D3 of a degree 3 curve is constant.
    assert!((v3.x - v3b.x).abs() < 1e-8);
    assert!((v3.y - v3b.y).abs() < 1e-8);
    assert!((v3.z - v3b.z).abs() < 1e-8);
}

#[test]
fn evaluation_dn() {
    let curve = original_curve();
    let dn = curve.dn(0.5, 1);
    assert!(dn.norm() > 0.0);
    // DN(4) for a degree 3 curve should be zero.
    let dn4 = curve.dn(0.5, 4);
    assert!((dn4.norm() - 0.0).abs() < 1e-10);
}

#[test]
fn start_end_points() {
    let curve = original_curve();
    let start = curve.start_point();
    let end = curve.end_point();
    assert!(start.is_equal(Pnt::new(0.0, 0.0, 0.0), 1e-10));
    assert!(end.is_equal(Pnt::new(3.0, 0.0, 0.0), 1e-10));
}

#[test]
fn set_pole() {
    let mut curve = original_curve();
    let new_pole = Pnt::new(1.0, 2.0, 3.0);
    curve.set_pole(2, new_pole);
    assert!(curve.pole(2).is_equal(new_pole, 1e-10));
    // End points should not change.
    assert!(curve.start_point().is_equal(Pnt::new(0.0, 0.0, 0.0), 1e-10));
}

#[test]
fn set_weight() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = [1.0, 1.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let mut curve = GeomBSplineCurve::new_rational(&poles, &weights, &knots, &mults, 2, false);
    let mid_before = curve.value(0.5);

    curve.set_weight(2, 5.0);
    assert_eq!(curve.weight(2), 5.0);
    assert!(curve.is_rational());

    let mid_after = curve.value(0.5);
    assert!(!mid_before.is_equal(mid_after, 1e-6));
}

#[test]
fn increase_degree() {
    let mut curve = original_curve();
    let val_before = curve.value(0.5);
    curve.increase_degree(5);
    assert_eq!(curve.degree(), 5);
    let val_after = curve.value(0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

#[test]
fn insert_knot() {
    let mut curve = original_curve();
    let val_before = curve.value(0.5);
    curve.insert_knot(0.5, 1);
    assert_eq!(curve.nb_knots(), 3);
    let val_after = curve.value(0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

#[test]
fn remove_knot() {
    let mut curve = original_curve();
    curve.insert_knot(0.5, 1);
    assert_eq!(curve.nb_knots(), 3);
    let val_before = curve.value(0.25);

    let is_removed = curve.remove_knot(2, 0, 1e-6);
    assert!(is_removed);
    assert_eq!(curve.nb_knots(), 2);
    let val_after = curve.value(0.25);
    assert!(val_before.is_equal(val_after, 1e-6));
}

#[test]
fn segment() {
    let mut curve = original_curve();
    let p25 = curve.value(0.25);
    let p75 = curve.value(0.75);

    curve.segment(0.25, 0.75);
    assert!((curve.first_parameter() - 0.25).abs() < 1e-10);
    assert!((curve.last_parameter() - 0.75).abs() < 1e-10);
    assert!(curve.start_point().is_equal(p25, 1e-6));
    assert!(curve.end_point().is_equal(p75, 1e-6));
}

#[test]
fn reverse() {
    let mut curve = original_curve();
    let start = curve.start_point();
    let end = curve.end_point();
    curve.reverse();
    assert!(curve.start_point().is_equal(end, 1e-10));
    assert!(curve.end_point().is_equal(start, 1e-10));
}

#[test]
fn resolution() {
    let curve = original_curve();
    let u_tol = curve.resolution(1.0);
    assert!(u_tol > 0.0);
}

#[test]
fn properties() {
    let curve = original_curve();
    assert_eq!(curve.degree(), 3);
    assert!(!curve.is_periodic());
    assert!(!curve.is_rational());
    assert!(!curve.is_closed());
    assert!(curve.is_cn(3));
    assert_eq!(curve.continuity(), Shape::CN);
}

#[test]
fn transform() {
    let mut curve = original_curve();
    let mut trsf = Trsf::default();
    trsf.set_translation(Pnt::new(10.0, 20.0, 30.0));
    let before = curve.value(0.5);
    curve.transform(&trsf);
    let after = curve.value(0.5);
    assert!((after.x - (before.x + 10.0)).abs() < 1e-10);
    assert!((after.y - (before.y + 20.0)).abs() < 1e-10);
    assert!((after.z - (before.z + 30.0)).abs() < 1e-10);
}

#[test]
fn periodic_curve() {
    let poles = [
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.309, 0.951, 0.0),
        Pnt::new(-0.809, 0.588, 0.0),
        Pnt::new(-0.809, -0.588, 0.0),
        Pnt::new(0.309, -0.951, 0.0),
    ];
    let knots: Vec<f64> = (1..=6).map(|i| (i - 1) as f64 * 0.2).collect();
    let mults = [1, 1, 1, 1, 1, 1];

    let mut curve = GeomBSplineCurve::new(&poles, &knots, &mults, 3, true);
    assert!(curve.is_periodic());

    let val1 = curve.value(0.5);
    curve.set_not_periodic();
    assert!(!curve.is_periodic());

    let val2 = curve.value(0.5);
    assert!(val1.is_equal(val2, 1e-10));
}

#[test]
fn multi_knot_span() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 2.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 2.0, 0.0),
        Pnt::new(4.0, 0.0, 0.0),
    ];
    let knots = [0.0, 1.0, 2.0, 3.0];
    let mults = [3, 1, 1, 3];

    let curve = GeomBSplineCurve::new(&poles, &knots, &mults, 2, false);
    assert_eq!(curve.nb_knots(), 4);
    assert_eq!(curve.nb_poles(), 5);

    let p = curve.d0(0.0);
    assert!(p.is_equal(Pnt::new(0.0, 0.0, 0.0), 1e-10));
    let p = curve.d0(3.0);
    assert!(p.is_equal(Pnt::new(4.0, 0.0, 0.0), 1e-10));
}

#[test]
fn increase_multiplicity() {
    let mut curve = original_curve();
    // Add a knot first to have an interior knot.
    curve.insert_knot(0.5, 1);
    assert_eq!(curve.nb_knots(), 3);
    assert_eq!(curve.multiplicity(2), 1);

    let val_before = curve.value(0.5);
    curve.increase_multiplicity(2, 2);
    assert_eq!(curve.multiplicity(2), 2);
    let val_after = curve.value(0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

#[test]
fn knot_access() {
    let curve = original_curve();
    assert_eq!(curve.knot(1), 0.0);
    assert_eq!(curve.knot(2), 1.0);
    assert_eq!(curve.first_u_knot_index(), 1);
    assert_eq!(curve.last_u_knot_index(), 2);

    let knots = curve.knots();
    assert_eq!(knots.len(), 2);

    let flat_knots = curve.knot_sequence();
    assert_eq!(flat_knots.len(), 8);

    let mults = curve.multiplicities();
    assert_eq!(mults.len(), 2);
    assert_eq!(mults[0], 4);
    assert_eq!(mults[1], 4);
}

#[test]
fn poles_access() {
    let curve = original_curve();
    let poles = curve.poles();
    assert_eq!(poles.len(), 4);
    assert!(poles[0].is_equal(Pnt::new(0.0, 0.0, 0.0), 1e-10));
}

#[test]
fn weights_access_non_rational() {
    let curve = original_curve();
    let weights = curve.weights();
    assert!(weights.is_none());
}

#[test]
fn weights_access_rational() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = [1.0, 2.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let curve = GeomBSplineCurve::new_rational(&poles, &weights, &knots, &mults, 2, false);
    let w = curve.weights();
    assert!(w.is_some());
    assert_eq!(w.unwrap()[1], 2.0);
}

#[test]
fn local_evaluation() {
    let mut curve = original_curve();
    curve.insert_knot(0.5, 1);
    assert_eq!(curve.nb_knots(), 3);

    let global = curve.value(0.25);
    let local = curve.local_value(0.25, 1, 2);
    assert!(global.is_equal(local, 1e-10));
}

#[test]
fn set_knot() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 1.0, 0.0),
    ];
    let knots = [0.0, 0.5, 1.0];
    let mults = [3, 1, 3];

    let mut curve = GeomBSplineCurve::new(&poles, &knots, &mults, 2, false);
    curve.set_knot(2, 0.6);
    assert_eq!(curve.knot(2), 0.6);
}

#[test]
fn copy_independence() {
    let mut orig = original_curve();
    let copy = orig.copy();
    orig.set_pole(2, Pnt::new(10.0, 10.0, 10.0));
    assert!(!copy.pole(2).is_equal(Pnt::new(10.0, 10.0, 10.0), 1e-10));
}

#[test]
fn move_point() {
    let mut curve = original_curve();
    let target = Pnt::new(1.5, 0.8, 0.0);
    let nb = curve.nb_poles();
    let (first, _last) = curve.move_point(0.5, target, 1, nb);
    assert!(first > 0);
    let moved = curve.value(0.5);
    assert!(moved.is_equal(target, 1e-6));
}

#[test]
fn locate_u() {
    let mut curve = original_curve();
    curve.insert_knot(0.5, 1);
    let (i1, i2) = curve.locate_u(0.25, 1e-10);
    assert!(i1 >= 1);
    assert!(i2 <= curve.nb_knots());
    assert!(curve.knot(i1) <= 0.25 + 1e-10);
    assert!(curve.knot(i2) >= 0.25 - 1e-10);
}

#[test]
fn is_equal() {
    let orig = original_curve();
    let mut copy = orig.copy();
    assert!(orig.is_equal(&copy, 1e-10));

    copy.set_pole(2, Pnt::new(10.0, 10.0, 10.0));
    assert!(!orig.is_equal(&copy, 1e-10));
}

#[test]
fn knot_distribution() {
    let curve = original_curve();
    let distr = curve.knot_distribution();
    // Single span with end mults = degree+1 -> PiecewiseBezier.
    assert_eq!(distr, BSplKnotDistribution::PiecewiseBezier);
}

#[test]
fn rational_curve_segment() {
    let poles = [
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let weights = [1.0, 1.0 / 2.0_f64.sqrt(), 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let mut curve = GeomBSplineCurve::new_rational(&poles, &weights, &knots, &mults, 2, false);
    assert!(curve.is_rational());

    let mid = curve.value(0.5);
    curve.segment(0.25, 0.75);
    let mid_after = curve.value(0.5);
    assert!(mid.is_equal(mid_after, 1e-6));
    assert!(curve.is_rational());
}

#[test]
fn rational_curve_increase_degree() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = [1.0, 2.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];

    let mut curve = GeomBSplineCurve::new_rational(&poles, &weights, &knots, &mults, 2, false);
    let val_before = curve.value(0.5);

    curve.increase_degree(4);
    assert_eq!(curve.degree(), 4);
    assert!(curve.is_rational());
    let val_after = curve.value(0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

#[test]
fn periodic_curve_set_origin() {
    let poles = [
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.309, 0.951, 0.0),
        Pnt::new(-0.809, 0.588, 0.0),
        Pnt::new(-0.809, -0.588, 0.0),
        Pnt::new(0.309, -0.951, 0.0),
    ];
    let knots: Vec<f64> = (1..=6).map(|i| (i - 1) as f64 * 0.2).collect();
    let mults = [1, 1, 1, 1, 1, 1];

    let mut curve = GeomBSplineCurve::new(&poles, &knots, &mults, 3, true);
    assert!(curve.is_periodic());

    curve.set_origin(3);
    assert!(curve.is_periodic());

    // Curve shape unchanged, just reparameterized -- can evaluate without crash.
    let val2 = curve.value(curve.first_parameter());
    assert!(val2.x * val2.x + val2.y * val2.y > 0.0);
}

#[test]
fn insert_knots_multiple() {
    let mut curve = original_curve();
    let new_knots = [0.25, 0.75];
    let new_mults = [1, 1];

    let val_before = curve.value(0.5);
    curve.insert_knots(&new_knots, &new_mults);
    assert_eq!(curve.nb_knots(), 4);
    let val_after = curve.value(0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

#[test]
fn closed_curve() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 0.0),
    ];
    let knots = [0.0, 1.0];
    let mults = [4, 4];

    let curve = GeomBSplineCurve::new(&poles, &knots, &mults, 3, false);
    assert!(curve.is_closed());
}

#[test]
fn local_d1() {
    let mut curve = original_curve();
    curve.insert_knot(0.5, 1);
    let (p, v1) = curve.d1(0.25);
    let (pl, v1l) = curve.local_d1(0.25, 1, 2);
    assert!(p.is_equal(pl, 1e-10));
    assert!((v1.x - v1l.x).abs() < 1e-10);
    assert!((v1.y - v1l.y).abs() < 1e-10);
}

#[test]
fn weights_array_non_rational_returns_unit_weights() {
    let curve = original_curve();
    assert!(!curve.is_rational());

    let weights = curve.weights_array();
    assert_eq!(weights.len() as i32, curve.nb_poles());
    for &w in weights {
        assert_eq!(w, 1.0);
    }
}

#[test]
fn weights_array_rational_returns_owning() {
    let poles = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 1.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    let weights_in = [1.0, 2.0, 3.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [4, 4];

    let rational = GeomBSplineCurve::new_rational(&poles, &weights_in, &knots, &mults, 3, false);
    assert!(rational.is_rational());

    let weights = rational.weights_array();
    assert_eq!(weights.len(), 4);
    assert_eq!(weights[0], 1.0);
    assert_eq!(weights[1], 2.0);
    assert_eq!(weights[2], 3.0);
    assert_eq!(weights[3], 1.0);
}
