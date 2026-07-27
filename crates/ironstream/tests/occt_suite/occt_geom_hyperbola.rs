//! Ported OpenCascade unit tests -- `Geom_Hyperbola`.
//!
//! Each test mirrors a scenario from the OCCT test suite or exercises a
//! specific documented property of the hyperbola.  Tolerances use
//! `precision::CONFUSION` (1e-7) for linear quantities and
//! `precision::ANGULAR` (1e-12) for angles, consistent with the rest of the
//! IronStream test corpus.

use ironstream::geom_hyperbola::GeomHyperbola;
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::precision;

// ---------------------------------------------------------------------------
// Helper: standard XY-plane placement centered at the origin.
// ---------------------------------------------------------------------------
fn xy_placement() -> Ax3 {
    Ax3::identity()
}

// ---------------------------------------------------------------------------
// Test 1 – construction and basic getters
// ---------------------------------------------------------------------------
#[test]
fn test_construction_and_getters() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    assert!((h.major_radius() - 3.0).abs() < precision::CONFUSION);
    assert!((h.minor_radius() - 4.0).abs() < precision::CONFUSION);
    // Center is at the origin.
    assert!(h.location().distance(Pnt::origin()) < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// Test 2 – eccentricity: e = sqrt(1 + (b/a)^2)
//           For a=3, b=4: e = sqrt(1 + 16/9) = sqrt(25/9) = 5/3.
// ---------------------------------------------------------------------------
#[test]
fn test_eccentricity() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let expected = 5.0_f64 / 3.0;
    assert!(
        (h.eccentricity() - expected).abs() < precision::CONFUSION,
        "eccentricity mismatch: got {}, expected {}",
        h.eccentricity(),
        expected
    );
}

// ---------------------------------------------------------------------------
// Test 3 – focal distance: 2c where c = sqrt(a^2 + b^2)
//           For a=3, b=4: c=5 so focal=10.
// ---------------------------------------------------------------------------
#[test]
fn test_focal() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    assert!(
        (h.focal() - 10.0).abs() < precision::CONFUSION,
        "focal mismatch: {}",
        h.focal()
    );
}

// ---------------------------------------------------------------------------
// Test 4 – foci: F1 = (c, 0, 0), F2 = (-c, 0, 0)  for the standard placement.
// ---------------------------------------------------------------------------
#[test]
fn test_foci() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let f1 = h.focus1();
    let f2 = h.focus2();
    let expected_f1 = Pnt::new(5.0, 0.0, 0.0);
    let expected_f2 = Pnt::new(-5.0, 0.0, 0.0);
    assert!(
        f1.distance(expected_f1) < precision::CONFUSION,
        "Focus1 wrong: {:?}",
        f1
    );
    assert!(
        f2.distance(expected_f2) < precision::CONFUSION,
        "Focus2 wrong: {:?}",
        f2
    );
}

// ---------------------------------------------------------------------------
// Test 5 – asymptotes pass through the center and have slope ±b/a.
//           For a=3, b=4: direction (3,4,0)/5 and (3,-4,0)/5.
// ---------------------------------------------------------------------------
#[test]
fn test_asymptotes() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let a1 = h.asymptote1();
    let a2 = h.asymptote2();

    // Both pass through the center.
    assert!(a1.location.distance(Pnt::origin()) < precision::CONFUSION);
    assert!(a2.location.distance(Pnt::origin()) < precision::CONFUSION);

    // Check direction vectors (unit).
    let d1_expected = Pnt::new(3.0, 4.0, 0.0).normalized();
    let d2_expected = Pnt::new(3.0, -4.0, 0.0).normalized();

    assert!(
        a1.direction.distance(d1_expected) < precision::CONFUSION,
        "Asymptote1 direction wrong: {:?}",
        a1.direction
    );
    assert!(
        a2.direction.distance(d2_expected) < precision::CONFUSION,
        "Asymptote2 direction wrong: {:?}",
        a2.direction
    );
}

// ---------------------------------------------------------------------------
// Test 6 – Value / D0: at U=0 the point is (a, 0, 0).
// ---------------------------------------------------------------------------
#[test]
fn test_value_at_zero() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let p = h.value(0.0);
    let expected = Pnt::new(3.0, 0.0, 0.0);
    assert!(
        p.distance(expected) < precision::CONFUSION,
        "Value(0) wrong: {:?}",
        p
    );
}

