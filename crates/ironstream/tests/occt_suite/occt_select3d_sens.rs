extern crate ironstream;
use ironstream::select3d_sens::*;

#[test]
fn test_select3d_sens_basic() {
    // SensitiveBox: ray hitting the unit cube.
    let b = SensitiveBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], 1);
    assert_eq!(b.owner_id, 1);
    assert!(b.matches_ray([-1.0, 0.5, 0.5], [1.0, 0.0, 0.0]));
    // Ray completely missing the box.
    assert!(!b.matches_ray([-1.0, 2.0, 0.5], [1.0, 0.0, 0.0]));
    // Sensitivity equals diagonal length ≈ sqrt(3).
    assert!((b.sensitivity() - 3_f64.sqrt()).abs() < 1e-10);

    // SensitiveSphere.
    let s = SensitiveSphere::new([0.0, 0.0, 0.0], 1.0, 2);
    assert_eq!(s.owner_id, 2);
    assert!(s.matches_ray([-2.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    assert!(!s.matches_ray([-2.0, 2.0, 0.0], [1.0, 0.0, 0.0]));

    // SensitiveFace: a unit square in the XY plane.
    let mut f = SensitiveFace::new(3);
    assert_eq!(f.owner_id, 3);
    f.add_point([0.0, 0.0, 0.0]);
    f.add_point([1.0, 0.0, 0.0]);
    f.add_point([1.0, 1.0, 0.0]);
    f.add_point([0.0, 1.0, 0.0]);
    // Ray from above hitting the square.
    assert!(f.matches_ray([0.5, 0.5, 1.0], [0.0, 0.0, -1.0]));
    // Ray missing the square.
    assert!(!f.matches_ray([2.0, 0.5, 1.0], [0.0, 0.0, -1.0]));
    // Degenerate face (fewer than 3 points) always misses.
    let mut f2 = SensitiveFace::new(0);
    f2.add_point([0.0, 0.0, 0.0]);
    f2.add_point([1.0, 0.0, 0.0]);
    assert!(!f2.matches_ray([0.5, 0.0, 1.0], [0.0, 0.0, -1.0]));
}
