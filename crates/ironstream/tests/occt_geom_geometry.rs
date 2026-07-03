// FILE: tests/occt_geom_geometry.rs
extern crate ironstream;
use ironstream::geom_geometry::{GeomGeometry, GeomPoint};
use ironstream::gp::{Pnt, Vec3, Ax1, Ax2, Ax3};

fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

fn pnt_approx_eq(p: &Pnt, x: f64, y: f64, z: f64, tol: f64) -> bool {
    approx_eq(p.x, x, tol) && approx_eq(p.y, y, tol) && approx_eq(p.z, z, tol)
}

fn make_ax1(lx: f64, ly: f64, lz: f64, dx: f64, dy: f64, dz: f64) -> Ax1 {
    Ax1::new(
        Pnt { x: lx, y: ly, z: lz },
        Pnt { x: dx, y: dy, z: dz },
    )
}

/// Build an Ax2 (= Ax3) from a location and a normal (z_dir).
fn make_ax2(lx: f64, ly: f64, lz: f64, nx: f64, ny: f64, nz: f64) -> Ax2 {
    Ax3::from_origin_normal(
        Pnt { x: lx, y: ly, z: lz },
        Pnt { x: nx, y: ny, z: nz },
        Pnt { x: 1.0, y: 0.0, z: 0.0 },
    )
}

#[test]
fn test_geom_point_identity() {
    let gp = GeomPoint::new(1.0, 2.0, 3.0);
    assert_eq!(gp.x(), 1.0);
    assert_eq!(gp.y(), 2.0);
    assert_eq!(gp.z(), 3.0);
}

#[test]
fn test_mirror_point_through_origin() {
    let gp = GeomPoint::new(4.0, 5.0, 6.0);
    let origin = Pnt { x: 0.0, y: 0.0, z: 0.0 };
    let result = gp.mirror_point(&origin);
    assert!(pnt_approx_eq(&result.pnt, -4.0, -5.0, -6.0, 1e-10));
}

#[test]
fn test_mirror_point_involution() {
    let gp = GeomPoint::new(2.0, -3.0, 7.0);
    let center = Pnt { x: 1.0, y: 1.0, z: 1.0 };
    let result = gp.mirror_point(&center).mirror_point(&center);
    assert!(pnt_approx_eq(&result.pnt, 2.0, -3.0, 7.0, 1e-10));
}

#[test]
fn test_mirror_ax1_y_axis() {
    // Mirror (2,3,4) through Y-axis: dot with Y = 3
    // r = (2,3,4), r' = 2*3*(0,1,0) - (2,3,4) = (0,6,0)-(2,3,4) = (-2,3,-4)
    let gp = GeomPoint::new(2.0, 3.0, 4.0);
    let y_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let result = gp.mirror_ax1(&y_ax);
    assert!(pnt_approx_eq(&result.pnt, -2.0, 3.0, -4.0, 1e-10));
}

#[test]
fn test_mirror_ax1_involution() {
    let gp = GeomPoint::new(5.0, 2.0, -1.0);
    let ax = make_ax1(2.0, 0.0, 0.0, 1.0, 1.0, 0.0);
    let result = gp.mirror_ax1(&ax).mirror_ax1(&ax);
    assert!(pnt_approx_eq(&result.pnt, 5.0, 2.0, -1.0, 1e-10));
}

