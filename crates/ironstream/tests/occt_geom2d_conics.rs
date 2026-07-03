extern crate ironstream;
use ironstream::geom2d_conics::*;
use std::f64::consts::PI;

#[test]
fn test_geom2d_conics_basic() {
    // Circle2dGeom
    let c = Circle2dGeom::new(0.0, 0.0, 5.0);
    let p = c.point_at(0.0);
    assert!((p[0] - 5.0).abs() < 1e-12);
    assert!((p[1]).abs() < 1e-12);
    assert!((c.period() - 2.0 * PI).abs() < 1e-12);

    // Ellipse2dGeom
    let e = Ellipse2dGeom::new(0.0, 0.0, 5.0, 3.0);
    let ep = e.point_at(0.0);
    assert!((ep[0] - 5.0).abs() < 1e-12);
    let f1 = e.focus1();
    let f2 = e.focus2();
    // c = sqrt(25-9) = 4
    assert!((f1[0] - 4.0).abs() < 1e-12);
    assert!((f2[0] + 4.0).abs() < 1e-12);

    // Hyperbola2d
    let h = Hyperbola2d::new(0.0, 0.0, 3.0, 4.0);
    let hp = h.point_at(0.0);
    assert!((hp[0] - 3.0).abs() < 1e-12);

    // Parabola2dGeom
    let par = Parabola2dGeom::new(2.0, 0.0, 2.0);
    let pp = par.point_at(0.0);
    assert!(pp[0].abs() < 1e-12); // vertex at origin
}