// ---------------------------------------------------------------------------
// Test 7 – D1 first derivative at U=0: (a*sinh(0), b*cosh(0)) = (0, b, 0).
// ---------------------------------------------------------------------------
#[test]
fn test_d1_at_zero() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let (p, v1) = h.d1(0.0);
    let expected_p = Pnt::new(3.0, 0.0, 0.0);
    let expected_v1 = Pnt::new(0.0, 4.0, 0.0); // a*0 X + b*1 Y
    assert!(p.distance(expected_p) < precision::CONFUSION, "D1 point wrong");
    assert!(
        v1.distance(expected_v1) < precision::CONFUSION,
        "D1 tangent wrong: {:?}",
        v1
    );
}

// ---------------------------------------------------------------------------
// Test 8 – D2 second derivative: at U=0 it equals (a*cosh(0), b*sinh(0)) = (a,0,0).
// ---------------------------------------------------------------------------
#[test]
fn test_d2_at_zero() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let (_, _, v2) = h.d2(0.0);
    // D2 = a*cosh(U)*XDir + b*sinh(U)*YDir; at U=0: (a, 0, 0)
    let expected = Pnt::new(3.0, 0.0, 0.0);
    assert!(
        v2.distance(expected) < precision::CONFUSION,
        "D2 wrong: {:?}",
        v2
    );
}

// ---------------------------------------------------------------------------
// Test 9 – DN with N=1 and N=2 match D1/D2.
// ---------------------------------------------------------------------------
#[test]
fn test_dn_matches_d1_d2() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let u = 1.2_f64;
    let (_, v1) = h.d1(u);
    let (_, _, v2) = h.d2(u);
    let dn1 = h.dn(u, 1);
    let dn2 = h.dn(u, 2);
    assert!(dn1.distance(v1) < precision::CONFUSION, "DN(1) != D1");
    assert!(dn2.distance(v2) < precision::CONFUSION, "DN(2) != D2");
}

