extern crate ironstream;
use ironstream::brep_shell_maker::*;

#[test]
fn test_brep_shell_maker_basic() {
    let mut sew = Sewing::new(1e-6);
    sew.add("face_1");
    sew.add("face_2");
    sew.add("face_3");
    sew.perform();
    let shell = sew.sewed_shape();
    assert!(!shell.is_empty());
    assert!(sew.nb_free_edges() == 0);

    let sm = ShellMaker::from_surface("surface_1");
    assert!(sm.is_done());
    let s = sm.shell();
    assert!(!s.is_empty());

    let result = sew_shapes(&["face_a", "face_b"], 1e-7);
    assert!(!result.is_empty());
}
