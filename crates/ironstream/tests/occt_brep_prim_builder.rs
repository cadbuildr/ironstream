extern crate ironstream;
use ironstream::brep_prim_builder::*;

#[test]
fn test_brep_prim_builder_basic() {
    // PrismBuilder
    let pb = PrismBuilder::new("profile", [0.0, 0.0, 1.0], 5.0);
    assert_eq!(pb.base, "profile");
    assert_eq!(pb.height, 5.0);
    let desc = pb.build();
    assert!(desc.contains("Prism(base=profile"));
    assert!((pb.volume() - 5.0).abs() < 1e-10);

    // RevolBuilder
    use std::f64::consts::PI;
    let rb = RevolBuilder::new("circle", [0.0, 0.0, 1.0], 2.0 * PI);
    let rdesc = rb.build();
    assert!(rdesc.contains("full"));

    // SweepBuilder
    let sb = SweepBuilder::new("section", "spine");
    let sdesc = sb.build();
    assert!(sdesc.contains("Sweep(base=section, spine=spine)"));

    // Primitives
    let b = make_box(1.0, 2.0, 3.0);
    assert!(b.contains("Box("));
    let c = make_cylinder(2.0, 5.0);
    assert!(c.contains("Cylinder("));
}
