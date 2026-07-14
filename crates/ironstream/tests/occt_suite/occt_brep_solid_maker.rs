extern crate ironstream;
use ironstream::brep_solid_maker::*;

#[test]
fn test_brep_solid_maker_basic() {
    let mut sm = SolidMaker::new();
    sm.add_shell("shell_1");
    let solid = sm.solid();
    assert!(!solid.is_empty());
    assert!(sm.is_done());

    let vol = sm.volume();
    assert!(vol >= 0.0);

    let bm = BoxMaker::new(2.0, 3.0, 4.0);
    let label = bm.build();
    assert!(!label.is_empty());
    assert!((bm.volume() - 24.0).abs() < 1e-10);
    assert!((bm.surface_area() - 52.0).abs() < 1e-10);

    let bm2 = BoxMaker::new(1.0, 1.0, 1.0).at([5.0, 5.0, 5.0]);
    let label2 = bm2.build();
    assert!(!label2.is_empty());

    let box_label = make_box(1.0, 2.0, 3.0);
    assert!(!box_label.is_empty());

    let sphere_label = make_sphere([0.0, 0.0, 0.0], 1.0);
    assert!(!sphere_label.is_empty());
}
