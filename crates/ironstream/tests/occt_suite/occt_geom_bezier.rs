//! Port of OCCT's `Geom_BezierCurve_Test.cxx` GTest suite
//! (`src/ModelingData/TKG3d/GTests/Geom_BezierCurve_Test.cxx`) to Rust
//! `#[test]` functions. Same numeric inputs, expected values and tolerances.
//!
//! Mapping:
//! - `gp_Pnt` / `gp_Vec` -> [`ironstream::gp::Pnt`]
//! - `NCollection_Array1<gp_Pnt>` / `<double>` -> `Vec<_>`
//! - `Handle(Geom_BezierCurve)` -> the [`GeomBezierCurve`] value
//! - `Precision::Confusion` etc. -> [`ironstream::precision`]
//! - `EXPECT_THROW(..., Standard_Failure)` -> `std::panic::catch_unwind`

use ironstream::geom_bezier::{GeomAbsShape, GeomBezierCurve};
use ironstream::gp::Pnt;

/// `gp_Vec::Magnitude()`.
fn magnitude(v: Pnt) -> f64 {
    v.norm()
}

/// Builds the shared test curve from `SetUp()`: poles (0,0,0),(1,1,0),(2,1,0),(3,0,0).
fn original_curve() -> GeomBezierCurve {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 1.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    GeomBezierCurve::new(poles)
}

#[test]
fn copy_constructor_basic_properties() {
    let original = original_curve();
    let copied = GeomBezierCurve::from_other(&original);

    assert_eq!(original.degree(), copied.degree());
    assert_eq!(original.nb_poles(), copied.nb_poles());
    assert_eq!(original.is_rational(), copied.is_rational());
    assert_eq!(original.is_closed(), copied.is_closed());
}

#[test]
fn copy_constructor_poles() {
    let original = original_curve();
    let copied = GeomBezierCurve::from_other(&original);

    for i in 1..=original.nb_poles() {
        let orig_pole = original.pole(i);
        let copy_pole = copied.pole(i);
        assert!(orig_pole.is_equal(copy_pole, 1e-10));
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
        assert!(orig_pnt.is_equal(copy_pnt, 1e-10));
        u += 0.25;
    }
}

#[test]
fn rational_curve_copy_constructor() {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = vec![1.0, 2.0, 1.0];

    let rational = GeomBezierCurve::new_rational(poles, weights);
    let copied = GeomBezierCurve::from_other(&rational);

    assert!(copied.is_rational());

    for i in 1..=rational.nb_poles() {
        assert_eq!(rational.weight(i), copied.weight(i));
    }
}

#[test]
fn copy_independence() {
    let mut original = original_curve();
    let copied = GeomBezierCurve::from_other(&original);

    // Modify the original curve.
    let new_pole = Pnt::new(10.0, 10.0, 10.0);
    original.set_pole(2, new_pole);

    // Verify the copied curve is not affected.
    let orig_pole = copied.pole(2);
    assert!(!orig_pole.is_equal(new_pole, 1e-10));
}

#[test]
fn evaluation_d0() {
    let curve = original_curve();
    let pnt = curve.d0(0.0);
    assert!(pnt.is_equal(Pnt::new(0.0, 0.0, 0.0), 1e-10));
    let pnt = curve.d0(1.0);
    assert!(pnt.is_equal(Pnt::new(3.0, 0.0, 0.0), 1e-10));
}

#[test]
fn evaluation_d1() {
    let curve = original_curve();
    let (_pnt, v1) = curve.d1(0.5);
    assert!(magnitude(v1) > 0.0);
}

#[test]
fn evaluation_d2() {
    let curve = original_curve();
    let (_pnt, v1, _v2) = curve.d2(0.5);
    assert!(magnitude(v1) > 0.0);
}

#[test]
fn evaluation_d3() {
    let curve = original_curve();
    let (_pnt, _v1, _v2, v3) = curve.d3(0.5);
    // D3 of a degree 3 curve is constant.
    let (_pnt_b, _v1b, _v2b, v3b) = curve.d3(0.25);
    assert!((v3.x - v3b.x).abs() < 1e-8);
    assert!((v3.y - v3b.y).abs() < 1e-8);
}

