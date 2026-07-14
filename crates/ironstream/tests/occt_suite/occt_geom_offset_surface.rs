// FILE: tests/occt_geom_offset_surface.rs
extern crate ironstream;
use ironstream::geom_offset_surface::*;

fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

fn vec3_approx_eq(a: [f64; 3], b: [f64; 3], tol: f64) -> bool {
    approx_eq(a[0], b[0], tol) && approx_eq(a[1], b[1], tol) && approx_eq(a[2], b[2], tol)
}

fn norm3(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

// ============================================================
// Integration: offset a plane
// ============================================================

#[test]
fn integration_offset_plane_xy_by_positive_distance() {
    // XY plane offset by 10 along normal (+Z)
    let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let surf = GeomOffsetSurface::new_with_plane(plane, 10.0);

    // At any (u,v), z should be exactly 10
    for i in 0..5 {
        for j in 0..5 {
            let u = (i as f64 - 2.0) * 3.0;
            let v = (j as f64 - 2.0) * 3.0;
            let q = surf.value(u, v);
            assert!(
                approx_eq(q[2], 10.0, 1e-7),
                "z should be 10 at u={} v={}, got {}",
                u,
                v,
                q[2]
            );
            assert!(approx_eq(q[0], u, 1e-7));
            assert!(approx_eq(q[1], v, 1e-7));
        }
    }
}

#[test]
fn integration_offset_plane_z_negative() {
    let plane = GeomOffsetPlane::new([0.0, 0.0, 5.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let surf = GeomOffsetSurface::new_with_plane(plane, -3.0);
    // z = 5 + (-3) * 1 = 2
    let q = surf.value(1.0, 2.0);
    assert!(approx_eq(q[2], 2.0, 1e-7));
}

#[test]
fn integration_offset_plane_offset_value() {
    let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let surf = GeomOffsetSurface::new_with_plane(plane, 7.5);
    assert!(approx_eq(surf.offset_value(), 7.5, 1e-15));
}

#[test]
fn integration_offset_plane_d1_flat_derivatives() {
    // For a flat plane, the offset derivatives equal the basis derivatives (dN/du = 0)
    let plane = GeomOffsetPlane::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let surf = GeomOffsetSurface::new_with_plane(plane, 5.0);

    let (_, du, dv) = surf.d1(3.0, -2.0);
    assert!(
        vec3_approx_eq(du, [1.0, 0.0, 0.0], 1e-5),
        "du should be x-axis for XY plane, got {:?}",
        du
    );
    assert!(
        vec3_approx_eq(dv, [0.0, 1.0, 0.0], 1e-5),
        "dv should be y-axis for XY plane, got {:?}",
        dv
    );
}

// ============================================================
// Integration: offset a sphere (check radius)
// ============================================================

#[test]
fn integration_offset_sphere_increases_radius() {
    let r = 8.0_f64;
    let d = 3.0_f64;
    let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], r);
    let surf = GeomOffsetSurface::new_with_sphere(sph, d);
    let expected = r + d;

    // Sample points across the sphere
    let pi = std::f64::consts::PI;
    for i in 0..12 {
        for j in 0..6 {
            let u = i as f64 * pi / 6.0;
            let v = -pi / 2.0 + j as f64 * pi / 5.0;
            let q = surf.value(u, v);
            let dist = norm3(q);
            assert!(
                approx_eq(dist, expected, 1e-6),
                "Sphere offset radius wrong: expected {} got {} at u={:.3} v={:.3}",
                expected,
                dist,
                u,
                v
            );
        }
    }
}

#[test]
fn integration_offset_sphere_decreases_radius() {
    let r = 12.0_f64;
    let d = -4.0_f64;
    let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], r);
    let surf = GeomOffsetSurface::new_with_sphere(sph, d);
    let expected = r + d; // 8.0

    let pi = std::f64::consts::PI;
    let q = surf.value(pi / 4.0, pi / 6.0);
    let dist = norm3(q);
    assert!(approx_eq(dist, expected, 1e-6));
}

#[test]
fn integration_offset_sphere_off_center() {
    let center = [3.0, -1.0, 2.0];
    let r = 6.0_f64;
    let d = 2.0_f64;
    let sph = GeomOffsetSphere::new(center, r);
    let surf = GeomOffsetSurface::new_with_sphere(sph, d);
    let expected = r + d;

    let pi = std::f64::consts::PI;
    for i in 0..8 {
        let u = i as f64 * pi / 4.0;
        let q = surf.value(u, 0.3);
        let dist = norm3(sub3(q, center));
        assert!(
            approx_eq(dist, expected, 1e-6),
            "Off-center sphere offset: expected {} got {} at u={:.3}",
            expected,
            dist,
            u
        );
    }
}

