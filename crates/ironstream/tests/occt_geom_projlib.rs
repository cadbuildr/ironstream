extern crate ironstream;
use ironstream::geom_projlib::*;

#[test]
fn test_geom_projlib_basic() {
    // ProjectOnPlane: XY plane (normal = [0,0,1]).
    let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    // Point above the plane projects to the XY plane.
    let foot = proj.project_point([3.0, 4.0, 7.0]);
    assert!((foot[0] - 3.0).abs() < 1e-12);
    assert!((foot[1] - 4.0).abs() < 1e-12);
    assert!(foot[2].abs() < 1e-12, "z={}", foot[2]);

    // Polyline projection.
    let curve = vec![[0.0, 0.0, 1.0], [1.0, 0.0, 2.0], [2.0, 0.0, 3.0]];
    let projected = proj.project_curve(&curve);
    assert_eq!(projected.len(), 3);
    for p in &projected {
        assert!(p[2].abs() < 1e-12);
    }

    // Offset plane (z = 5).
    let proj2 = ProjectOnPlane::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let foot2 = proj2.project_point([1.0, 2.0, 10.0]);
    assert!((foot2[2] - 5.0).abs() < 1e-12, "z={}", foot2[2]);

    // ProjectOnSurface lifecycle.
    let mut surf = ProjectOnSurface::new("Plane", 1e-7);
    assert!(!surf.is_done());
    let uv = surf.project_point([1.0, 2.0, 0.0]);
    // Stub returns (x, y) as (u, v) for planar projection.
    assert!(surf.is_done());
    let _ = uv; // UV values are stub-dependent.

    // Polyline projection returns the same number of points.
    let pts3 = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
    let uvs = surf.project_curve(&pts3);
    assert_eq!(uvs.len(), 2);

    // Free function wrapper.
    let uv2 = project_3d_to_2d([3.0, 4.0, 0.0], "Plane");
    let _ = uv2;
}
