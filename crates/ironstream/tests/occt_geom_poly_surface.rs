extern crate ironstream;
use ironstream::geom_poly_surface::*;

#[test]
fn sequence_of_surface_append_and_value() {
    let mut seq = SequenceOfSurface::new();
    seq.append(SurfaceHandle { id: 1, surface_type: "Plane".into() });
    seq.append(SurfaceHandle { id: 2, surface_type: "Cylinder".into() });
    assert_eq!(seq.size(), 2);
    assert_eq!(seq.value(1).unwrap().id, 1);
    assert_eq!(seq.value(2).unwrap().surface_type, "Cylinder");
}

#[test]
fn polygon_on_surface_nodes() {
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let p = PolygonOnSurface::new(5, uvs);
    assert_eq!(p.nb_nodes(), 4);
    assert_eq!(p.node(1).unwrap(), [0.0, 0.0]);
    assert_eq!(p.node(3).unwrap(), [1.0, 1.0]);
}

#[test]
fn offset_surface_value() {
    let s = OffsetSurface::new(2, 1.5);
    assert!((s.offset_value() - 1.5).abs() < 1e-10);
    assert_eq!(s.basis_surface_id, 2);
}

#[test]
fn rectangular_trimmed_surface_bounds() {
    let s = RectangularTrimmedSurface::new(1, 0.0, 2.0, 1.0, 3.0);
    assert_eq!(s.u_bounds(), (0.0, 2.0));
    assert_eq!(s.v_bounds(), (1.0, 3.0));
    assert!(!s.is_u_periodic());
    assert!(!s.is_v_periodic());
}

#[test]
fn swept_surface_direction() {
    let s = SweptSurface::new(1, [0.0, 1.0, 0.0]);
    let dir = s.direction();
    assert!((dir[1] - 1.0).abs() < 1e-10);
}
