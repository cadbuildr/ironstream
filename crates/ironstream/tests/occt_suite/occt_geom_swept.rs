use ironstream::geom_swept::*;

use std::f64::consts::PI;

#[test]
fn linear_extrusion_direction_normalized() {
    // Non-unit direction should be normalized
    let s = GeomSurfLinExtrusion::new(1, [0.0, 0.0, 3.0], 0.0, 2.0 * PI);
    let d = s.direction();
    let len = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-10);
    assert!((d[2] - 1.0).abs() < 1e-10);
    assert_eq!(s.surface_id(), 11001);
}

#[test]
fn linear_extrusion_value() {
    let s = GeomSurfLinExtrusion::new(1, [0.0, 0.0, 1.0], 0.0, 1.0);
    // value(u=1.0, v=2.0): p[2] = v * direction[2] = 2.0
    let p = s.value(1.0, 2.0);
    assert!((p[2] - 2.0).abs() < 1e-10);
    // u component: u + v*dir[0] = 1.0 + 0.0 = 1.0
    assert!((p[0] - 1.0).abs() < 1e-10);
}

#[test]
fn surface_of_revolution_periodic_and_u_closed() {
    let s = GeomSurfRevolution::new(1, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    assert!(s.is_u_periodic());
    assert!(s.is_u_closed());
    assert!(!s.is_v_periodic());
    assert!((s.u_first_parameter()).abs() < 1e-10);
    assert!((s.u_last_parameter() - 2.0 * PI).abs() < 1e-10);
}

#[test]
fn surface_of_revolution_value() {
    let s = GeomSurfRevolution::new(1, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // angle=0, radius=2 -> point = (2, 0, 0)
    let p = s.value(0.0, 2.0);
    assert!((p[0] - 2.0).abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
    assert!(p[2].abs() < 1e-10);
    // angle=PI/2, radius=3 -> point ~ (0, 3, 0)
    let q = s.value(PI / 2.0, 3.0);
    assert!(q[0].abs() < 1e-10);
    assert!((q[1] - 3.0).abs() < 1e-10);
}

#[test]
fn rect_trimmed_surface_spans_and_valid_param() {
    let mut s = GeomRectTrimmedSurface::new(5, 0.0, 4.0, 1.0, 3.0);
    assert!((s.u_span() - 4.0).abs() < 1e-10);
    assert!((s.v_span() - 2.0).abs() < 1e-10);
    assert!(s.is_valid_param(2.0, 2.0));
    assert!(!s.is_valid_param(5.0, 2.0));
    assert!(!s.is_valid_param(2.0, 0.0));
    // surface_id = basis_id + 13000
    assert_eq!(s.surface_id, 5 + 13000);
    // set_trim_u narrows the u range
    s.set_trim_u(1.0, 3.0);
    assert!((s.u_first_parameter() - 1.0).abs() < 1e-10);
    assert!((s.u_last_parameter() - 3.0).abs() < 1e-10);
    // old u=0.5 now out of range
    assert!(!s.is_valid_param(0.5, 2.0));
}

#[test]
fn offset_surface_value() {
    let s = GeomOffsetSurface::new(3, 1.5);
    assert!(s.is_done());
    assert!((s.offset() - 1.5).abs() < 1e-12);
    // value(u, v) -> [u, v, offset]
    let p = s.value(0.3, 0.7);
    assert!((p[0] - 0.3).abs() < 1e-12);
    assert!((p[1] - 0.7).abs() < 1e-12);
    assert!((p[2] - 1.5).abs() < 1e-12);
    // surface_id = basis_id + 14000
    assert_eq!(s.surface_id(), 3 + 14000);
    // invalid basis_id=0 -> is_done false
    let bad = GeomOffsetSurface::new(0, 2.0);
    assert!(!bad.is_done());
}
