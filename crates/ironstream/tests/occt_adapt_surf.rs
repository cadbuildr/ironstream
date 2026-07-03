// FILE: tests/occt_adapt_surf.rs
extern crate ironstream;

use ironstream::adapt_surf::{
    SurfaceType, Adaptor3dSurface, GeomAdaptorSurface, BrepAdaptorSurface,
};

#[test]
fn adaptor3d_default_bounds() {
    let s = Adaptor3dSurface::new(1, SurfaceType::BSplineSurface);
    assert!((s.u_first() - 0.0).abs() < 1e-10);
    assert!((s.u_last() - 1.0).abs() < 1e-10);
    assert!((s.v_first() - 0.0).abs() < 1e-10);
    assert!((s.v_last() - 1.0).abs() < 1e-10);
}

#[test]
fn adaptor3d_plane_value() {
    let s = Adaptor3dSurface::new(2, SurfaceType::Plane);
    let p = s.value(3.0, 4.0);
    assert!((p[0] - 3.0).abs() < 1e-10);
    assert!((p[1] - 4.0).abs() < 1e-10);
    assert!((p[2] - 0.0).abs() < 1e-10);
}

#[test]
fn adaptor3d_set_bounds_and_trim() {
    let s = Adaptor3dSurface::new(3, SurfaceType::BSplineSurface);
    let t = s.trim(0.0, 5.0, -1.0, 2.0);
    assert!((t.u_last() - 5.0).abs() < 1e-10);
    assert!((t.v_first() - (-1.0)).abs() < 1e-10);
    assert!((t.v_last() - 2.0).abs() < 1e-10);
}

#[test]
fn adaptor3d_periodicity_cylinder() {
    let s = Adaptor3dSurface::new(4, SurfaceType::Cylinder);
    assert!(s.is_u_periodic());
    assert!(!s.is_v_periodic());
    assert!(s.is_u_closed());
    assert!(!s.is_v_closed());
}

#[test]
fn geom_adaptor_surface_plane_type_and_value() {
    let g = GeomAdaptorSurface::new_plane(10);
    assert_eq!(g.get_type(), SurfaceType::Plane);
    let p = g.value(1.5, 2.5);
    assert!((p[0] - 1.5).abs() < 1e-10);
    assert!((p[1] - 2.5).abs() < 1e-10);
    assert_eq!(g.geom_surface_id, 10);
}

#[test]
fn brep_adaptor_surface_basics() {
    let b = BrepAdaptorSurface::new(99);
    assert_eq!(b.face(), 99);
    assert_eq!(b.get_type(), SurfaceType::Plane);
    assert!((b.tolerance() - 1e-7).abs() < 1e-15);
    // Plane value: z should be 0
    let p = b.value(2.0, 3.0);
    assert!((p[2] - 0.0).abs() < 1e-10);
}
