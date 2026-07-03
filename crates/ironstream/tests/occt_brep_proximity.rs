extern crate ironstream;
use ironstream::brep_proximity::*;

#[test]
fn test_brep_proximity_basic() {
    // DistShapeShape
    let mut dss = DistShapeShape::new("S1", "S2");
    assert!(!dss.is_done());
    assert_eq!(dss.nb_solution(), 0);
    dss.perform();
    assert!(dss.is_done());
    assert_eq!(dss.nb_solution(), 1);
    let v = dss.value();
    assert!(v >= 0.0);
    let p1 = dss.point_on_shape1(0);
    let p2 = dss.point_on_shape2(0);
    assert_eq!(p1.len(), 3);
    assert_eq!(p2.len(), 3);

    // ShapeProximity
    let mut sp = ShapeProximity::new("A", "B", 0.01);
    assert_eq!(sp.tolerance, 0.01);
    sp.perform();
    let o1 = sp.overlaps_1();
    let o2 = sp.overlaps_2();
    // stub returns empty overlap sets
    assert_eq!(o1.len(), 0);
    assert_eq!(o2.len(), 0);
}