// ---------------------------------------------------------------------------
// Test 10 – Transform: translate the center by (10, 0, 0) and verify foci shift.
// ---------------------------------------------------------------------------
#[test]
fn test_transform_translation() {
    let mut h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let t = Trsf::translation(Pnt::new(10.0, 0.0, 0.0));
    h.transform(&t);
    // Center must now be at (10, 0, 0).
    assert!(
        h.location().distance(Pnt::new(10.0, 0.0, 0.0)) < precision::CONFUSION,
        "center after translate: {:?}",
        h.location()
    );
    // Focus1 = center + (5,0,0) = (15,0,0).
    assert!(
        h.focus1().distance(Pnt::new(15.0, 0.0, 0.0)) < precision::CONFUSION,
        "Focus1 after translate: {:?}",
        h.focus1()
    );
    // Radii unchanged by pure translation.
    assert!((h.major_radius() - 3.0).abs() < precision::CONFUSION);
    assert!((h.minor_radius() - 4.0).abs() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// Test 11 – Transform: uniform scale by 2 doubles the radii and foci distance.
// ---------------------------------------------------------------------------
#[test]
fn test_transform_scale() {
    let mut h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let mut t = Trsf::identity();
    t.set_scale(Pnt::origin(), 2.0);
    h.transform(&t);
    assert!(
        (h.major_radius() - 6.0).abs() < precision::CONFUSION,
        "major after scale: {}",
        h.major_radius()
    );
    assert!(
        (h.minor_radius() - 8.0).abs() < precision::CONFUSION,
        "minor after scale: {}",
        h.minor_radius()
    );
    // c = sqrt(6^2 + 8^2) = 10; Focus1 at (10,0,0).
    assert!(
        h.focus1().distance(Pnt::new(10.0, 0.0, 0.0)) < precision::CONFUSION,
        "Focus1 after scale: {:?}",
        h.focus1()
    );
}

// ---------------------------------------------------------------------------
// Test 12 – Setters: SetMajorRadius / SetMinorRadius update correctly.
// ---------------------------------------------------------------------------
#[test]
fn test_setters() {
    let mut h = GeomHyperbola::new(xy_placement(), 1.0, 1.0);
    h.set_major_radius(5.0);
    h.set_minor_radius(12.0);
    assert!((h.major_radius() - 5.0).abs() < precision::CONFUSION);
    assert!((h.minor_radius() - 12.0).abs() < precision::CONFUSION);
    // c = sqrt(25 + 144) = sqrt(169) = 13; focal = 26.
    assert!(
        (h.focal() - 26.0).abs() < precision::CONFUSION,
        "focal: {}",
        h.focal()
    );
}

// ---------------------------------------------------------------------------
// Test 13 – is_closed / is_periodic.
// ---------------------------------------------------------------------------
#[test]
fn test_closed_periodic() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    assert!(!h.is_closed(), "hyperbola must not be closed");
    assert!(!h.is_periodic(), "hyperbola must not be periodic");
}

// ---------------------------------------------------------------------------
// Test 14 – reversed_parameter: returns -U.
// ---------------------------------------------------------------------------
#[test]
fn test_reversed_parameter() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    assert!((h.reversed_parameter(1.5) - (-1.5)).abs() < precision::CONFUSION);
    assert!((h.reversed_parameter(-2.3) - 2.3).abs() < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// Test 15 – conjugate hyperbola: swaps radii and major axis.
// ---------------------------------------------------------------------------
#[test]
fn test_conjugate() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let conj = h.conjugate();
    // Conjugate has major=4, minor=3.
    assert!((conj.major_radius() - 4.0).abs() < precision::CONFUSION);
    assert!((conj.minor_radius() - 3.0).abs() < precision::CONFUSION);
    // Center unchanged.
    assert!(conj.location().distance(Pnt::origin()) < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// Test 16 – axis and x_axis / y_axis.
// ---------------------------------------------------------------------------
#[test]
fn test_axes() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    // Main axis = Z direction (identity placement).
    let ax = h.axis();
    assert!(ax.location.distance(Pnt::origin()) < precision::CONFUSION);
    assert!(ax.direction.distance(Pnt::new(0.0, 0.0, 1.0)) < precision::CONFUSION);
    // X axis along major axis.
    let xa = h.x_axis();
    assert!(xa.direction.distance(Pnt::new(1.0, 0.0, 0.0)) < precision::CONFUSION);
    // Y axis along minor axis.
    let ya = h.y_axis();
    assert!(ya.direction.distance(Pnt::new(0.0, 1.0, 0.0)) < precision::CONFUSION);
}

// ---------------------------------------------------------------------------
// Test 17 – D3 third derivative: at U=0 = (0, b, 0)  (same as V1).
// ---------------------------------------------------------------------------
#[test]
fn test_d3_at_zero() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let (_, _, _, v3) = h.d3(0.0);
    // D3 = a*sinh(0)*XDir + b*cosh(0)*YDir = (0, 4, 0).
    let expected = Pnt::new(0.0, 4.0, 0.0);
    assert!(
        v3.distance(expected) < precision::CONFUSION,
        "D3 wrong: {:?}",
        v3
    );
}

// ---------------------------------------------------------------------------
// Test 18 – parameter p = b^2 / a.
// ---------------------------------------------------------------------------
#[test]
fn test_parameter() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    // p = 16 / 3
    let expected = 16.0 / 3.0;
    assert!(
        (h.parameter() - expected).abs() < precision::CONFUSION,
        "parameter: {}",
        h.parameter()
    );
}

// ---------------------------------------------------------------------------
// Test 19 – Transform: rotation by 90° about Z swaps X→Y for the major axis.
// ---------------------------------------------------------------------------
#[test]
fn test_transform_rotation() {
    let h = GeomHyperbola::new(xy_placement(), 3.0, 4.0);
    let rot = Trsf::rotation(Ax1::oz(), std::f64::consts::FRAC_PI_2);
    let hr = h.transformed(&rot);
    // After 90° rotation, X axis becomes Y axis.
    let xa = hr.x_axis();
    assert!(
        xa.direction.distance(Pnt::new(0.0, 1.0, 0.0)) < precision::CONFUSION,
        "x_axis after rot: {:?}",
        xa.direction
    );
    // Radii unchanged by pure rotation.
    assert!((hr.major_radius() - 3.0).abs() < precision::CONFUSION);
    assert!((hr.minor_radius() - 4.0).abs() < precision::CONFUSION);
    // Focus1 was (5,0,0); after rotation it should be (0,5,0).
    assert!(
        hr.focus1().distance(Pnt::new(0.0, 5.0, 0.0)) < precision::CONFUSION,
        "Focus1 after rot: {:?}",
        hr.focus1()
    );
}
