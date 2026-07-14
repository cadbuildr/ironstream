extern crate ironstream;
use ironstream::geom_vector2d::*;

#[test]
fn test_geom_vector2d_basic() {
    let v = Vec2d::new(3.0, 4.0);
    assert!((v.magnitude() - 5.0).abs() < 1e-10);

    let z = Vec2d::zero();
    assert_eq!(z.x, 0.0);
    assert_eq!(z.y, 0.0);

    let a = Vec2d::new(1.0, 0.0);
    let b = Vec2d::new(0.0, 1.0);
    assert!((a.dot(&b) - 0.0).abs() < 1e-10);
    assert!((a.crossed(&b) - 1.0).abs() < 1e-10);

    let n = v.normalize();
    assert!((n.magnitude() - 1.0).abs() < 1e-10);

    let d = Dir2d::new(1.0, 0.0).expect("unit dir");
    let d2 = Dir2d::new(0.0, 1.0).expect("unit dir");
    let ang = d.angle(&d2);
    assert!((ang - std::f64::consts::FRAC_PI_2).abs() < 1e-10);

    // None for zero input
    assert!(Dir2d::new(0.0, 0.0).is_none());

    let scaled = v.scale(2.0);
    assert!((scaled.x - 6.0).abs() < 1e-10);
    assert!((scaled.y - 8.0).abs() < 1e-10);
}
