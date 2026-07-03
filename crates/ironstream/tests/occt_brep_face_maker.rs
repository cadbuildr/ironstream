extern crate ironstream;
use ironstream::brep_face_maker::*;

#[test]
fn test_brep_face_maker_basic() {
    let fm = FaceMaker::from_wire("wire_1");
    assert!(fm.is_done());
    assert!(!fm.face().is_empty());
    assert!(fm.error().is_none());

    let fm2 = FaceMaker::from_plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    assert!(fm2.is_done());
    assert!(!fm2.face().is_empty());

    let fm3 = FaceMaker::from_cylinder(1.0, 5.0);
    assert!(fm3.is_done());

    let mut fm4 = FaceMaker::from_wire("wire_base");
    fm4.add_wire("wire_hole");
    assert!(fm4.is_done());

    let face_label = make_planar_face("wire_outer");
    assert!(!face_label.is_empty());

    let face_label2 = make_face_from_plane(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        0.0, 1.0, 0.0, 1.0,
    );
    assert!(!face_label2.is_empty());
}
