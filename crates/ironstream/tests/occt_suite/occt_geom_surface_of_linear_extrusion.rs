extern crate ironstream;
use ironstream::geom_surface_of_linear_extrusion::*;

#[test]
fn test_geom_surface_of_linear_extrusion_basic() {
    let mut s = SurfaceOfLinearExtrusion::new([0.0, 0.0, 1.0]);
    s.add_profile_point([0.0, 0.0, 0.0]);
    s.add_profile_point([1.0, 0.0, 0.0]);
    s.add_profile_point([2.0, 0.0, 0.0]);

    // S(0, 0) = profile[0] + 0 * dir = [0,0,0]
    let p = s.evaluate(0.0, 0.0);
    assert!(p[0].abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
    assert!(p[2].abs() < 1e-10);

    // S(0, 3) = [0,0,0] + 3*[0,0,1] = [0,0,3]
    let p2 = s.evaluate(0.0, 3.0);
    assert!((p2[2] - 3.0).abs() < 1e-10, "z should be 3, got {}", p2[2]);

    // d1v should be unit direction
    let dv = s.d1v(0.0, 0.0);
    assert!((dv[2] - 1.0).abs() < 1e-10);

    // is_u_closed should be false for open polyline
    assert!(!s.is_u_closed());

    // extrude_curve
    let result = extrude_curve(&[[0.0,0.0,0.0],[1.0,0.0,0.0]], [0.0,0.0,1.0], 5.0);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].len(), 2);
}