#[test]
fn evaluation_dn() {
    let curve = original_curve();
    let dn1 = curve.dn(0.5, 1);
    assert!(magnitude(dn1) > 0.0);
    // DN(4) for a degree 3 curve should be zero.
    let dn4 = curve.dn(0.5, 4);
    assert!((magnitude(dn4) - 0.0).abs() < 1e-10);
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
fn properties() {
    let curve = original_curve();
    assert_eq!(curve.degree(), 3);
    assert!(!curve.is_periodic());
    assert!(!curve.is_rational());
    assert!(!curve.is_closed());
    assert!(curve.is_cn(100));
    assert_eq!(curve.continuity(), GeomAbsShape::CN);
    assert_eq!(curve.first_parameter(), 0.0);
    assert_eq!(curve.last_parameter(), 1.0);
}

#[test]
fn set_pole() {
    let mut curve = original_curve();
    let new_pole = Pnt::new(1.0, 5.0, 0.0);
    curve.set_pole(2, new_pole);
    assert!(curve.pole(2).is_equal(new_pole, 1e-10));
    // Endpoints unchanged.
    assert!(curve.start_point().is_equal(Pnt::new(0.0, 0.0, 0.0), 1e-10));
    assert!(curve.end_point().is_equal(Pnt::new(3.0, 0.0, 0.0), 1e-10));
}

#[test]
fn set_weight() {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = vec![1.0, 1.0, 1.0];

    let mut curve = GeomBezierCurve::new_rational(poles, weights);
    let mid_before = curve.value(0.5);

    curve.set_weight(2, 10.0);
    assert_eq!(curve.weight(2), 10.0);
    assert!(curve.is_rational());

    let mid_after = curve.value(0.5);
    // High weight pulls curve toward the control point.
    assert!(mid_after.y > mid_before.y);
}

#[test]
fn insert_pole_after() {
    let mut curve = original_curve();
    let nb_before = curve.nb_poles();
    let new_pole = Pnt::new(1.5, 2.0, 0.0);
    curve.insert_pole_after(2, new_pole);
    assert_eq!(curve.nb_poles(), nb_before + 1);
    assert_eq!(curve.degree(), 4);
    assert!(curve.pole(3).is_equal(new_pole, 1e-10));
}

#[test]
fn insert_pole_before() {
    let mut curve = original_curve();
    let nb_before = curve.nb_poles();
    let new_pole = Pnt::new(0.5, 2.0, 0.0);
    curve.insert_pole_before(2, new_pole);
    assert_eq!(curve.nb_poles(), nb_before + 1);
    assert!(curve.pole(2).is_equal(new_pole, 1e-10));
}

#[test]
fn remove_pole() {
    let mut curve = original_curve();
    // Add a pole first so we have 5, then remove one.
    curve.insert_pole_after(2, Pnt::new(1.5, 2.0, 0.0));
    assert_eq!(curve.nb_poles(), 5);

    curve.remove_pole(3);
    assert_eq!(curve.nb_poles(), 4);
    assert_eq!(curve.degree(), 3);
}

#[test]
fn increase() {
    let mut curve = original_curve();
    let val_before = curve.value(0.5);
    curve.increase(5);
    assert_eq!(curve.degree(), 5);
    assert_eq!(curve.nb_poles(), 6);
    let val_after = curve.value(0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

#[test]
fn segment() {
    let mut curve = original_curve();
    let p25 = curve.value(0.25);
    let p75 = curve.value(0.75);

    curve.segment(0.25, 0.75);
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
fn transform() {
    use ironstream::gp::Trsf;
    let mut curve = original_curve();
    let trsf = Trsf::translation(Pnt::new(10.0, 20.0, 30.0));
    let pt_before = curve.value(0.5);
    curve.transform(&trsf);
    let pt_after = curve.value(0.5);
    assert!((pt_after.x - (pt_before.x + 10.0)).abs() < 1e-10);
    assert!((pt_after.y - (pt_before.y + 20.0)).abs() < 1e-10);
    assert!((pt_after.z - (pt_before.z + 30.0)).abs() < 1e-10);
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
fn rational_curve_evaluation() {
    // Create a rational quadratic Bezier (approximation of circular arc).
    let poles = vec![
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let weights = vec![1.0, 1.0 / 2.0_f64.sqrt(), 1.0];

    let curve = GeomBezierCurve::new_rational(poles, weights);
    assert!(curve.is_rational());

    // At u=0.5, the rational curve should be close to the unit circle.
    let mid = curve.value(0.5);
    let radius = (mid.x * mid.x + mid.y * mid.y).sqrt();
    assert!((radius - 1.0).abs() < 1e-6);
}

#[test]
fn max_degree() {
    assert!(GeomBezierCurve::max_degree() >= 25);
}

#[test]
fn set_pole_with_weight() {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = vec![1.0, 1.0, 1.0];

    let mut curve = GeomBezierCurve::new_rational(poles, weights);
    let new_pole = Pnt::new(1.0, 2.0, 0.0);
    curve.set_pole_with_weight(2, new_pole, 5.0);
    assert!(curve.pole(2).is_equal(new_pole, 1e-10));
    assert_eq!(curve.weight(2), 5.0);
    assert!(curve.is_rational());
}

#[test]
fn insert_pole_after_with_weight() {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = vec![1.0, 2.0, 1.0];

    let mut curve = GeomBezierCurve::new_rational(poles, weights);
    curve.insert_pole_after_with_weight(2, Pnt::new(1.5, 0.5, 0.0), 3.0);
    assert_eq!(curve.nb_poles(), 4);
    assert_eq!(curve.weight(3), 3.0);
    assert!(curve.is_rational());
}

#[test]
fn closed_curve() {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(0.0, 0.0, 0.0),
    ];
    let curve = GeomBezierCurve::new(poles);
    assert!(curve.is_closed());
}

#[test]
fn rational_segment() {
    let poles = vec![
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let weights = vec![1.0, 1.0 / 2.0_f64.sqrt(), 1.0];

    let mut curve = GeomBezierCurve::new_rational(poles, weights);
    let mid = curve.value(0.5);

    curve.segment(0.25, 0.75);
    let mid_after = curve.value(0.5);
    assert!(mid.is_equal(mid_after, 1e-6));
    assert!(curve.is_rational());
}

#[test]
fn rational_increase() {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = vec![1.0, 2.0, 1.0];

    let mut curve = GeomBezierCurve::new_rational(poles, weights);
    let val_before = curve.value(0.5);

    curve.increase(5);
    assert_eq!(curve.degree(), 5);
    assert!(curve.is_rational());
    let val_after = curve.value(0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

#[test]
fn rational_reverse() {
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights = vec![1.0, 3.0, 2.0];

    let mut curve = GeomBezierCurve::new_rational(poles, weights);
    let start = curve.start_point();
    let end = curve.end_point();

    curve.reverse();
    assert!(curve.start_point().is_equal(end, 1e-10));
    assert!(curve.end_point().is_equal(start, 1e-10));
    // Weights should be reversed.
    assert_eq!(curve.weight(1), 2.0);
    assert_eq!(curve.weight(3), 1.0);
}

#[test]
fn reversed_parameter() {
    let curve = original_curve();
    assert!((curve.reversed_parameter(0.3) - 0.7).abs() < 1e-12);
    assert_eq!(curve.reversed_parameter(0.0), 1.0);
}

#[test]
fn linear_curve() {
    // Degree 1 Bezier = line segment.
    let poles = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(3.0, 4.0, 0.0)];
    let curve = GeomBezierCurve::new(poles);
    assert_eq!(curve.degree(), 1);

    let mid = curve.value(0.5);
    assert!(mid.is_equal(Pnt::new(1.5, 2.0, 0.0), 1e-10));

    // D2 should be zero for a line.
    let (_pnt, _v1, v2) = curve.d2(0.5);
    assert!((magnitude(v2) - 0.0).abs() < 1e-10);
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
    let poles = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    let weights_in = vec![1.0, 2.0, 1.0];

    let rational = GeomBezierCurve::new_rational(poles, weights_in);
    assert!(rational.is_rational());

    let weights = rational.weights_array();
    assert_eq!(weights.len(), 3);
    assert_eq!(weights[0], 1.0);
    assert_eq!(weights[1], 2.0);
    assert_eq!(weights[2], 1.0);
}

// Test OCC2569: Geom_BezierCurve degree equals NbPoles - 1.
// Migrated from QABugs_17.cxx OCC2569.
#[test]
fn occ2569_degree_equals_nb_poles_minus_one() {
    let nb_poles = 26;
    let mut poles = Vec::with_capacity(nb_poles);
    for i in 1..=nb_poles as i32 {
        poles.push(Pnt::new(
            (i + 10) as f64,
            (i * 2 + 20) as f64,
            (i * 3 + 45) as f64,
        ));
    }

    let curve = GeomBezierCurve::new(poles);
    assert_eq!(curve.degree(), nb_poles as i32 - 1);
}

// Test OCC2569: Geom_BezierCurve throws when NbPoles exceeds maximum allowed.
// Migrated from QABugs_17.cxx OCC2569 (bug2569_2).
#[test]
fn occ2569_throws_for_too_many_poles() {
    let nb_poles = 29;
    let mut poles = Vec::with_capacity(nb_poles);
    for i in 1..=nb_poles as i32 {
        poles.push(Pnt::new(
            (i + 10) as f64,
            (i * 2 + 20) as f64,
            (i * 3 + 45) as f64,
        ));
    }

    let result = std::panic::catch_unwind(|| GeomBezierCurve::new(poles));
    assert!(result.is_err());
}
