// Integration tests for shape_fix_ext module
extern crate ironstream;

use ironstream::shape_fix_ext::*;

#[test]
fn fix_status_predicates() {
    assert!(FixStatus::Ok.is_done());
    assert!(FixStatus::Done1.is_done());
    assert!(FixStatus::Done2.is_done());
    assert!(FixStatus::Done5.is_done());
    assert!(!FixStatus::NotDone.is_done());
    assert!(!FixStatus::Fail1.is_done());

    assert!(FixStatus::Fail1.is_fail());
    assert!(FixStatus::Fail5.is_fail());
    assert!(!FixStatus::Done1.is_fail());
    assert!(!FixStatus::Ok.is_fail());
    assert!(!FixStatus::NotDone.is_fail());

    let s = FixStatus::default();
    assert_eq!(s, FixStatus::NotDone);
}

#[test]
fn fix_report_messages_and_counters() {
    let mut r = FixReport::new();
    assert!(r.is_empty());
    assert_eq!(r.nb_messages(), 0);

    r.nb_faces_fixed = 3;
    r.nb_edges_fixed = 1;
    r.add_message("Fixed degenerate edge");
    r.add_message("Reoriented face");
    assert!(!r.is_empty());
    assert_eq!(r.nb_messages(), 2);
    assert_eq!(r.nb_faces_fixed, 3);
}

#[test]
fn shape_fix_shape_perform() {
    let mut fs = ShapeFixShape::new(1);
    fs.set_precision(1e-6);
    fs.set_max_tolerance(0.1);
    fs.set_min_tolerance(1e-8);
    fs.set_fix_solid_mode(1);
    fs.set_fix_shell_mode(0);

    assert!((fs.tolerance - 1e-6).abs() < 1e-15);
    assert!((fs.max_tolerance - 0.1).abs() < 1e-9);

    assert!(fs.perform());
    assert!(fs.is_done());
    assert!(fs.shape() > 0);
    assert_eq!(fs.shape(), 2); // solid_id + 1
    let report = fs.report();
    assert_eq!(report.nb_messages(), 0);
}

#[test]
fn shape_fix_shape_fail_on_null_shape() {
    let mut fs = ShapeFixShape::new(0);
    assert!(!fs.perform());
    assert!(!fs.is_done());
    assert!(fs.status.is_fail());
    assert_eq!(fs.shape(), 0);
}

#[test]
fn shape_fix_shell_and_solid() {
    // shell
    let mut shell = ShapeFixShell::new(5);
    shell.set_fix_orientation_mode(false);
    assert!(!shell.fix_orientation);
    assert!(shell.perform());
    assert!(shell.is_done());
    assert_eq!(shell.shell(), 7); // shell_id + 2
    assert_eq!(shell.nb_faces(), 0);

    // shell null id
    let mut bad_shell = ShapeFixShell::new(0);
    assert!(!bad_shell.perform());
    assert!(bad_shell.status.is_fail());

    // solid
    let mut solid = ShapeFixSolid::new(10);
    solid.set_fix_shell_orientation_mode(false);
    assert!(!solid.fix_shell_orientation);
    assert!(solid.perform());
    assert!(solid.is_done());
    assert_eq!(solid.solid(), 13); // solid_id + 3
    assert_eq!(solid.nb_shells_fixed(), 0);

    // solid null id
    let mut bad_solid = ShapeFixSolid::new(0);
    assert!(!bad_solid.perform());
    assert!(bad_solid.status.is_fail());
}

#[test]
fn shape_fix_split_face() {
    // valid face
    let mut sf = ShapeFixSplitFace::new(7);
    assert_eq!(sf.face_id, 7);
    sf.split();
    assert!(sf.is_done());
    assert_eq!(sf.nb_result_faces(), 1);
    assert_eq!(sf.result_face(0), Some(7));
    assert!(sf.result_face(1).is_none());

    // null face_id
    let mut bad_sf = ShapeFixSplitFace::new(0);
    bad_sf.split();
    assert!(!bad_sf.is_done());
    assert_eq!(bad_sf.status, FixStatus::Fail1);
    assert_eq!(bad_sf.nb_result_faces(), 0);
}
