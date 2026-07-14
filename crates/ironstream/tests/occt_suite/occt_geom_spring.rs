extern crate ironstream;
use ironstream::geom_spring::*;

#[test]
fn test_geom_spring_basic() {
    let s = Spring::new(1.0, 2.0, 5.0);
    assert!((s.radius - 1.0).abs() < 1e-10);
    assert!((s.pitch - 2.0).abs() < 1e-10);
    assert!((s.turns - 5.0).abs() < 1e-10);

    // total_length should be positive
    assert!(s.total_length() > 0.0);

    // point_at t=0
    let p = s.point_at(0.0);
    assert!((p[0] - 1.0).abs() < 1e-10, "at t=0 x should equal radius");
    assert!(p[1].abs() < 1e-10);

    // tangent_at should be a unit vector
    let tan = s.tangent_at(0.0);
    let mag = (tan[0]*tan[0] + tan[1]*tan[1] + tan[2]*tan[2]).sqrt();
    assert!((mag - 1.0).abs() < 1e-9, "tangent should be unit length");
}

#[test]
fn test_helix_basic() {
    let h = Helix::new(1.0, 1.0);
    let p = h.at(0.0);
    assert!((p[0] - 1.0).abs() < 1e-10);
    assert!(p[1].abs() < 1e-10);
    assert!(p[2].abs() < 1e-10);

    // arc_length for one full turn
    let l = h.arc_length(0.0, std::f64::consts::TAU);
    assert!(l > 0.0);
}

#[test]
fn test_spring_points_free_function() {
    let pts = spring_points(2.0, 1.0, 3.0, 37);
    assert_eq!(pts.len(), 37);
    // First point should have radius ~2 from z-axis
    let p0 = &pts[0];
    let r = (p0[0]*p0[0] + p0[1]*p0[1]).sqrt();
    assert!((r - 2.0).abs() < 1e-9, "radius of first point should be ~2, got {}", r);
}

#[test]
fn test_spring_num_points() {
    let s = Spring::new(2.0, 1.0, 2.0);
    let pts = s.num_points(10);
    assert_eq!(pts.len(), 10);
}
