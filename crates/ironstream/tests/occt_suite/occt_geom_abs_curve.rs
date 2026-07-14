extern crate ironstream;

use ironstream::geom_abs_curve::{
    CurveType, IsoType, JoinType, KnotDistribution, ShapeContinuity, SurfaceType,
};

#[test]
fn curve_type_properties() {
    assert!(CurveType::Ellipse.is_conic());
    assert!(CurveType::BezierCurve.is_bounded());
    assert!(!CurveType::Line.is_conic());
    assert!(!CurveType::Line.is_bounded());
    assert_eq!(CurveType::OffsetCurve.name(), "OffsetCurve");
}

#[test]
fn surface_type_properties() {
    assert!(SurfaceType::Torus.is_elementary());
    assert!(SurfaceType::ExtrusionSurface.is_swept());
    assert!(!SurfaceType::BSplineSurface.is_elementary());
    assert_eq!(SurfaceType::Plane.name(), "Plane");
}

#[test]
fn shape_continuity_ordering() {
    assert!(ShapeContinuity::C0 < ShapeContinuity::C1);
    assert!(ShapeContinuity::C2 < ShapeContinuity::CN);
    assert_eq!(ShapeContinuity::C3.order(), 3);
    assert!(ShapeContinuity::G1.is_geometric());
    assert!(!ShapeContinuity::C1.is_geometric());
}

#[test]
fn join_and_iso_types() {
    let _j = JoinType::Arc;
    let _i = IsoType::IsoU;
    assert_ne!(JoinType::Arc, JoinType::Tangent);
    assert_ne!(IsoType::IsoU, IsoType::IsoV);
    assert_eq!(IsoType::NoneIso, IsoType::NoneIso);
}

#[test]
fn knot_distribution_variants() {
    assert_ne!(KnotDistribution::Uniform, KnotDistribution::NonUniform);
    assert_eq!(KnotDistribution::PiecewiseBezier, KnotDistribution::PiecewiseBezier);
}
