// FILE: tests/occt_geom_tools.rs
extern crate ironstream;

use ironstream::geom_tools::{
    GeomToolsCurveAnalysis, GeomToolsCurveSet, GeomToolsSurfaceSet, GeomToolsUVBounds,
};

#[test]
fn uv_bounds_range_values() {
    let b = GeomToolsUVBounds::new(2.0, 8.0, -3.0, 1.0);
    assert_eq!(b.u_range(), 6.0);
    assert_eq!(b.v_range(), 4.0);
}

#[test]
fn uv_bounds_center_values() {
    let b = GeomToolsUVBounds::new(0.0, 10.0, -4.0, 4.0);
    assert_eq!(b.center_u(), 5.0);
    assert_eq!(b.center_v(), 0.0);
}

#[test]
fn uv_bounds_contains_inside() {
    let b = GeomToolsUVBounds::new(0.0, 1.0, 0.0, 1.0);
    assert!(b.contains_uv(0.0, 0.0));
    assert!(b.contains_uv(1.0, 1.0));
    assert!(b.contains_uv(0.5, 0.5));
}

#[test]
fn uv_bounds_contains_outside() {
    let b = GeomToolsUVBounds::new(0.0, 1.0, 0.0, 1.0);
    assert!(!b.contains_uv(-0.001, 0.5));
    assert!(!b.contains_uv(0.5, 1.001));
    assert!(!b.contains_uv(2.0, 2.0));
}

#[test]
fn uv_bounds_closed_default_open() {
    let b = GeomToolsUVBounds::new(0.0, 1.0, 0.0, 1.0);
    assert!(!b.is_u_closed);
    assert!(!b.is_v_closed);
}

#[test]
fn uv_bounds_with_u_closed() {
    let b = GeomToolsUVBounds::new(0.0, 6.28, 0.0, 1.0).with_u_closed();
    assert!(b.is_u_closed);
    assert!(!b.is_v_closed);
}

#[test]
fn uv_bounds_with_both_closed() {
    let b = GeomToolsUVBounds::new(0.0, 6.28, 0.0, 6.28)
        .with_u_closed()
        .with_v_closed();
    assert!(b.is_u_closed);
    assert!(b.is_v_closed);
}

#[test]
fn curve_set_add_returns_index() {
    let mut cs = GeomToolsCurveSet::new();
    let i0 = cs.add("Line".to_string(), 0.0, 1.0);
    let i1 = cs.add("Circle".to_string(), 0.0, 6.28);
    assert_eq!(i0, 0);
    assert_eq!(i1, 1);
    assert_eq!(cs.nb_curves(), 2);
}

#[test]
fn curve_set_get_type_and_range() {
    let mut cs = GeomToolsCurveSet::new();
    cs.add("BSpline".to_string(), 0.0, 1.0);
    assert_eq!(cs.curve_type(0), Some("BSpline"));
    let r = cs.curve_range(0).unwrap();
    assert_eq!(r[0], 0.0);
    assert_eq!(r[1], 1.0);
}

#[test]
fn curve_set_clear_empties() {
    let mut cs = GeomToolsCurveSet::new();
    cs.add("Ellipse".to_string(), 0.0, 6.28);
    cs.add("Line".to_string(), -10.0, 10.0);
    cs.clear();
    assert_eq!(cs.nb_curves(), 0);
    assert!(cs.curve_type(0).is_none());
}

#[test]
fn surface_set_add_and_retrieve() {
    let mut ss = GeomToolsSurfaceSet::new();
    let idx = ss.add("Sphere".to_string(), -1.57, 1.57, 0.0, 6.28);
    assert_eq!(idx, 0);
    assert_eq!(ss.nb_surfaces(), 1);
    assert_eq!(ss.surface_type(0), Some("Sphere"));
    let r = ss.surface_range(0).unwrap();
    assert!((r[0] - (-1.57)).abs() < 1e-12);
    assert!((r[2] - 0.0).abs() < 1e-12);
}

#[test]
fn surface_set_multiple_types() {
    let mut ss = GeomToolsSurfaceSet::new();
    ss.add("Plane".to_string(), 0.0, 1.0, 0.0, 1.0);
    ss.add("Cylinder".to_string(), 0.0, 10.0, 0.0, 6.28);
    ss.add("BSplineSurface".to_string(), 0.0, 1.0, 0.0, 1.0);
    assert_eq!(ss.nb_surfaces(), 3);
    assert_eq!(ss.surface_type(2), Some("BSplineSurface"));
}

#[test]
fn surface_set_clear_empties() {
    let mut ss = GeomToolsSurfaceSet::new();
    ss.add("Torus".to_string(), 0.0, 6.28, 0.0, 6.28);
    ss.clear();
    assert_eq!(ss.nb_surfaces(), 0);
    assert!(ss.surface_range(0).is_none());
}

#[test]
fn guess_kind_and_bounds_analysis() {
    assert_eq!(GeomToolsCurveAnalysis::guess_kind(0.0, 6.28, true), "Circle");
    assert_eq!(GeomToolsCurveAnalysis::guess_kind(0.0, 1.0, false), "BSpline");
    assert_eq!(GeomToolsCurveAnalysis::guess_kind(1.0, 5.0, false), "Curve");
    assert_eq!(GeomToolsCurveAnalysis::bounds_of_cos(2), 3.0);
    assert_eq!(GeomToolsCurveAnalysis::bound_of_bspline(3, 7), 6.0);
}
