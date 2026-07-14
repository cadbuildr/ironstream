extern crate ironstream;
use ironstream::geom_surface_of_revolution::*;
use std::f64::consts::PI;

#[test]
fn test_geom_surface_of_revolution_basic() {
    let mut s = SurfaceOfRevolution::new([0.0, 0.0, 1.0], [0.0, 0.0, 0.0]);
    s.add_profile_point([1.0, 0.0, 0.0]);
    s.add_profile_point([1.0, 0.0, 1.0]);

    // At u=0, v=0 -> profile[0] rotated by 0 = [1,0,0]
    let p = s.evaluate(0.0, 0.0);
    assert!((p[0] - 1.0).abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);

    // Surface is closed in U
    assert!(s.is_u_closed());

    // At u=PI the x-coordinate should be approximately -1
    let p2 = s.evaluate(PI, 0.0);
    assert!((p2[0] + 1.0).abs() < 1e-10, "x should be -1 at u=PI, got {}", p2[0]);

    // revolve_profile
    let pts = vec![[1.0, 0.0, 0.0], [1.0, 0.0, 1.0]];
    let grid = revolve_profile(&pts, [0.0, 0.0, 1.0], 2.0 * PI, 4);
    assert_eq!(grid.len(), 5); // n+1 steps
    assert_eq!(grid[0].len(), 2);
}
