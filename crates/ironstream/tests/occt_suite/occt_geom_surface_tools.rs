extern crate ironstream;
use ironstream::geom_surface_tools::*;

#[test]
fn test_geom_surface_tools_basic() {
    // surface_of_face
    let s = SurfaceTools::surface_of_face("face1");
    assert!(!s.is_empty());

    // uv_bounds
    let (u, v) = SurfaceTools::uv_bounds("my_surface");
    assert!(u[0] <= u[1]);
    assert!(v[0] <= v[1]);

    // sample_surface
    let grid = SurfaceTools::sample_surface("my_surface", 3, 3);
    assert_eq!(grid.len(), 3);
    assert_eq!(grid[0].len(), 3);

    // is_planar
    assert!(SurfaceTools::is_planar("Plane_XY", 1e-7));
    assert!(!SurfaceTools::is_planar("Cylinder", 1e-7));

    // continuity
    let c = SurfaceTools::continuity("my_surface");
    assert!(c <= 9);

    // surface_normal
    let n = surface_normal("my_surface", 0.0, 0.0);
    assert_eq!(n, [0.0, 0.0, 1.0]);

    // surface_d1
    let (pt, du, dv) = surface_d1("my_surface", 1.0, 2.0);
    assert_eq!(pt[0], 1.0);
    assert_eq!(pt[1], 2.0);
    assert_eq!(du, [1.0, 0.0, 0.0]);
    assert_eq!(dv, [0.0, 1.0, 0.0]);

    // project_on_surface
    let uv = project_on_surface([1.0, 2.0, 3.0], "my_surface");
    assert_eq!(uv[0], 1.0);
    assert_eq!(uv[1], 2.0);
}