#[test]
fn integration_offset_sphere_normal_outward() {
    let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 5.0);
    let surf = GeomOffsetSurface::new_with_sphere(sph, 2.0);

    // At u=0, v=0, the normal of the offset sphere should still point in +X
    let n = surf.normal(0.0, 0.0);
    assert!(approx_eq(n[0], 1.0, 1e-5), "Expected normal +X, got {:?}", n);
    assert!(approx_eq(n[1], 0.0, 1e-5));
    assert!(approx_eq(n[2], 0.0, 1e-5));
}

// ============================================================
// Integration: offset a cylinder
// ============================================================

#[test]
fn integration_offset_cylinder_radius() {
    let r = 5.0_f64;
    let d = 2.5_f64;
    let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
    let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);
    let expected = r + d;

    let pi = std::f64::consts::PI;
    for i in 0..12 {
        let angle = i as f64 * pi / 6.0;
        for j in 0..5 {
            let height = (j as f64 - 2.0) * 4.0;
            let q = surf.value(angle, height);
            let xy_dist = (q[0] * q[0] + q[1] * q[1]).sqrt();
            assert!(
                approx_eq(xy_dist, expected, 1e-7),
                "Cylinder offset radius wrong at angle={:.3} h={}: expected {} got {}",
                angle,
                height,
                expected,
                xy_dist
            );
            // Height should be preserved
            assert!(
                approx_eq(q[2], height, 1e-7),
                "Cylinder height not preserved: expected {} got {}",
                height,
                q[2]
            );
        }
    }
}

#[test]
fn integration_offset_cylinder_inward() {
    let r = 8.0_f64;
    let d = -2.0_f64;
    let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
    let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);
    let expected = r + d; // 6.0

    let pi = std::f64::consts::PI;
    let q = surf.value(pi / 3.0, 5.0);
    let xy_dist = (q[0] * q[0] + q[1] * q[1]).sqrt();
    assert!(approx_eq(xy_dist, expected, 1e-7));
}

#[test]
fn integration_offset_cylinder_shifted_axis() {
    // Cylinder centered at (2, 3, 0) along Z
    let r = 4.0_f64;
    let d = 1.0_f64;
    let cyl = GeomOffsetCylinder::new([2.0, 3.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], r);
    let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);
    let expected = r + d; // 5.0
    let center_xy = [2.0, 3.0];

    let pi = std::f64::consts::PI;
    for i in 0..8 {
        let angle = i as f64 * pi / 4.0;
        let q = surf.value(angle, 0.0);
        let dx = q[0] - center_xy[0];
        let dy = q[1] - center_xy[1];
        let dist = (dx * dx + dy * dy).sqrt();
        assert!(
            approx_eq(dist, expected, 1e-7),
            "Shifted cylinder offset: expected {} got {} at angle={:.3}",
            expected,
            dist,
            angle
        );
    }
}

#[test]
fn integration_offset_cylinder_d1_matches_finite_differences() {
    let r = 3.0_f64;
    let d = 0.5_f64;
    let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], r);
    let surf = GeomOffsetSurface::new_with_cylinder(cyl, d);

    let u = 1.1_f64;
    let v = 2.3_f64;
    let (_, du, dv) = surf.d1(u, v);
    let eps = 1e-5_f64;

    // Finite difference du
    let qpu = surf.value(u + eps, v);
    let qmu = surf.value(u - eps, v);
    let fd_du = [
        (qpu[0] - qmu[0]) / (2.0 * eps),
        (qpu[1] - qmu[1]) / (2.0 * eps),
        (qpu[2] - qmu[2]) / (2.0 * eps),
    ];
    assert!(
        vec3_approx_eq(du, fd_du, 1e-4),
        "Cylinder d1 du mismatch: {:?} vs fd {:?}",
        du,
        fd_du
    );

    // Finite difference dv
    let qpv = surf.value(u, v + eps);
    let qmv = surf.value(u, v - eps);
    let fd_dv = [
        (qpv[0] - qmv[0]) / (2.0 * eps),
        (qpv[1] - qmv[1]) / (2.0 * eps),
        (qpv[2] - qmv[2]) / (2.0 * eps),
    ];
    assert!(
        vec3_approx_eq(dv, fd_dv, 1e-4),
        "Cylinder d1 dv mismatch: {:?} vs fd {:?}",
        dv,
        fd_dv
    );
}

