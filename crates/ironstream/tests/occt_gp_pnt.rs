extern crate ironstream;
use ironstream::gp_pnt::*;

#[test]
fn test_gp_pnt_basic() {
    let p = Pnt::new(3.0, 4.0, 0.0);
    assert_eq!(p.x, 3.0);
    assert_eq!(p.y, 4.0);
    assert_eq!(p.z, 0.0);

    let o = Pnt::origin();
    assert_eq!(o.x, 0.0);

    let dist = p.distance(&o);
    assert!((dist - 5.0).abs() < 1e-10);

    let sq = p.square_distance(&o);
    assert!((sq - 25.0).abs() < 1e-10);

    assert!(p.is_equal(&Pnt::new(3.0, 4.0, 0.0), 1e-10));

    let mut p2 = Pnt::new(1.0, 2.0, 3.0);
    p2.translate([1.0, 1.0, 1.0]);
    assert!((p2.x - 2.0).abs() < 1e-10);

    let mid = midpoint(&Pnt::new(0.0, 0.0, 0.0), &Pnt::new(4.0, 0.0, 0.0));
    assert!((mid.x - 2.0).abs() < 1e-10);

    let bc = Pnt::barycenter(&Pnt::new(0.0, 0.0, 0.0), 1.0, &Pnt::new(4.0, 0.0, 0.0), 1.0);
    assert!((bc.x - 2.0).abs() < 1e-10);

    let (d01, d12, d02) = distance_3pts(
        &Pnt::new(0.0, 0.0, 0.0),
        &Pnt::new(1.0, 0.0, 0.0),
        &Pnt::new(2.0, 0.0, 0.0),
    );
    assert!((d01 - 1.0).abs() < 1e-10);
    assert!((d12 - 1.0).abs() < 1e-10);
    assert!((d02 - 2.0).abs() < 1e-10);
}
