// FILE: tests/occt_geom_plate_ext.rs
extern crate ironstream;

use ironstream::geom_plate_ext::{
    PlateBuildStatus, PlatePointConstraint, PlateCurveConstraint,
    GeomPlateSurfaceExt, GeomPlateBuildAveragePlane,
};

#[test]
fn plate_build_status_is_done() {
    assert!(PlateBuildStatus::Done.is_done());
    assert!(!PlateBuildStatus::NotDone.is_done());
    assert!(!PlateBuildStatus::Error.is_done());
    assert!(!PlateBuildStatus::NoConstraint.is_done());
}

#[test]
fn plate_point_constraint_orders_and_fields() {
    let c0 = PlatePointConstraint::g0([1.0, 2.0, 3.0]);
    assert_eq!(c0.order(), 0);
    assert!(c0.normal().is_none());
    let pt = c0.point();
    assert!((pt[2] - 3.0).abs() < 1e-10);

    let c1 = PlatePointConstraint::g1([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
    assert_eq!(c1.order(), 1);
    assert!(c1.normal().is_some());
}

#[test]
fn plate_surface_build_with_point_constraints() {
    let mut s = GeomPlateSurfaceExt::new();
    s.add_point_constraint(PlatePointConstraint::g0([0.0, 0.0, 0.0]));
    s.add_point_constraint(PlatePointConstraint::g0([1.0, 0.0, 0.0]));
    s.add_point_constraint(PlatePointConstraint::g0([0.0, 1.0, 0.0]));
    s.build();
    assert!(s.is_done());
    assert_eq!(s.nb_point_constraints(), 3);
    assert!(s.surface() > 0);
}

#[test]
fn plate_surface_no_constraint_status() {
    let mut s = GeomPlateSurfaceExt::new();
    s.build();
    assert_eq!(s.status, PlateBuildStatus::NoConstraint);
    assert!(!s.is_done());
}

#[test]
fn plate_curve_constraint_fields() {
    let cc = PlateCurveConstraint::new(42, 1).with_surface(7);
    assert_eq!(cc.curve_id(), 42);
    assert_eq!(cc.order(), 1);
    assert_eq!(cc.surface_id, Some(7));
}

#[test]
fn average_plane_centroid_and_deviation() {
    let pts = vec![
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [1.0, 2.0, 0.0],
    ];
    let p = GeomPlateBuildAveragePlane::new(pts);
    assert!(p.is_done());
    let o = p.origin();
    assert!((o[0] - 1.0).abs() < 1e-10);
    // z is flat, so max_deviation should be 0
    assert!((p.max_deviation() - 0.0).abs() < 1e-10);
    // plane_id = 97000 + 3 = 97003
    assert_eq!(p.plane(), 97003);
}