// ============================================================
// Integration: GeomSurfaceEval trait usage via GeomBasisSurface
// ============================================================

#[test]
fn integration_basis_surface_enum_dispatch_plane() {
    let plane = GeomOffsetPlane::new([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let basis = GeomBasisSurface::Plane(plane);
    let p = basis.value(2.0, 3.0);
    assert!(vec3_approx_eq(p, [2.0, 3.0, 1.0], 1e-10));
    let n = basis.normal(0.0, 0.0);
    assert!(vec3_approx_eq(n, [0.0, 0.0, 1.0], 1e-10));
}

#[test]
fn integration_basis_surface_enum_dispatch_cylinder() {
    let cyl = GeomOffsetCylinder::along_z([0.0, 0.0, 0.0], 2.0);
    let basis = GeomBasisSurface::Cylinder(cyl);
    let p = basis.value(0.0, 0.0);
    assert!(vec3_approx_eq(p, [2.0, 0.0, 0.0], 1e-10));
    let n = basis.normal(0.0, 0.0);
    assert!(vec3_approx_eq(n, [1.0, 0.0, 0.0], 1e-10));
}

#[test]
fn integration_basis_surface_enum_dispatch_sphere() {
    let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 3.0);
    let basis = GeomBasisSurface::Sphere(sph);
    let p = basis.value(0.0, 0.0);
    assert!(vec3_approx_eq(p, [3.0, 0.0, 0.0], 1e-10));
    let n = basis.normal(0.0, 0.0);
    assert!(vec3_approx_eq(n, [1.0, 0.0, 0.0], 1e-10));
}

// ============================================================
// Integration: compute_normal helper
// ============================================================

#[test]
fn integration_compute_normal_xz_plane() {
    // du = (1,0,0), dv = (0,0,1) -> normal = (0,-1,0)
    let du = [1.0, 0.0, 0.0];
    let dv = [0.0, 0.0, 1.0];
    let n = compute_normal(du, dv);
    assert!(vec3_approx_eq(n, [0.0, -1.0, 0.0], 1e-10));
}

#[test]
fn integration_compute_normal_is_unit_length() {
    let du = [3.0, 0.0, 0.0];
    let dv = [0.0, 2.0, 0.0];
    let n = compute_normal(du, dv);
    let mag = norm3(n);
    assert!(approx_eq(mag, 1.0, 1e-10));
}

// ============================================================
// Integration: basis_surface_offset accessor
// ============================================================

#[test]
fn integration_basis_surface_offset_evaluates_correctly() {
    let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 5.0);
    let surf = GeomOffsetSurface::new_with_sphere(sph, 3.0);

    // The basis surface should evaluate to the original sphere (radius 5)
    let basis = surf.basis_surface_offset();
    let p = basis.value(0.0, 0.0); // should be (5, 0, 0)
    assert!(vec3_approx_eq(p, [5.0, 0.0, 0.0], 1e-10));

    // The offset surface value at (0,0) should be (8, 0, 0)
    let q = surf.value(0.0, 0.0);
    assert!(vec3_approx_eq(q, [8.0, 0.0, 0.0], 1e-7));
}

// ============================================================
// Integration: zero offset
// ============================================================

#[test]
fn integration_zero_offset_equals_basis_plane() {
    let plane = GeomOffsetPlane::new([1.0, 2.0, 3.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let surf = GeomOffsetSurface::new_with_plane(plane.clone(), 0.0);
    // With zero offset, points should equal basis surface
    let q = surf.value(4.0, 5.0);
    let p = plane.value(4.0, 5.0);
    assert!(vec3_approx_eq(q, p, 1e-7));
}

#[test]
fn integration_zero_offset_equals_basis_sphere() {
    let sph = GeomOffsetSphere::new([0.0, 0.0, 0.0], 7.0);
    let surf = GeomOffsetSurface::new_with_sphere(sph, 0.0);

    // With zero offset, radius should still be 7
    let q = surf.value(0.5, 0.5);
    let dist = norm3(q);
    assert!(approx_eq(dist, 7.0, 1e-7));
}
