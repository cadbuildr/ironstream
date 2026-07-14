extern crate ironstream;
use ironstream::geom_adapt::*;

#[test]
fn test_geom_adapt_basic() {
    // AdaptorCurve construction and default domain.
    let curve = AdaptorCurve::new("BSplineCurve");
    assert_eq!(curve.curve_name, "BSplineCurve");
    assert!((curve.first - 0.0).abs() < f64::EPSILON);
    assert!((curve.last - 1.0).abs() < f64::EPSILON);
    assert_eq!(curve.curve_type, CurveType::OtherCurve);

    // Evaluation stubs.
    let v = curve.value(0.5);
    assert!((v[0] - 0.5).abs() < f64::EPSILON);
    assert_eq!(v[1], 0.0);
    assert_eq!(v[2], 0.0);

    let d1 = curve.d1(0.5);
    assert!((d1[0] - 1.0).abs() < f64::EPSILON);

    let d2 = curve.d2(0.5);
    assert_eq!(d2, [0.0, 0.0, 0.0]);

    assert_eq!(curve.continuity(), 2);

    // CurveType variants.
    assert_ne!(CurveType::Line, CurveType::Circle);
    assert_ne!(CurveType::BSplineCurve, CurveType::OtherCurve);

    // AdaptorSurface construction and stubs.
    let surf = AdaptorSurface::new("Plane");
    assert_eq!(surf.surface_name, "Plane");
    assert_eq!(surf.surface_type, SurfaceType::OtherSurface);

    let pv = surf.value(0.3, 0.7);
    assert!((pv[0] - 0.3).abs() < f64::EPSILON);
    assert!((pv[1] - 0.7).abs() < f64::EPSILON);
    assert_eq!(pv[2], 0.0);

    let du = surf.d1u(0.0, 0.0);
    assert!((du[0] - 1.0).abs() < f64::EPSILON);

    let dv = surf.d1v(0.0, 0.0);
    assert!((dv[1] - 1.0).abs() < f64::EPSILON);

    // SurfaceType variants.
    assert_ne!(SurfaceType::Plane, SurfaceType::Sphere);
    assert_ne!(SurfaceType::BSplineSurface, SurfaceType::OtherSurface);
}