#[test]
fn test_mirror_ax2_yz_plane() {
    // Mirror (3,5,7) through YZ plane (normal=X, at origin)
    // result: (-3,5,7)
    let gp = GeomPoint::new(3.0, 5.0, 7.0);
    let yz_plane = make_ax2(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let result = gp.mirror_ax2(&yz_plane);
    assert!(pnt_approx_eq(&result.pnt, -3.0, 5.0, 7.0, 1e-10));
}

#[test]
fn test_mirror_ax2_involution() {
    let gp = GeomPoint::new(1.0, 2.0, 3.0);
    let plane = make_ax2(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let result = gp.mirror_ax2(&plane).mirror_ax2(&plane);
    assert!(pnt_approx_eq(&result.pnt, 1.0, 2.0, 3.0, 1e-10));
}

#[test]
fn test_rotate_y_axis_90deg() {
    // Rotate (1,0,0) 90 degrees around Y-axis -> (0,0,-1)
    let gp = GeomPoint::new(1.0, 0.0, 0.0);
    let y_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let angle = std::f64::consts::PI / 2.0;
    let result = gp.rotate(&y_ax, angle);
    assert!(pnt_approx_eq(&result.pnt, 0.0, 0.0, -1.0, 1e-10));
}

#[test]
fn test_rotate_360_identity() {
    let gp = GeomPoint::new(7.0, -3.0, 2.5);
    let ax = make_ax1(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let result = gp.rotate(&ax, 2.0 * std::f64::consts::PI);
    assert!(pnt_approx_eq(&result.pnt, 7.0, -3.0, 2.5, 1e-10));
}

#[test]
fn test_rotate_negative_angle() {
    // Rotate (0,1,0) -90 degrees around Z-axis -> (1,0,0)
    let gp = GeomPoint::new(0.0, 1.0, 0.0);
    let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let result = gp.rotate(&z_ax, -std::f64::consts::PI / 2.0);
    assert!(pnt_approx_eq(&result.pnt, 1.0, 0.0, 0.0, 1e-10));
}

#[test]
fn test_scale_unit() {
    // Scale by 1 is identity
    let gp = GeomPoint::new(3.0, 4.0, 5.0);
    let center = Pnt { x: 0.0, y: 0.0, z: 0.0 };
    let result = gp.scale(&center, 1.0);
    assert!(pnt_approx_eq(&result.pnt, 3.0, 4.0, 5.0, 1e-10));
}

#[test]
fn test_scale_half() {
    // Scale (4,6,8) by 0.5 from (0,0,0) -> (2,3,4)
    let gp = GeomPoint::new(4.0, 6.0, 8.0);
    let origin = Pnt { x: 0.0, y: 0.0, z: 0.0 };
    let result = gp.scale(&origin, 0.5);
    assert!(pnt_approx_eq(&result.pnt, 2.0, 3.0, 4.0, 1e-10));
}

#[test]
fn test_scale_from_center() {
    // Scale (5,5,5) by 3 from (2,2,2):
    // result = (2,2,2) + 3*(3,3,3) = (2,2,2)+(9,9,9) = (11,11,11)
    let gp = GeomPoint::new(5.0, 5.0, 5.0);
    let center = Pnt { x: 2.0, y: 2.0, z: 2.0 };
    let result = gp.scale(&center, 3.0);
    assert!(pnt_approx_eq(&result.pnt, 11.0, 11.0, 11.0, 1e-10));
}

#[test]
fn test_translate_vec_negative() {
    let gp = GeomPoint::new(5.0, 5.0, 5.0);
    let v = Vec3 { x: -5.0, y: -5.0, z: -5.0 };
    let result = gp.translate_vec(&v);
    assert!(pnt_approx_eq(&result.pnt, 0.0, 0.0, 0.0, 1e-10));
}

#[test]
fn test_translate_pnt_vector() {
    // translate_pnt should shift by (to - from)
    let gp = GeomPoint::new(0.0, 0.0, 0.0);
    let from = Pnt { x: 1.0, y: 2.0, z: 3.0 };
    let to = Pnt { x: 4.0, y: 6.0, z: 8.0 };
    let result = gp.translate_pnt(&from, &to);
    // shift = (3,4,5)
    assert!(pnt_approx_eq(&result.pnt, 3.0, 4.0, 5.0, 1e-10));
}

#[test]
fn test_translate_pnt_consistency_with_vec() {
    let gp = GeomPoint::new(2.0, 3.0, 4.0);
    let from = Pnt { x: 1.0, y: 0.0, z: 0.0 };
    let to = Pnt { x: 4.0, y: 2.0, z: -1.0 };
    let v = Vec3 { x: to.x - from.x, y: to.y - from.y, z: to.z - from.z };
    let result_pnt = gp.translate_pnt(&from, &to);
    let result_vec = gp.translate_vec(&v);
    assert!(pnt_approx_eq(&result_pnt.pnt, result_vec.pnt.x, result_vec.pnt.y, result_vec.pnt.z, 1e-10));
}

#[test]
fn test_copy_is_independent() {
    let gp = GeomPoint::new(1.0, 2.0, 3.0);
    let _copied = gp.copy();
    // Original is unchanged
    assert_eq!(gp.x(), 1.0);
    assert_eq!(gp.y(), 2.0);
    assert_eq!(gp.z(), 3.0);
}

#[test]
fn test_geom_geometry_trait_dynamic_dispatch() {
    let gp: Box<dyn GeomGeometry> = Box::new(GeomPoint::new(10.0, 20.0, 30.0));
    let copied = gp.copy();
    // Both copies exist independently
    let _ = gp.copy();
    let _ = copied.copy();
}

#[test]
fn test_rotate_then_mirror() {
    // Rotate (1,0,0) 90-deg around Z -> (0,1,0), then mirror through XZ plane (normal Y) -> (0,-1,0)
    let gp = GeomPoint::new(1.0, 0.0, 0.0);
    let z_ax = make_ax1(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let xz_plane = make_ax2(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let result = gp.rotate(&z_ax, std::f64::consts::PI / 2.0).mirror_ax2(&xz_plane);
    assert!(pnt_approx_eq(&result.pnt, 0.0, -1.0, 0.0, 1e-10));
}

#[test]
fn test_scale_then_translate() {
    let gp = GeomPoint::new(1.0, 1.0, 1.0);
    let origin = Pnt { x: 0.0, y: 0.0, z: 0.0 };
    let v = Vec3 { x: 5.0, y: 0.0, z: 0.0 };
    // scale by 2 -> (2,2,2), translate by (5,0,0) -> (7,2,2)
    let result = gp.scale(&origin, 2.0).translate_vec(&v);
    assert!(pnt_approx_eq(&result.pnt, 7.0, 2.0, 2.0, 1e-10));
}

#[test]
fn test_mirror_point_precise() {
    // Mirror (0.1, 0.2, 0.3) through (0.5, 0.5, 0.5)
    // result: (0.9, 0.8, 0.7)
    let gp = GeomPoint::new(0.1, 0.2, 0.3);
    let center = Pnt { x: 0.5, y: 0.5, z: 0.5 };
    let result = gp.mirror_point(&center);
    assert!(pnt_approx_eq(&result.pnt, 0.9, 0.8, 0.7, 1e-10));
}

#[test]
fn test_rotate_arbitrary_axis() {
    // Rotate (1,0,0) by 120 degrees around (1,1,1)/sqrt(3) axis.
    // By symmetry this maps X->Y->Z->X, so result should be (0,1,0).
    let gp = GeomPoint::new(1.0, 0.0, 0.0);
    let ax = make_ax1(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let angle = 2.0 * std::f64::consts::PI / 3.0;
    let result = gp.rotate(&ax, angle);
    assert!(pnt_approx_eq(&result.pnt, 0.0, 1.0, 0.0, 1e-10));
}
